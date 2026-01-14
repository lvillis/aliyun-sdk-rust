use http::{HeaderMap, Method, StatusCode, header};

use crate::{
    error::{Error, ErrorInfo},
    transport::{Response, retry::parse_retry_after},
    util::redact,
};

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
pub(crate) struct AliyunErrorBody {
    pub(crate) code: String,
    pub(crate) message: String,
    #[serde(default)]
    pub(crate) request_id: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
#[serde(untagged)]
pub(crate) enum AliyunEnvelope<T> {
    Err(AliyunErrorBody),
    Ok(T),
}

pub(crate) fn extract_request_id(headers: &HeaderMap) -> Option<String> {
    let names = [
        "x-acs-request-id",
        "x-request-id",
        "x-amzn-requestid",
        "x-amz-request-id",
    ];
    for name in names {
        if let Ok(name) = header::HeaderName::from_bytes(name.as_bytes())
            && let Some(value) = headers.get(name)
            && let Ok(value) = value.to_str()
            && !value.trim().is_empty()
        {
            return Some(value.trim().to_owned());
        }
    }
    None
}

pub(crate) fn maybe_body_snippet(enabled: bool, body: &[u8], max_len: usize) -> Option<String> {
    if !enabled {
        return None;
    }
    redact::body_snippet(body, max_len)
}

pub(crate) fn classify_http_error(
    method: Method,
    path: String,
    response: Response,
    request_id: Option<String>,
    capture_body_snippet: bool,
    max_body_snippet_len: usize,
) -> Error {
    let message = parse_aliyun_error_message(&response.body)
        .or_else(|| Some(format!("http status {}", response.status)));

    let info = Box::new(ErrorInfo {
        status: Some(response.status),
        method: Some(method.clone()),
        path: Some(path),
        message,
        request_id,
        body_snippet: maybe_body_snippet(
            capture_body_snippet,
            &response.body,
            max_body_snippet_len,
        ),
    });

    match response.status {
        StatusCode::UNAUTHORIZED | StatusCode::FORBIDDEN => Error::Auth { info },
        StatusCode::NOT_FOUND => Error::NotFound { info },
        StatusCode::CONFLICT | StatusCode::PRECONDITION_FAILED => Error::Conflict { info },
        StatusCode::TOO_MANY_REQUESTS => Error::RateLimited {
            retry_after: parse_retry_after(&response.headers),
            info,
        },
        _ => Error::Api { info },
    }
}

pub(crate) fn classify_aliyun_error(
    method: Method,
    path: String,
    status: StatusCode,
    request_id: Option<String>,
    body: AliyunErrorBody,
    body_snippet: Option<String>,
) -> Error {
    let request_id = request_id.or_else(|| body.request_id.clone());
    let message = Some(format!("{}: {}", body.code, body.message));
    let info = Box::new(ErrorInfo {
        status: Some(status),
        method: Some(method),
        path: Some(path),
        message,
        request_id,
        body_snippet,
    });

    if is_auth_error_code(&body.code) {
        return Error::Auth { info };
    }

    Error::Api { info }
}

fn parse_aliyun_error_message(body: &[u8]) -> Option<String> {
    let parsed: serde_json::Value = serde_json::from_slice(body).ok()?;

    let code = parsed.get("Code")?.as_str()?.trim();
    let message = parsed.get("Message")?.as_str()?.trim();
    if code.is_empty() && message.is_empty() {
        return None;
    }
    Some(format!("{code}: {message}"))
}

fn is_auth_error_code(code: &str) -> bool {
    matches!(
        code,
        "InvalidAccessKeyId.NotFound"
            | "InvalidAccessKeyId"
            | "SignatureDoesNotMatch"
            | "InvalidSecurityToken"
            | "UnauthorizedOperation"
    )
}
