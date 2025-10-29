pub mod error;

pub use error::TestSecretsError;

use once_cell::sync::Lazy;
use std::env;

pub struct TestSecrets {
    pub access_key_id: String,
    pub access_key_secret: String,
}

impl TestSecrets {
    fn from_env() -> Result<Self, TestSecretsError> {
        let access_key_id =
            env::var("TEST_ACCESS_KEY_ID").map_err(TestSecretsError::AccessKeyId)?;
        let access_key_secret =
            env::var("TEST_ACCESS_KEY_SECRET").map_err(TestSecretsError::AccessKeySecret)?;

        Ok(TestSecrets {
            access_key_id,
            access_key_secret,
        })
    }
}

pub static TEST_SECRETS: Lazy<TestSecrets> =
    Lazy::new(|| TestSecrets::from_env().expect("Failed to load test secrets from environment"));
