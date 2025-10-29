use std::env;

/// Error type for test secrets loading failures
#[derive(Debug)]
pub enum TestSecretsError {
    AccessKeyId(env::VarError),
    AccessKeySecret(env::VarError),
}

impl std::fmt::Display for TestSecretsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TestSecretsError::AccessKeyId(e) => {
                write!(f, "Failed to get TEST_ACCESS_KEY_ID: {}", e)
            }
            TestSecretsError::AccessKeySecret(e) => {
                write!(f, "Failed to get TEST_ACCESS_KEY_SECRET: {}", e)
            }
        }
    }
}

impl std::error::Error for TestSecretsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            TestSecretsError::AccessKeyId(e) => Some(e),
            TestSecretsError::AccessKeySecret(e) => Some(e),
        }
    }
}
