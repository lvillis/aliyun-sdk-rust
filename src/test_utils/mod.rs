pub mod error;

pub use error::TestSecretsError;

use once_cell::sync::Lazy;
use std::env;

use crate::client::AliyunClient;

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

        let access_key_id = env::var("TEST_ACCESS_KEY_ID").map_err(|e| TestSecretsError {
            var_name: "TEST_ACCESS_KEY_ID".to_string(),
            source: e,
        })?;
        let access_key_secret = env::var("TEST_ACCESS_KEY_SECRET").map_err(|e| TestSecretsError {
            var_name: "TEST_ACCESS_KEY_SECRET".to_string(),
            source: e,
        })?;

        Ok(TestSecrets {
            access_key_id,
            access_key_secret,
        })
    }
}

pub static TEST_SECRETS: Lazy<TestSecrets> =
    Lazy::new(|| TestSecrets::from_env().expect("Failed to load test secrets from environment"));

#[allow(non_camel_case_types)]
pub enum AliyunClientCreationVariant {
    GLOBAL_TEST_SECRETS,
    EMPTY,
}

pub trait ClientCredentialProvider {
    fn get_credentials() -> (String, String);
}

#[allow(non_camel_case_types)]
pub struct GLOBAL_TEST_SECRETS;
#[allow(non_camel_case_types)]
pub struct EMPTY;
#[allow(non_camel_case_types)]
pub struct INVALID;

impl ClientCredentialProvider for GLOBAL_TEST_SECRETS {
    fn get_credentials() -> (String, String) {
        (
            TEST_SECRETS.access_key_id.clone(),
            TEST_SECRETS.access_key_secret.clone(),
        )
    }
}

impl ClientCredentialProvider for EMPTY {
    fn get_credentials() -> (String, String) {
        ("".to_owned(), "".to_owned())
    }
}

impl ClientCredentialProvider for INVALID {
    fn get_credentials() -> (String, String) {
        (
            "INVALID-ACCESS-TOKEN".to_owned(),
            "INVALID-ACCESS_SECRET".to_owned(),
        )
    }
}

pub fn create_aliyun_client<P: ClientCredentialProvider>() -> AliyunClient {
    let credentials = P::get_credentials();
    AliyunClient::new(credentials.0, credentials.1)
}

/// Macro to test multiple credential providers that should fail.
/// 
/// # Example
/// ```ignore
/// use crate::test_utils::{EMPTY, INVALID};
/// 
/// test_invalid_clients! {
///     [EMPTY => "EMPTY", INVALID => "INVALID"],
///     |client, name| async {
///         println!("Testing {} credentials...", name);
///         let result = client.sts().get_caller_identity().await;
///         assert_matches!(result, Err(AdvancedClientError::AliyunRejectError(_)));
///     }
/// }
/// ```
#[macro_export]
macro_rules! test_multiple_clients {
    ([$($provider:ty => $name:expr),* $(,)?], |$client:ident, $name_var:ident| $test_fn:expr) => {
        $(
            {
                let $client = $crate::test_utils::create_aliyun_client::<$provider>();
                let $name_var = $name;
                let test_closure = $test_fn;
                test_closure.await;
            }
        )*
    };
}
