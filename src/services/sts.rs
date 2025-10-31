use crate::client::AliyunClient;
use serde_json::Value;
use std::{collections::BTreeMap, error::Error};

/// Get Caller Identity - GetCallerIdentity
///
/// **API Description:**
/// - Request Domain: sts.aliyuncs.com
/// - API Version: 2015-04-01
/// - Description: This API returns information about the caller's identity.
/// - Aliyun Document Link: https://help.aliyun.com/zh/ram/developer-reference/api-sts-2015-04-01-getcalleridentity
///
/// **Input Parameters:**
///
/// | Parameter        | Type   | Description                                    |
/// |------------------|--------|------------------------------------------------|
/// | Action           | String | Fixed value: "GetCallerIdentity"               |
/// | Format           | String | Fixed value: "JSON"                            |
/// | Version          | String | Fixed value: "2015-04-01"                      |
/// | AccessKeyId      | String | Provided by the client's credentials           |
/// | SignatureMethod  | String | Fixed value: "HMAC-SHA1"                       |
/// | SignatureVersion | String | Fixed value: "1.0"                             |
/// | SignatureNonce   | String | A unique random string                         |
/// | Timestamp        | String | UTC time in ISO8601 format                     |
/// | Signature        | String | Calculated signature for authentication        |
///
/// **Output Parameters:**
///
/// | Field       | Type   | Description                                                      |
/// |-------------|--------|------------------------------------------------------------------|
/// | IdentityType| String | Type of identity ("Account", "RAMUser", "AssumedRoleUser")                          |
/// | RequestId   | String | Unique request ID                                                |
/// | AccountId   | String | Alibaba Cloud account ID                                         |
/// | PrincipalId | String | Principal identifier                                             |
/// | UserId      | String | The user ID                                                      |
/// | Arn         | String | The ARN of the caller                                            |
/// | RoleId      | String | The role id; returned only when the current caller is a RAM role |
pub async fn get_caller_identity(client: &AliyunClient) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    // todo: abstract `to_string`
    params.insert("Action".to_string(), "GetCallerIdentity".to_string());
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2015-04-01".to_string());
    client.send_request("sts.aliyuncs.com", params).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::create_aliyun_client;

    #[tokio::test]
    async fn test_get_caller_identity() {
        let client = create_aliyun_client();

        let result = get_caller_identity(&client).await;
        println!("get_caller_identity: {:?}", result);
        assert!(result.is_ok());

        let response = result.unwrap();
        assert!(response.get("IdentityType").is_some());
        assert!(response.get("RequestId").is_some());
        assert!(response.get("AccountId").is_some());
        assert!(response.get("PrincipalId").is_some());
        assert!(response.get("UserId").is_some());
        assert!(response.get("Arn").is_some());
        // Returned only when the current caller is a RAM role.
        // assert!(response.get("RoleId").is_some());
    }
}
