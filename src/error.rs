use std::{error::Error as StdError, fmt, time::Duration};

use http::{Method, StatusCode};

/// SDK error.
#[non_exhaustive]
#[derive(Debug)]
pub enum Error {
    /// Configuration or input validation errors.
    InvalidConfig {
        message: String,
        source: Option<Box<dyn StdError + Send + Sync + 'static>>,
    },

    /// Authentication errors (401/403, or API-level auth failures).
    Auth { info: Box<ErrorInfo> },

    /// Resource not found (404).
    NotFound { info: Box<ErrorInfo> },

    /// Conflict errors (409/412).
    Conflict { info: Box<ErrorInfo> },

    /// Rate limited (429).
    RateLimited {
        info: Box<ErrorInfo>,
        retry_after: Option<Duration>,
    },

    /// API errors (non-2xx responses or well-formed error bodies).
    Api { info: Box<ErrorInfo> },

    /// Network/TLS/timeout errors.
    Transport {
        info: Box<ErrorInfo>,
        source: Box<dyn StdError + Send + Sync + 'static>,
    },

    /// JSON decoding errors.
    Decode {
        info: Box<ErrorInfo>,
        source: Box<dyn StdError + Send + Sync + 'static>,
    },
}

/// Error diagnostics (safe to print).
#[derive(Debug, Clone, Default)]
pub struct ErrorInfo {
    pub(crate) status: Option<StatusCode>,
    pub(crate) method: Option<Method>,
    pub(crate) path: Option<String>,
    pub(crate) message: Option<String>,
    pub(crate) request_id: Option<String>,
    pub(crate) body_snippet: Option<String>,
}

impl Error {
    pub(crate) fn invalid_config(
        message: impl Into<String>,
        source: Option<Box<dyn StdError + Send + Sync + 'static>>,
    ) -> Self {
        Self::InvalidConfig {
            message: message.into(),
            source,
        }
    }

    pub fn status(&self) -> Option<StatusCode> {
        self.info().and_then(|info| info.status)
    }

    pub fn request_id(&self) -> Option<&str> {
        self.info().and_then(|info| info.request_id.as_deref())
    }

    pub fn body_snippet(&self) -> Option<&str> {
        self.info().and_then(|info| info.body_snippet.as_deref())
    }

    pub fn message(&self) -> Option<&str> {
        self.info().and_then(|info| info.message.as_deref())
    }

    pub fn path(&self) -> Option<&str> {
        self.info().and_then(|info| info.path.as_deref())
    }

    pub fn method(&self) -> Option<&Method> {
        self.info().and_then(|info| info.method.as_ref())
    }

    pub fn is_auth_error(&self) -> bool {
        matches!(self, Error::Auth { .. })
    }

    pub fn is_retryable(&self) -> bool {
        match self {
            Error::RateLimited { .. } => true,
            Error::Transport { .. } => true,
            Error::Api { info }
            | Error::Auth { info }
            | Error::NotFound { info }
            | Error::Conflict { info } => {
                if let Some(status) = info.status {
                    matches!(
                        status,
                        StatusCode::TOO_MANY_REQUESTS
                            | StatusCode::BAD_GATEWAY
                            | StatusCode::SERVICE_UNAVAILABLE
                            | StatusCode::GATEWAY_TIMEOUT
                    )
                } else {
                    false
                }
            }
            Error::Decode { .. } | Error::InvalidConfig { .. } => false,
        }
    }

    pub(crate) fn info(&self) -> Option<&ErrorInfo> {
        match self {
            Error::InvalidConfig { .. } => None,
            Error::Auth { info }
            | Error::NotFound { info }
            | Error::Conflict { info }
            | Error::RateLimited { info, .. }
            | Error::Api { info }
            | Error::Transport { info, .. }
            | Error::Decode { info, .. } => Some(info.as_ref()),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidConfig { message, .. } => write!(f, "invalid configuration: {message}"),
            Error::Auth { info } => write!(f, "authentication failed{}", display_suffix(info)),
            Error::NotFound { info } => write!(f, "resource not found{}", display_suffix(info)),
            Error::Conflict { info } => write!(f, "request conflict{}", display_suffix(info)),
            Error::RateLimited { info, retry_after } => {
                if let Some(wait) = retry_after {
                    write!(
                        f,
                        "rate limited (retry-after={:?}){}",
                        wait,
                        display_suffix(info)
                    )
                } else {
                    write!(f, "rate limited{}", display_suffix(info))
                }
            }
            Error::Api { info } => write!(f, "api error{}", display_suffix(info)),
            Error::Transport { info, .. } => write!(f, "transport error{}", display_suffix(info)),
            Error::Decode { info, .. } => write!(f, "decode error{}", display_suffix(info)),
        }
    }
}

fn display_suffix(info: &ErrorInfo) -> String {
    let mut parts = Vec::new();
    if let Some(status) = info.status {
        parts.push(format!("status={status}"));
    }
    if let (Some(method), Some(path)) = (info.method.as_ref(), info.path.as_deref()) {
        parts.push(format!("request={method} {path}"));
    } else if let Some(path) = info.path.as_deref() {
        parts.push(format!("path={path}"));
    }
    if let Some(request_id) = info.request_id.as_deref() {
        parts.push(format!("request_id={request_id}"));
    }
    if let Some(message) = info.message.as_deref() {
        parts.push(format!("message={message}"));
    }
    if parts.is_empty() {
        return String::new();
    }
    format!(" ({})", parts.join(", "))
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::InvalidConfig { source, .. } => source.as_deref().map(|e| e as _),
            Error::Transport { source, .. } => Some(source.as_ref() as _),
            Error::Decode { source, .. } => Some(source.as_ref() as _),
            _ => None,
        }
    }
}
