use percent_encoding::{percent_encode, AsciiSet, NON_ALPHANUMERIC};

/// Encodes the given string using Aliyun's percent encoding rules,
/// preserving unreserved characters (A-Z, a-z, 0-9, '-', '_', '.', '~').
pub fn aliyun_percent_encode(input: &str) -> String {
    const ALIYUN_ENCODE_SET: &AsciiSet = &NON_ALPHANUMERIC
        .remove(b'-')
        .remove(b'_')
        .remove(b'.')
        .remove(b'~');
    percent_encode(input.as_bytes(), ALIYUN_ENCODE_SET).to_string()
}

/// Constructs the canonicalized query string by URL encoding each key and value,
/// then concatenating them as key=value pairs joined by '&'.
pub fn build_canonicalized_query(params: &std::collections::BTreeMap<String, String>) -> String {
    params
        .iter()
        .map(|(k, v)| format!("{}={}", aliyun_percent_encode(k), aliyun_percent_encode(v)))
        .collect::<Vec<String>>()
        .join("&")
}
