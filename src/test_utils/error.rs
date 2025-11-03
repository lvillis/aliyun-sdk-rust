use std::env;
use thiserror::Error;

/// Error type for test secrets loading failures
#[derive(Debug, Error)]
#[error("Failed to get test credential from environment variable '{var_name}': {source}")]
pub struct TestSecretsError {
    pub var_name: String,
    #[source]
    pub source: env::VarError,
}
