pub mod sts;
pub mod error;

use crate::signing;
use crate::utils;
use chrono::Utc;
use reqwest;
use serde_json::Value;
use std::collections::BTreeMap;
use uuid::Uuid;

/// The AliyunClient struct holds credentials and provides methods to send requests.
pub struct AliyunClient {
    pub access_key_id: String,
    pub access_key_secret: String,
}

impl AliyunClient {
    /// Creates a new AliyunClient instance with the provided access key and secret.
    pub fn new(access_key_id: String, access_key_secret: String) -> Self {
        Self {
            access_key_id,
            access_key_secret,
        }
    }

    /// Sends a request to the given endpoint with specific parameters.
    ///
    /// This method automatically adds common parameters (e.g., AccessKeyId, SignatureMethod, Timestamp, etc.),
    /// computes the signature, and constructs the final URL.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint (e.g., "business.aliyuncs.com" or "ecs.aliyuncs.com").
    /// * `params` - A BTreeMap containing the specific API parameters.
    ///
    /// # Returns
    ///
    /// A Result containing the parsed JSON response as a serde_json::Value, or an error.
    pub async fn send_request(
        &self,
        endpoint: &str,
        mut params: BTreeMap<String, String>,
    ) -> Result<Value, Box<dyn std::error::Error>> {
        // Insert common parameters
        params
            .entry("AccessKeyId".to_string())
            .or_insert(self.access_key_id.clone());
        params
            .entry("SignatureMethod".to_string())
            .or_insert("HMAC-SHA1".to_string());
        params
            .entry("SignatureVersion".to_string())
            .or_insert("1.0".to_string());
        params.insert("SignatureNonce".to_string(), Uuid::new_v4().to_string());
        let timestamp = Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string();
        params.insert("Timestamp".to_string(), timestamp);

        // Calculate signature using the signing module
        let signature = signing::calculate_signature(&params, &self.access_key_secret);
        params.insert("Signature".to_string(), signature);

        // Construct final URL query string
        let final_query = params
            .iter()
            .map(|(k, v)| {
                format!(
                    "{}={}",
                    utils::aliyun_percent_encode(k),
                    utils::aliyun_percent_encode(v)
                )
            })
            .collect::<Vec<String>>()
            .join("&");
        let url = format!("https://{}?{}", endpoint, final_query);

        // Send HTTP GET request
        let client = reqwest::Client::new();
        let response = client.get(&url).send().await?;
        let json = response.json::<Value>().await?;
        Ok(json)
    }
}
