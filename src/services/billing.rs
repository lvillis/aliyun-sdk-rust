use crate::client::AliyunClient;
use serde_json::Value;
use std::collections::BTreeMap;

/// Query Account Balance - QueryAccountBalance
///
/// **API Description:**
/// - Request Domain: business.aliyuncs.com
/// - API Version: 2017-12-14
/// - Description: This API is used to query the user's account balance.
///
/// **Input Parameters:**
///
/// | Parameter         | Type   | Description                                        |
/// |-------------------|--------|----------------------------------------------------|
/// | Action            | String | Fixed value: "QueryAccountBalance"                 |
/// | Format            | String | Fixed value: "JSON"                                |
/// | Version           | String | Fixed value: "2017-12-14"                          |
/// | AccessKeyId       | String | Provided by the client's credentials             |
/// | SignatureMethod   | String | Fixed value: "HMAC-SHA1"                           |
/// | SignatureVersion  | String | Fixed value: "1.0"                                 |
/// | SignatureNonce    | String | A unique random string                             |
/// | Timestamp         | String | UTC time in ISO8601 format                         |
/// | Signature         | String | Calculated signature for authentication          |
///
/// **Output Parameters:**
///
/// | Field      | Type    | Description                                      |
/// |------------|---------|--------------------------------------------------|
/// | Code       | String  | Response code (e.g., "200" indicates success)    |
/// | Message    | String  | Response message                                 |
/// | RequestId  | String  | Unique request ID                                |
/// | Success    | Boolean | Indicates whether the API call was successful    |
/// | Data       | Object  | Account balance details                          |
pub async fn query_account_balance(
    client: &AliyunClient,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut params: BTreeMap<String, String> = BTreeMap::new();
    params.insert("Action".to_string(), "QueryAccountBalance".to_string());
    params.insert("Format".to_string(), "JSON".to_string());
    // API version as per official documentation
    params.insert("Version".to_string(), "2017-12-14".to_string());
    client.send_request("business.aliyuncs.com", params).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::AliyunClient;
    use crate::test_utils::TEST_SECRETS;
    use tokio;

    #[tokio::test]
    async fn test_query_account_balance() {
        // Use the centralized TEST_SECRETS to build the client
        let secrets = &*TEST_SECRETS;
        let client = AliyunClient::new(
            secrets.access_key_id.clone(),
            secrets.access_key_secret.clone(),
        );

        let result = query_account_balance(&client).await;
        match &result {
            Ok(json) => println!("Billing module - Query Account Balance success:\n{}", json),
            Err(err) => eprintln!("Billing module - Query Account Balance error:\n{}", err),
        }

        assert!(
            result.is_ok(),
            "Billing module - QueryAccountBalance API call failed: {:?}",
            result.err()
        );
    }
}
