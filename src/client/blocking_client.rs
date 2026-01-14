use std::{collections::BTreeMap, sync::Arc, time::Duration};

use http::{HeaderMap, HeaderValue, Method, header};
use serde::de::DeserializeOwned;

use crate::{
    api::{BlockingBillingService, BlockingEcsService, BlockingStsService},
    auth::Auth,
    error::{Error, ErrorInfo},
    transport::{
        BlockingTransport, Request, Response,
        blocking_transport::UreqTransport,
        retry::{RetryPolicy, backoff_delay, parse_retry_after, should_retry_status},
    },
    util::{rpc, url as url_util},
};

use super::common::{
    AliyunEnvelope, classify_aliyun_error, classify_http_error, extract_request_id,
    maybe_body_snippet,
};

#[derive(Clone)]
pub struct BlockingClient {
    inner: Arc<Inner>,
}

struct Inner {
    auth: Auth,
    endpoints: Endpoints,
    defaults: RequestDefaults,
    retry: RetryPolicy,
    transport: Arc<dyn BlockingTransport>,
}

#[derive(Debug, Clone)]
struct Endpoints {
    ecs: url::Url,
    sts: url::Url,
    billing: url::Url,
}

#[derive(Debug, Clone)]
struct RequestDefaults {
    timeout: Duration,
    connect_timeout: Duration,
    default_headers: HeaderMap,
    capture_body_snippet: bool,
    body_snippet_max_len: usize,
}

pub struct BlockingClientBuilder {
    auth: Auth,
    ecs_endpoint: String,
    sts_endpoint: String,
    billing_endpoint: String,
    defaults: RequestDefaults,
    retry: RetryPolicy,
    #[cfg(test)]
    transport_override: Option<Arc<dyn BlockingTransport>>,
}

impl BlockingClient {
    pub fn builder() -> BlockingClientBuilder {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            header::USER_AGENT,
            HeaderValue::from_static(concat!("alibabacloud/", env!("CARGO_PKG_VERSION"))),
        );

