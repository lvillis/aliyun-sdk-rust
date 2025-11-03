use std::fmt::Debug;

use serde::Deserialize;

use crate::client::error::{AdvancedClientError, AliyunRejection};

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum ResponseFromAliyun<R: Debug> {
    Normal(R),
    Rejected(AliyunRejection),
}

pub fn parse_json_value<R: Debug + for<'de> Deserialize<'de>>(
    value: serde_json::Value,
) -> Result<R, AdvancedClientError> {
    match serde_json::from_value::<ResponseFromAliyun<R>>(value)? {
        ResponseFromAliyun::Normal(result) => Ok(result),
        ResponseFromAliyun::Rejected(err) => Err(AdvancedClientError::AliyunRejectError(err)),
    }
}
