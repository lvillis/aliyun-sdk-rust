use std::{future::Future, pin::Pin, time::Duration};

use crate::transport::{AsyncTransport, BoxError, Request, Response};

#[cfg(feature = "native-tls")]
pub(crate) struct ReqwestTransport {
    client: reqwest::Client,
}

#[cfg(feature = "native-tls")]
impl ReqwestTransport {
    pub(crate) fn new(connect_timeout: Duration) -> Result<Self, BoxError> {
        let client = reqwest::Client::builder()
            .connect_timeout(connect_timeout)
            .build()?;
        Ok(Self { client })
    }
}

#[cfg(feature = "rustls")]
pub(crate) struct HyperRustlsTransport {
    client: hyper_util::client::legacy::Client<
        hyper_rustls::HttpsConnector<hyper_util::client::legacy::connect::HttpConnector>,
        http_body_util::Empty<hyper::body::Bytes>,
    >,
}

#[cfg(feature = "rustls")]
impl HyperRustlsTransport {
    pub(crate) fn new(connect_timeout: Duration) -> Result<Self, BoxError> {
        use std::sync::Arc;

        use hyper_util::client::legacy::connect::HttpConnector;
        use hyper_util::rt::TokioExecutor;

        let mut http = HttpConnector::new();
        http.set_connect_timeout(Some(connect_timeout));
        http.enforce_http(false);

        let https = hyper_rustls::HttpsConnectorBuilder::new()
            .with_provider_and_platform_verifier(
                Arc::new(rustls::crypto::ring::default_provider()),
            )?
            .https_or_http()
            .enable_http1()
            .wrap_connector(http);

        let client = hyper_util::client::legacy::Client::builder(TokioExecutor::new()).build(https);

        Ok(Self { client })
    }
}

#[cfg(feature = "rustls")]
impl AsyncTransport for HyperRustlsTransport {
    fn send<'a>(
        &'a self,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = Result<Response, BoxError>> + Send + 'a>> {
        use http_body_util::BodyExt;

        Box::pin(async move {
            let timeout = request.timeout;

            let mut builder = http::Request::builder()
                .method(request.method)
                .uri(request.url.as_str());

            for (name, value) in request.headers.iter() {
                builder = builder.header(name, value);
            }

            let http_request = builder.body(http_body_util::Empty::new())?;

            let response =
                tokio::time::timeout(timeout, self.client.request(http_request)).await??;

            let status = response.status();
            let headers = response.headers().clone();
            let body = response.into_body().collect().await?.to_bytes().to_vec();

            Ok(Response {
                status,
                headers,
                body,
            })
        })
    }
}

#[cfg(feature = "native-tls")]
impl AsyncTransport for ReqwestTransport {
    fn send<'a>(
        &'a self,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = Result<Response, BoxError>> + Send + 'a>> {
        Box::pin(async move {
            let response = self
                .client
                .request(request.method, request.url)
                .headers(request.headers)
                .timeout(request.timeout)
                .send()
                .await?;

            let status = response.status();
            let headers = response.headers().clone();
            let body = response.bytes().await?.to_vec();

            Ok(Response {
                status,
                headers,
                body,
            })
        })
    }
}