        BlockingClientBuilder {
            auth: Auth::none(),
            ecs_endpoint: "https://ecs.aliyuncs.com/".to_owned(),
            sts_endpoint: "https://sts.aliyuncs.com/".to_owned(),
            billing_endpoint: "https://business.aliyuncs.com/".to_owned(),
            defaults: RequestDefaults {
                timeout: Duration::from_secs(30),
                connect_timeout: Duration::from_secs(10),
                default_headers,
                capture_body_snippet: true,
                body_snippet_max_len: 4096,
            },
            retry: RetryPolicy::default(),
            #[cfg(test)]
            transport_override: None,
        }
    }

    pub fn ecs(&self) -> BlockingEcsService {
        BlockingEcsService::new(self.clone())
    }

    pub fn sts(&self) -> BlockingStsService {
        BlockingStsService::new(self.clone())
    }

    pub fn billing(&self) -> BlockingBillingService {
        BlockingBillingService::new(self.clone())
    }

    pub(crate) fn rpc_json<T: DeserializeOwned>(
        &self,
        base_url: &url::Url,
        action: &'static str,
        version: &'static str,
        mut params: BTreeMap<String, String>,
    ) -> Result<T, Error> {
        params.insert("Action".to_owned(), action.to_owned());
        params.insert("Version".to_owned(), version.to_owned());
        params
            .entry("Format".to_owned())
            .or_insert("JSON".to_owned());

        let Some(access_key) = self.inner.auth.as_access_key() else {
            return Err(Error::invalid_config(
                "access key authentication is required",
                None,
            ));
        };

        rpc::inject_common_rpc_params(&mut params, access_key)?;

        let canonical_query = rpc::canonical_query(&params);
        let signature = rpc::signature(
            &Method::GET,
            &canonical_query,
            access_key.access_key_secret.expose(),
        )?;
        params.insert("Signature".to_owned(), signature);

        let mut url = url_util::endpoint(base_url, &[])?;
        url.set_query(Some(&rpc::canonical_query(&params)));

        self.send_json(Method::GET, url)
    }

    pub(crate) fn endpoint_ecs(&self) -> &url::Url {
        &self.inner.endpoints.ecs
    }

    pub(crate) fn endpoint_sts(&self) -> &url::Url {
        &self.inner.endpoints.sts
    }

    pub(crate) fn endpoint_billing(&self) -> &url::Url {
        &self.inner.endpoints.billing
    }

    fn send_json<T: DeserializeOwned>(&self, method: Method, url: url::Url) -> Result<T, Error> {
        let path = url.path().to_owned();
        let headers = self.inner.defaults.default_headers.clone();

        let request = Request {
            method: method.clone(),
            url,
            headers,
            timeout: self.inner.defaults.timeout,
        };

        let response = self.send_with_retries(&request)?;
        let request_id = extract_request_id(&response.headers);

        if !response.status.is_success() {
            return Err(classify_http_error(
                method,
                path,
                response,
                request_id,
                self.inner.defaults.capture_body_snippet,
                self.inner.defaults.body_snippet_max_len,
            ));
        }

        let mut deserializer = serde_json::Deserializer::from_slice(&response.body);
        let parsed = serde_path_to_error::deserialize::<_, AliyunEnvelope<T>>(&mut deserializer);
        match parsed {
            Ok(AliyunEnvelope::Ok(value)) => Ok(value),
            Ok(AliyunEnvelope::Err(err)) => {
                let body_snippet = maybe_body_snippet(
                    self.inner.defaults.capture_body_snippet,
                    &response.body,
                    self.inner.defaults.body_snippet_max_len,
                );
                Err(classify_aliyun_error(
                    method,
                    path,
                    response.status,
                    request_id,
                    err,
                    body_snippet,
                ))
            }
            Err(source) => Err(Error::Decode {
                info: Box::new(ErrorInfo {
                    status: Some(response.status),
                    method: Some(method),
                    path: Some(path),
                    request_id,
                    body_snippet: maybe_body_snippet(
                        self.inner.defaults.capture_body_snippet,
                        &response.body,
                        self.inner.defaults.body_snippet_max_len,
                    ),
                    message: None,
                }),
                source: Box::new(source),
            }),
        }
    }

    fn send_with_retries(&self, request: &Request) -> Result<Response, Error> {
        let mut attempt = 0usize;
        loop {
            let result = self.inner.transport.send(request.clone());
            match result {
                Ok(response) => {
                    if attempt >= self.inner.retry.max_retries
                        || !should_retry_status(response.status)
                    {
                        return Ok(response);
                    }

                    let delay = parse_retry_after(&response.headers)
                        .unwrap_or_else(|| backoff_delay(&self.inner.retry, attempt));
                    std::thread::sleep(delay);
                    attempt += 1;
                    continue;
                }
                Err(source) => {
                    if attempt < self.inner.retry.max_retries {
                        let delay = backoff_delay(&self.inner.retry, attempt);
                        std::thread::sleep(delay);
                        attempt += 1;
                        continue;
                    }

                    return Err(Error::Transport {
                        info: Box::new(ErrorInfo {
                            status: None,
                            method: Some(request.method.clone()),
                            path: Some(request.url.path().to_owned()),
                            message: None,
                            request_id: None,
                            body_snippet: None,
                        }),
                        source,
                    });
                }
            }
        }
    }
}

impl BlockingClientBuilder {
    pub fn auth(mut self, auth: Auth) -> Self {
        self.auth = auth;
        self
    }

    pub fn ecs_endpoint(mut self, endpoint: impl AsRef<str>) -> Self {
        self.ecs_endpoint = endpoint.as_ref().to_owned();
        self
    }

    pub fn sts_endpoint(mut self, endpoint: impl AsRef<str>) -> Self {
        self.sts_endpoint = endpoint.as_ref().to_owned();
        self
    }

