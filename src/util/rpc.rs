use std::collections::BTreeMap;

use base64::{Engine as _, engine::general_purpose};
use hmac::{Hmac, Mac};
use sha1::Sha1;

use crate::auth::AccessKey;
use crate::error::Error;

pub(crate) type HmacSha1 = Hmac<Sha1>;

pub(crate) fn percent_encode(input: &str) -> String {
    const ALIYUN_ENCODE_SET: &percent_encoding::AsciiSet = &percent_encoding::NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'~');
    percent_encoding::percent_encode(input.as_bytes(), ALIYUN_ENCODE_SET).to_string()
}

pub(crate) fn canonical_query(params: &BTreeMap<String, String>) -> String {
    params
        .iter()
        .map(|(k, v)| format!("{}={}", percent_encode(k), percent_encode(v)))
        .collect::<Vec<_>>()
        .join("&")
}

pub(crate) fn signature(
    method: &http::Method,
    canonical_query: &str,
    secret: &str,
) -> Result<String, Error> {
    let string_to_sign = format!(
        "{}&{}&{}",
        method.as_str(),
        percent_encode("/"),
        percent_encode(canonical_query)
    );

    let key = format!("{}&", secret);
    let mut mac = HmacSha1::new_from_slice(key.as_bytes())
        .map_err(|e| Error::invalid_config("invalid signing key", Some(Box::new(e))))?;
    mac.update(string_to_sign.as_bytes());

    Ok(general_purpose::STANDARD.encode(mac.finalize().into_bytes()))
}

pub(crate) fn timestamp() -> Result<String, Error> {
    let now = time::OffsetDateTime::now_utc()
        .replace_nanosecond(0)
        .map_err(|e| Error::invalid_config("failed to normalize timestamp", Some(Box::new(e))))?;
    now.format(&time::format_description::well_known::Rfc3339)
        .map_err(|e| Error::invalid_config("failed to format timestamp", Some(Box::new(e))))
}

pub(crate) fn inject_common_rpc_params(
    params: &mut BTreeMap<String, String>,
    access_key: &AccessKey,
) -> Result<(), Error> {
    params.insert("AccessKeyId".to_owned(), access_key.access_key_id.clone());
    params.insert("SignatureMethod".to_owned(), "HMAC-SHA1".to_owned());
    params.insert("SignatureVersion".to_owned(), "1.0".to_owned());
    params.insert(
        "SignatureNonce".to_owned(),
        uuid::Uuid::new_v4().to_string(),
    );
    params.insert("Timestamp".to_owned(), timestamp()?);
    if let Some(token) = access_key.security_token.as_ref() {
        params.insert("SecurityToken".to_owned(), token.expose().to_owned());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn percent_encode_uses_rfc3986_unreserved_set() {
        assert_eq!(percent_encode("a b"), "a%20b");
        assert_eq!(percent_encode("~"), "~");
        assert_eq!(percent_encode(":"), "%3A");
        assert_eq!(percent_encode("/"), "%2F");
    }

    #[test]
    fn canonical_query_is_sorted_and_encoded() {
        let mut params = BTreeMap::new();
        params.insert("b".to_owned(), "2".to_owned());
        params.insert("a".to_owned(), "1".to_owned());
        params.insert("space".to_owned(), "a b".to_owned());

        assert_eq!(canonical_query(&params), "a=1&b=2&space=a%20b");
    }

    #[test]
    fn signature_matches_known_vector() {
        let mut params = BTreeMap::new();
        params.insert("AccessKeyId".to_owned(), "testid".to_owned());
        params.insert("Action".to_owned(), "DescribeRegions".to_owned());
        params.insert("Format".to_owned(), "JSON".to_owned());
        params.insert("RegionId".to_owned(), "cn-hangzhou".to_owned());
        params.insert("SignatureMethod".to_owned(), "HMAC-SHA1".to_owned());
        params.insert("SignatureNonce".to_owned(), "testnonce".to_owned());
        params.insert("SignatureVersion".to_owned(), "1.0".to_owned());
        params.insert("Timestamp".to_owned(), "2015-01-01T12:00:00Z".to_owned());
        params.insert("Version".to_owned(), "2014-05-26".to_owned());

        let canonical = canonical_query(&params);
        assert_eq!(
            canonical,
            "AccessKeyId=testid&Action=DescribeRegions&Format=JSON&RegionId=cn-hangzhou&SignatureMethod=HMAC-SHA1&SignatureNonce=testnonce&SignatureVersion=1.0&Timestamp=2015-01-01T12%3A00%3A00Z&Version=2014-05-26"
        );

        let sig = signature(&http::Method::GET, &canonical, "testsecret").unwrap();
        assert_eq!(sig, "D93NxUhhlH206jRKH5QQOSAUcT4=");
    }

    #[test]
    fn timestamp_is_seconds_precision_utc() {
        let ts = timestamp().unwrap();
        assert!(ts.ends_with('Z'));
        assert!(!ts.contains('.'));
    }
}
