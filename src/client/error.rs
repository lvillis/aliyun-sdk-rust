use std::error::Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AdvancedClientError {
    #[error("When using the underlying client to send request it throw an error: {0}")]
    UnderlyingError(
        #[source]
        #[from]
        Box<dyn Error>,
    ),
    #[error("When trying to deserialization the result an error occurred")]
    ResultDeserializationError(#[from] serde_json::Error),
}
