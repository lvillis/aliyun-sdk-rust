use std::{collections::BTreeMap, sync::Arc, time::Duration};

#[cfg(feature = "tracing")]
use std::time::Instant;

use http::{HeaderMap, HeaderValue, Method, header};
use serde::de::DeserializeOwned;

use crate::{
    api::{BillingService, EcsService, StsService},
    auth::Auth,
    error::{Error, ErrorInfo},
    transport::{
        AsyncTransport, Request, Response,
        retry::{RetryPolicy, backoff_delay, parse_retry_after, should_retry_status},
    },
    util::{rpc, url as url_util},
};

#[cfg(feature = "rustls")]
use crate::transport::async_transport::HyperRustlsTransport;
#[cfg(feature = "native-tls")]
use crate::transport::async_transport::ReqwestTransport;

use super::common::{
    AliyunEnvelope, classify_aliyun_error, classify_http_error, extract_request_id,
    maybe_body_snippet,
};

#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}

struct Inner {
    auth: Auth,
    endpoints: Endpoints,
    defaults: RequestDefaults,
    retry: RetryPolicy,
    transport: Arc<dyn AsyncTransport>,
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

pub struct ClientBuilder {
    auth: Auth,
    ecs_endpoint: String,
    sts_endpoint: String,
    billing_endpoint: String,
    defaults: RequestDefaults,
    retry: RetryPolicy,
    #[cfg(test)]
    transport_override: Option<Arc<dyn AsyncTransport>>,
}

impl Client {
    pub fn builder() -> ClientBuilder {
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            header::USER_AGENT,
            HeaderValue::from_static(concat!("alibabacloud/", env!("CARGO_PKG_VERSION"))),
        );

