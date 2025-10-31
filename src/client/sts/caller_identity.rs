use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum IdentityType {
    Account,
    RAMUser,
    AssumedRoleUser,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CallerIdentityBody {
    pub identity_type: IdentityType,
    pub request_id: String,
    pub account_id: String,
    pub principal_id: String,
    pub user_id: String,
    pub arn: String,
    pub role_id: Option<String>,
}
