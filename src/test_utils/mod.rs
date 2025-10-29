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
        // It is recommended to store test credentials in a `.env` file at the project root
        // for local development. Environment variables set in `.cargo/config.toml`
        // will override corresponding values from the `.env` file.
        //
        // Note: some IDEs and their debuggers may not load environment variables from
        // `.cargo/config.toml`, which can lead to different behavior when running or
        // debugging tests inside the IDE.
        let _ = dotenv::dotenv();

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