        ClientBuilder {
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

    pub fn ecs(&self) -> EcsService {
        EcsService::new(self.clone())
    }

    pub fn sts(&self) -> StsService {
        StsService::new(self.clone())
    }

    pub fn billing(&self) -> BillingService {
        BillingService::new(self.clone())
    }

    pub(crate) async fn rpc_json<T: DeserializeOwned>(
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

        self.send_json(Method::GET, url).await
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

    async fn send_json<T: DeserializeOwned>(
        &self,
        method: Method,
        url: url::Url,
    ) -> Result<T, Error> {
        let path = url.path().to_owned();
        #[cfg(feature = "tracing")]
        let start = Instant::now();

        #[cfg(feature = "tracing")]
        let host = url.host_str().unwrap_or("<unknown>");
        #[cfg(feature = "tracing")]
        let span = tracing::info_span!(
            "alibabacloud.request",
            method = %method,
            host = host,
            path = %path,
            status = tracing::field::Empty,
            latency_ms = tracing::field::Empty,
            retry_count = tracing::field::Empty,
            request_id = tracing::field::Empty,
        );
        #[cfg(feature = "tracing")]
        let _guard = span.enter();

        let headers = self.inner.defaults.default_headers.clone();

        let request = Request {
            method: method.clone(),
            url,
            headers,
            timeout: self.inner.defaults.timeout,
        };

        let response = match self.send_with_retries(&request).await {
            Ok(response) => response,
            Err(error) => {
                #[cfg(feature = "tracing")]
                {
                    let latency_ms = start.elapsed().as_millis() as u64;
                    record_span_outcome(error.status(), error.request_id(), latency_ms);
                    tracing::warn!(error_kind = error_kind(&error), "request failed");
                }
                return Err(error);
            }
        };

        let request_id = extract_request_id(&response.headers);
        #[cfg(feature = "tracing")]
        {
            record_span_outcome(Some(response.status), request_id.as_deref(), 0);
        }

        if !response.status.is_success() {
            let error = classify_http_error(
                method,
                path,
                response,
                request_id,
                self.inner.defaults.capture_body_snippet,
                self.inner.defaults.body_snippet_max_len,
            );
            #[cfg(feature = "tracing")]
            {
                let latency_ms = start.elapsed().as_millis() as u64;
                record_span_outcome(error.status(), error.request_id(), latency_ms);
                tracing::warn!(error_kind = error_kind(&error), "request failed");
            }
            return Err(error);
        }

        let mut deserializer = serde_json::Deserializer::from_slice(&response.body);
        let parsed = serde_path_to_error::deserialize::<_, AliyunEnvelope<T>>(&mut deserializer);
        match parsed {
            Ok(AliyunEnvelope::Ok(value)) => {
                #[cfg(feature = "tracing")]
                {
                    let latency_ms = start.elapsed().as_millis() as u64;
                    record_span_outcome(Some(response.status), request_id.as_deref(), latency_ms);
                }
                Ok(value)
            }
            Ok(AliyunEnvelope::Err(err)) => {
                let body_snippet = maybe_body_snippet(
                    self.inner.defaults.capture_body_snippet,
                    &response.body,
                    self.inner.defaults.body_snippet_max_len,
                );
                let error = classify_aliyun_error(
                    method,
                    path,
                    response.status,
                    request_id,
                    err,
                    body_snippet,
                );
                #[cfg(feature = "tracing")]
                {
                    let latency_ms = start.elapsed().as_millis() as u64;
                    record_span_outcome(error.status(), error.request_id(), latency_ms);
                    tracing::warn!(error_kind = error_kind(&error), "request failed");
                }
                Err(error)
            }
            Err(source) => {
                let error = Error::Decode {
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
                };
                #[cfg(feature = "tracing")]
                {
                    let latency_ms = start.elapsed().as_millis() as u64;
                    record_span_outcome(error.status(), error.request_id(), latency_ms);
                    tracing::warn!(error_kind = error_kind(&error), "request failed");
                }
                Err(error)
            }
        }
    }

    async fn send_with_retries(&self, request: &Request) -> Result<Response, Error> {
        let mut attempt = 0usize;
        loop {
            let result = self.inner.transport.send(request.clone()).await;
            match result {
                Ok(response) => {
                    if attempt >= self.inner.retry.max_retries
                        || !should_retry_status(response.status)
                    {
                        #[cfg(feature = "tracing")]
                        tracing::Span::current().record("retry_count", attempt as u64);
                        return Ok(response);
                    }

                    let delay = parse_retry_after(&response.headers)
                        .unwrap_or_else(|| backoff_delay(&self.inner.retry, attempt));
                    #[cfg(feature = "tracing")]
                    tracing::debug!(
                        retry_count = attempt + 1,
                        delay_ms = delay.as_millis() as u64,
                        status = response.status.as_u16(),
                        "retrying request"
                    );
                    tokio::time::sleep(delay).await;
                    attempt += 1;
                    continue;
                }
                Err(source) => {
                    if attempt < self.inner.retry.max_retries
                        && is_retryable_transport_error(&*source)
                    {
                        let delay = backoff_delay(&self.inner.retry, attempt);
                        #[cfg(feature = "tracing")]
                        tracing::debug!(
                            retry_count = attempt + 1,
                            delay_ms = delay.as_millis() as u64,
                            "retrying after transport error"
                        );
                        tokio::time::sleep(delay).await;
                        attempt += 1;
                        continue;
                    }

                    #[cfg(feature = "tracing")]
                    tracing::Span::current().record("retry_count", attempt as u64);
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

#[cfg(feature = "tracing")]
fn record_span_outcome(
    status: Option<http::StatusCode>,
    request_id: Option<&str>,
    latency_ms: u64,
) {
    let span = tracing::Span::current();
    if let Some(status) = status {
        span.record("status", status.as_u16());
    }
    if let Some(request_id) = request_id {
        span.record("request_id", request_id);
    }
    if latency_ms > 0 {
        span.record("latency_ms", latency_ms);
    }
}

#[cfg(feature = "tracing")]
fn error_kind(error: &Error) -> &'static str {
    match error {
        Error::InvalidConfig { .. } => "invalid_config",
        Error::Auth { .. } => "auth",
        Error::NotFound { .. } => "not_found",
        Error::Conflict { .. } => "conflict",
        Error::RateLimited { .. } => "rate_limited",
        Error::Api { .. } => "api",
        Error::Transport { .. } => "transport",
        Error::Decode { .. } => "decode",
    }
}

impl ClientBuilder {
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
    pub(crate) fn transport_override(mut self, transport: Arc<dyn AsyncTransport>) -> Self {
        self.transport_override = Some(transport);
        self
    }

    pub fn build(self) -> Result<Client, Error> {
        let ecs = url_util::parse_base_url(&self.ecs_endpoint)?;
        let sts = url_util::parse_base_url(&self.sts_endpoint)?;
        let billing = url_util::parse_base_url(&self.billing_endpoint)?;

        let transport: Arc<dyn AsyncTransport> = {
            #[cfg(test)]
            if let Some(transport) = self.transport_override {
                transport
            } else {
                default_transport(self.defaults.connect_timeout)?
            }
            #[cfg(not(test))]
            {
                default_transport(self.defaults.connect_timeout)?
            }
        };

        Ok(Client {
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

fn default_transport(connect_timeout: Duration) -> Result<Arc<dyn AsyncTransport>, Error> {
    #[cfg(feature = "native-tls")]
    {
        return Ok(Arc::new(ReqwestTransport::new(connect_timeout).map_err(
            |e| Error::invalid_config("failed to build async http transport (reqwest)", Some(e)),
        )?));
    }

    #[cfg(feature = "rustls")]
    {
        return Ok(Arc::new(
            HyperRustlsTransport::new(connect_timeout).map_err(|e| {
                Error::invalid_config(
                    "failed to build async http transport (hyper-rustls)",
                    Some(e),
                )
            })?,
        ));
    }

    #[allow(unreachable_code)]
    Err(Error::invalid_config(
        "no async http transport available",
        None,
    ))
}

fn is_retryable_transport_error(error: &(dyn std::error::Error + 'static)) -> bool {
    if let Some(err) = error.downcast_ref::<reqwest::Error>() {
        return err.is_timeout() || err.is_connect();
    }

    #[cfg(feature = "rustls")]
    if let Some(err) = error.downcast_ref::<hyper::Error>()
        && err.is_timeout()
    {
        return true;
    }

    if error
        .downcast_ref::<tokio::time::error::Elapsed>()
        .is_some()
    {
        return true;
    }

    let mut source: Option<&(dyn std::error::Error + 'static)> = Some(error);
    while let Some(err) = source {
        if let Some(io) = err.downcast_ref::<std::io::Error>() {
            return matches!(
                io.kind(),
                std::io::ErrorKind::TimedOut
                    | std::io::ErrorKind::ConnectionRefused
                    | std::io::ErrorKind::ConnectionReset
                    | std::io::ErrorKind::ConnectionAborted
                    | std::io::ErrorKind::NotConnected
                    | std::io::ErrorKind::AddrInUse
                    | std::io::ErrorKind::AddrNotAvailable
            );
        }
        source = err.source();
    }

    false
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
        transport::{AsyncTransport, BoxError, Request, Response},
    };

    use super::*;

    struct MockAsyncTransport {
        calls: AtomicUsize,
        last_request: Mutex<Option<Request>>,
        responses: Mutex<VecDeque<Response>>,
    }

    impl MockAsyncTransport {
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

    impl AsyncTransport for MockAsyncTransport {
        fn send<'a>(
            &'a self,
            request: Request,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<Response, BoxError>> + Send + 'a>,
        > {
            self.calls.fetch_add(1, Ordering::SeqCst);
            *self.last_request.lock().unwrap() = Some(request.clone());

            let response = self.responses.lock().unwrap().pop_front();
            Box::pin(async move {
                match response {
                    Some(response) => Ok(response),
                    None => {
                        let err: BoxError = Box::new(std::io::Error::new(
                            std::io::ErrorKind::UnexpectedEof,
                            "no more mock responses",
                        ));
                        Err(err)
                    }
                }
            })
        }
    }

    fn response(status: StatusCode, headers: HeaderMap, body: &str) -> Response {
        Response {
            status,
            headers,
            body: body.as_bytes().to_vec(),
        }
    }

    #[tokio::test]
    async fn sts_error_body_is_classified_and_redacted() {
        let mut headers = HeaderMap::new();
        headers.insert("x-acs-request-id", "header-request-id".parse().unwrap());

        let transport = Arc::new(MockAsyncTransport::new(vec![response(
            StatusCode::OK,
            headers,
            r#"{"Code":"SignatureDoesNotMatch","Message":"bad signature","RequestId":"body-request-id","AccessKeySecret":"supersecret"}"#,
        )]));

        let client = Client::builder()
            .auth(Auth::access_key("id", "secret"))
            .sts_endpoint("https://sts.example.com/")
            .transport_override(transport)
            .build()
            .unwrap();

        let err = client.sts().get_caller_identity().await.unwrap_err();
        assert!(err.is_auth_error());
        assert_eq!(err.status(), Some(StatusCode::OK));
        assert_eq!(err.request_id(), Some("header-request-id"));

        let snippet = err.body_snippet().unwrap();
        assert!(!snippet.contains("supersecret"));
    }

    #[tokio::test]
    async fn retry_on_503_then_succeeds() {
        let transport = Arc::new(MockAsyncTransport::new(vec![
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

        let client = Client::builder()
            .auth(Auth::access_key("id", "secret"))
            .max_retries(1)
            .retry_base_delay(Duration::from_millis(0))
            .retry_max_delay(Duration::from_millis(0))
            .transport_override(transport.clone())
            .build()
            .unwrap();

        let ok = client.sts().get_caller_identity().await.unwrap();
        assert_eq!(ok.request_id, "req");
        assert_eq!(transport.calls(), 2);
    }

    #[tokio::test]
    async fn capture_body_snippet_can_be_disabled() {
        let transport = Arc::new(MockAsyncTransport::new(vec![response(
            StatusCode::OK,
            HeaderMap::new(),
            r#"{"Code":"SignatureDoesNotMatch","Message":"bad signature","RequestId":"req","AccessKeySecret":"supersecret"}"#,
        )]));

        let client = Client::builder()
            .auth(Auth::access_key("id", "secret"))
            .capture_body_snippet(false)
            .transport_override(transport)
            .build()
            .unwrap();

        let err = client.sts().get_caller_identity().await.unwrap_err();
        assert!(err.body_snippet().is_none());
    }

    #[tokio::test]
    async fn request_includes_required_rpc_query_params() {
        let transport = Arc::new(MockAsyncTransport::new(vec![response(
            StatusCode::OK,
            HeaderMap::new(),
            "{}",
        )]));

        let client = Client::builder()
            .auth(Auth::access_key("id", "secret"))
            .ecs_endpoint("https://ecs.example.com/")
            .transport_override(transport.clone())
            .build()
            .unwrap();

        let _ = client
            .ecs()
            .describe_regions(Default::default())
            .await
            .unwrap();

        let request = transport.last_request().unwrap();
        let query = request.url.query().unwrap();
        assert!(query.contains("Action=DescribeRegions"));
        assert!(query.contains("SignatureMethod=HMAC-SHA1"));
        assert!(query.contains("SignatureNonce="));
        assert!(query.contains("Signature="));
    }
}
