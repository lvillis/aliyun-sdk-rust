use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IdentityType {
    Account,
    #[serde(rename = "RAMUser")]
    RamUser,
    AssumedRoleUser,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CallerIdentity {
    pub identity_type: IdentityType,
    pub request_id: String,
    pub account_id: String,
    pub principal_id: String,
    pub user_id: String,
    pub arn: String,
    pub role_id: Option<String>,
}
