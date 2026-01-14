use std::time::Duration;

#[cfg(feature = "async")]
use std::{future::Future, pin::Pin};

use http::{HeaderMap, Method, StatusCode};

pub(crate) mod retry;

#[cfg(feature = "async")]
pub(crate) mod async_transport;

#[cfg(feature = "blocking")]
pub(crate) mod blocking_transport;

pub(crate) type BoxError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Clone)]
pub(crate) struct Request {
    pub method: Method,
    pub url: url::Url,
    pub headers: HeaderMap,
    pub timeout: Duration,
}

#[derive(Debug, Clone)]
pub(crate) struct Response {
    pub status: StatusCode,
    pub headers: HeaderMap,
    pub body: Vec<u8>,
}

#[cfg(feature = "async")]
pub(crate) trait AsyncTransport: Send + Sync {
    fn send<'a>(
        &'a self,
        request: Request,
    ) -> Pin<Box<dyn Future<Output = Result<Response, BoxError>> + Send + 'a>>;
}

#[cfg(feature = "blocking")]
pub(crate) trait BlockingTransport: Send + Sync {
    fn send(&self, request: Request) -> Result<Response, BoxError>;
}
