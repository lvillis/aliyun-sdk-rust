use std::error::Error;

use serde::{Deserialize, Serialize};
use thiserror::Error;

fn default_aliyun_rejection_field() -> String {
    "No content. If you see this please submit a bug issue (to the rust sdk repo).".to_string()
}

/// Represents an error response returned by the Aliyun server.
///
/// The response fields use PascalCase naming convention to match
/// the format returned by the Aliyun API and maintain consistency.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AliyunRejection {
    pub code: String,
    pub host_id: String,
    #[serde(default = "default_aliyun_rejection_field")]
    pub message: String,
    pub request_id: String,
    #[serde(default = "default_aliyun_rejection_field")]
    pub recommend: String,
}

#[derive(Error, Debug)]
pub enum AdvancedClientError {
    #[error("Aliyun rejected the request and returned an error")]
    AliyunRejectError(AliyunRejection),
    #[error("When using the underlying client to send request it throw an error: {0}")]
    UnderlyingError(
        #[source]
        #[from]
        Box<dyn Error>,
    ),
    #[error("When trying to deserialization the result an error occurred. This should not happened; please using services to debug and open a bug issue")]
    ResultDeserializationError(#[from] serde_json::Error),
}