    pub fn billing_endpoint(mut self, endpoint: impl AsRef<str>) -> Self {
        self.billing_endpoint = endpoint.as_ref().to_owned();
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.defaults.timeout = timeout;
        self
    }

    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.defaults.connect_timeout = timeout;
        self
    }

    pub fn capture_body_snippet(mut self, enabled: bool) -> Self {
        self.defaults.capture_body_snippet = enabled;
        self
    }

    pub fn body_snippet_max_len(mut self, max_len: usize) -> Self {
        self.defaults.body_snippet_max_len = max_len;
        self
    }

    pub fn max_retries(mut self, max_retries: usize) -> Self {
        self.retry.max_retries = max_retries;
        self
    }

    pub fn retry_base_delay(mut self, base_delay: Duration) -> Self {
        self.retry.base_delay = base_delay;
        self
    }

    pub fn retry_max_delay(mut self, max_delay: Duration) -> Self {
        self.retry.max_delay = max_delay;
        self
    }

    pub fn default_header(mut self, name: header::HeaderName, value: HeaderValue) -> Self {
        self.defaults.default_headers.insert(name, value);
        self
    }

    pub fn default_headers(mut self, headers: HeaderMap) -> Self {
        self.defaults.default_headers = headers;
        self
    }

    #[cfg(test)]
    pub(crate) fn transport_override(mut self, transport: Arc<dyn BlockingTransport>) -> Self {
        self.transport_override = Some(transport);
        self
    }

    pub fn build(self) -> Result<BlockingClient, Error> {
        let ecs = url_util::parse_base_url(&self.ecs_endpoint)?;
        let sts = url_util::parse_base_url(&self.sts_endpoint)?;
        let billing = url_util::parse_base_url(&self.billing_endpoint)?;

        let transport: Arc<dyn BlockingTransport> = {
            #[cfg(test)]
            if let Some(transport) = self.transport_override {
                transport
            } else {
                Arc::new(
                    UreqTransport::new(self.defaults.connect_timeout).map_err(|e| {
                        Error::invalid_config("failed to build blocking http transport", Some(e))
                    })?,
                )
            }
            #[cfg(not(test))]
            {
                Arc::new(
                    UreqTransport::new(self.defaults.connect_timeout).map_err(|e| {
                        Error::invalid_config("failed to build blocking http transport", Some(e))
                    })?,
                )
            }
        };

        Ok(BlockingClient {
            inner: Arc::new(Inner {
                auth: self.auth,
                endpoints: Endpoints { ecs, sts, billing },
                defaults: self.defaults,
                retry: self.retry,
                transport,
            }),
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::VecDeque,
        sync::{
            Arc, Mutex,
            atomic::{AtomicUsize, Ordering},
        },
        time::Duration,
    };

    use http::{HeaderMap, StatusCode};

    use crate::{
        auth::Auth,
        transport::{BlockingTransport, BoxError, Request, Response},
    };

    use super::*;

    struct MockBlockingTransport {
        calls: AtomicUsize,
        last_request: Mutex<Option<Request>>,
        responses: Mutex<VecDeque<Response>>,
    }

    impl MockBlockingTransport {
        fn new(responses: Vec<Response>) -> Self {
            Self {
                calls: AtomicUsize::new(0),
                last_request: Mutex::new(None),
                responses: Mutex::new(responses.into()),
            }
        }

        fn calls(&self) -> usize {
            self.calls.load(Ordering::SeqCst)
        }

        fn last_request(&self) -> Option<Request> {
            self.last_request.lock().unwrap().clone()
        }
    }

    impl BlockingTransport for MockBlockingTransport {
        fn send(&self, request: Request) -> Result<Response, BoxError> {
            self.calls.fetch_add(1, Ordering::SeqCst);
            *self.last_request.lock().unwrap() = Some(request.clone());

            let response = self.responses.lock().unwrap().pop_front();
            match response {
                Some(response) => Ok(response),
                None => Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::UnexpectedEof,
                    "no more mock responses",
                ))),
            }
        }
    }

    fn response(status: StatusCode, headers: HeaderMap, body: &str) -> Response {
        Response {
            status,
            headers,
            body: body.as_bytes().to_vec(),
        }
    }

    #[test]
    fn sts_error_body_is_classified_and_redacted() {
        let mut headers = HeaderMap::new();
        headers.insert("x-acs-request-id", "header-request-id".parse().unwrap());

        let transport = Arc::new(MockBlockingTransport::new(vec![response(
            StatusCode::OK,
            headers,
            r#"{"Code":"SignatureDoesNotMatch","Message":"bad signature","RequestId":"body-request-id","AccessKeySecret":"supersecret"}"#,
        )]));

        let client = BlockingClient::builder()
            .auth(Auth::access_key("id", "secret"))
            .sts_endpoint("https://sts.example.com/")
            .transport_override(transport)
            .build()
            .unwrap();

        let err = client.sts().get_caller_identity().unwrap_err();
        assert!(err.is_auth_error());
        assert_eq!(err.status(), Some(StatusCode::OK));
        assert_eq!(err.request_id(), Some("header-request-id"));

        let snippet = err.body_snippet().unwrap();
        assert!(!snippet.contains("supersecret"));
    }

    #[test]
    fn retry_on_503_then_succeeds() {
        let transport = Arc::new(MockBlockingTransport::new(vec![
            response(
                StatusCode::SERVICE_UNAVAILABLE,
                HeaderMap::new(),
                "temporary",
            ),
            response(
                StatusCode::OK,
                HeaderMap::new(),
                r#"{"IdentityType":"Account","RequestId":"req","AccountId":"1","PrincipalId":"p","UserId":"u","Arn":"arn","RoleId":null}"#,
            ),
        ]));

        let client = BlockingClient::builder()
            .auth(Auth::access_key("id", "secret"))
            .max_retries(1)
            .retry_base_delay(Duration::from_millis(0))
            .retry_max_delay(Duration::from_millis(0))
            .transport_override(transport.clone())
            .build()
            .unwrap();

        let ok = client.sts().get_caller_identity().unwrap();
        assert_eq!(ok.request_id, "req");
        assert_eq!(transport.calls(), 2);
    }

    #[test]
    fn capture_body_snippet_can_be_disabled() {
        let transport = Arc::new(MockBlockingTransport::new(vec![response(
            StatusCode::OK,
            HeaderMap::new(),
            r#"{"Code":"SignatureDoesNotMatch","Message":"bad signature","RequestId":"req","AccessKeySecret":"supersecret"}"#,
        )]));

        let client = BlockingClient::builder()
            .auth(Auth::access_key("id", "secret"))
            .capture_body_snippet(false)
            .transport_override(transport)
            .build()
            .unwrap();

        let err = client.sts().get_caller_identity().unwrap_err();
        assert!(err.body_snippet().is_none());
    }

    #[test]
    fn request_includes_required_rpc_query_params() {
        let transport = Arc::new(MockBlockingTransport::new(vec![response(
            StatusCode::OK,
            HeaderMap::new(),
            "{}",
        )]));

        let client = BlockingClient::builder()
            .auth(Auth::access_key("id", "secret"))
            .ecs_endpoint("https://ecs.example.com/")
            .transport_override(transport.clone())
            .build()
            .unwrap();

        let _ = client.ecs().describe_regions(Default::default()).unwrap();

        let request = transport.last_request().unwrap();
        let query = request.url.query().unwrap();
        assert!(query.contains("Action=DescribeRegions"));
        assert!(query.contains("SignatureMethod=HMAC-SHA1"));
        assert!(query.contains("SignatureNonce="));
        assert!(query.contains("Signature="));
    }
}
