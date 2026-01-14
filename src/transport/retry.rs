use std::time::Duration;

use http::{HeaderMap, StatusCode, header};

/// Conservative retry configuration.
#[derive(Debug, Clone)]
pub(crate) struct RetryPolicy {
    pub max_retries: usize,
    pub base_delay: Duration,
    pub max_delay: Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(10),
        }
    }
}

pub(crate) fn should_retry_status(status: StatusCode) -> bool {
    matches!(
        status,
        StatusCode::TOO_MANY_REQUESTS
            | StatusCode::BAD_GATEWAY
            | StatusCode::SERVICE_UNAVAILABLE
            | StatusCode::GATEWAY_TIMEOUT
    )
}

pub(crate) fn parse_retry_after(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(header::RETRY_AFTER)?.to_str().ok()?.trim();

    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }

    let date = httpdate::parse_http_date(value).ok()?;
    let now = std::time::SystemTime::now();
    date.duration_since(now).ok()
}

pub(crate) fn backoff_delay(policy: &RetryPolicy, attempt: usize) -> Duration {
    let exp = 1u64 << attempt.min(31) as u32;
    let base = policy.base_delay.saturating_mul(exp as u32);
    let capped = base.min(policy.max_delay);

    // Full jitter: random value in [0, capped].
    Duration::from_millis(fastrand::u64(0..=capped.as_millis() as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retry_statuses_are_conservative() {
        assert!(should_retry_status(StatusCode::TOO_MANY_REQUESTS));
        assert!(should_retry_status(StatusCode::BAD_GATEWAY));
        assert!(should_retry_status(StatusCode::SERVICE_UNAVAILABLE));
        assert!(should_retry_status(StatusCode::GATEWAY_TIMEOUT));
        assert!(!should_retry_status(StatusCode::BAD_REQUEST));
    }

    #[test]
    fn retry_after_seconds_is_parsed() {
        let mut headers = HeaderMap::new();
        headers.insert(header::RETRY_AFTER, "3".parse().unwrap());
        assert_eq!(parse_retry_after(&headers), Some(Duration::from_secs(3)));
    }

    #[test]
    fn retry_after_http_date_is_parsed() {
        let now = std::time::SystemTime::now();
        let target = now + Duration::from_secs(2);
        let date = httpdate::fmt_http_date(target);

        let mut headers = HeaderMap::new();
        headers.insert(header::RETRY_AFTER, date.parse().unwrap());

        let delay = parse_retry_after(&headers).unwrap();
        assert!(delay <= Duration::from_secs(2));
    }

    #[test]
    fn backoff_is_capped() {
        let policy = RetryPolicy {
            max_retries: 3,
            base_delay: Duration::from_millis(50),
            max_delay: Duration::from_millis(200),
        };

        for attempt in 0..10 {
            let delay = backoff_delay(&policy, attempt);
            assert!(delay <= policy.max_delay);
        }
    }
}
