use crate::utils;
use base64::{engine::general_purpose, Engine as _};
use hmac::{Hmac, Mac};
use sha1::Sha1;
use std::collections::BTreeMap;

pub type HmacSha1 = Hmac<Sha1>;

/// Calculates the Aliyun API signature for the given parameters.
///
/// # Arguments
///
/// * `params` - A reference to a BTreeMap containing the request parameters (already sorted).
/// * `access_key_secret` - The user's AccessKeySecret.
///
/// # Returns
///
/// The calculated signature as a Base64 encoded string.
pub fn calculate_signature(params: &BTreeMap<String, String>, access_key_secret: &str) -> String {
    // Build the canonicalized query string
    let canonicalized_query = utils::build_canonicalized_query(params);
    // Construct the string to sign in the format: "GET&%2F&<URL-encoded canonicalized query>"
    let string_to_sign = format!(
        "GET&{}&{}",
        utils::aliyun_percent_encode("/"),
        utils::aliyun_percent_encode(&canonicalized_query)
    );

    // The signing key is AccessKeySecret appended with "&"
    let key = format!("{}&", access_key_secret);
    let mut mac = HmacSha1::new_from_slice(key.as_bytes()).expect("HMAC can take key of any size");
    mac.update(string_to_sign.as_bytes());
    let result = mac.finalize().into_bytes();
    // Use the new Engine API for base64 encoding
    general_purpose::STANDARD.encode(&result)
}
