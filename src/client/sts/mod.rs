pub mod caller_identity;

use crate::{
    client::{
        error::AdvancedClientError, sts::caller_identity::CallerIdentityBody,
        utils::parse_json_value, AliyunClient,
    },
    services::sts::get_caller_identity,
};

/// A thin, non-owning client for STS (Security Token Service) operations.
///
/// `STSClient` holds a borrowed reference to an `AliyunClient` and exposes
/// STS-specific API methods. Because it borrows the parent client it does not
/// clone or take ownership of the underlying client, keeping calls lightweight
/// and suitable for method chaining.
pub struct STSClient<'a> {
    pub client: &'a AliyunClient,
}

impl<'a> STSClient<'a> {
    /// Create a new `STSClient` that borrows the provided `AliyunClient`.
    pub fn new(client: &'a AliyunClient) -> Self {
        Self { client }
    }

    /// Call the STS `GetCallerIdentity` operation and return the deserialized
    /// `CallerIdentityBody` on success.
    ///
    /// Errors from the HTTP client or JSON deserialization are propagated as
    /// `AdvancedClientError`.
    pub async fn get_caller_identity(&self) -> Result<CallerIdentityBody, AdvancedClientError> {
        let response = get_caller_identity(&self.client).await?;
        let parsed = parse_json_value::<CallerIdentityBody>(response)?;
        Result::Ok(parsed)
    }
}

impl AliyunClient {
    /// Obtain an `STSClient` that borrows this `AliyunClient`.
    ///
    /// This convenience method enables ergonomic method chaining like
    /// `client.sts().get_caller_identity().await` without taking ownership of
    /// the parent client.
    pub fn sts<'a>(&'a self) -> STSClient<'a> {
        STSClient { client: self }
    }
}

#[cfg(test)]
mod tests {
    use claims::assert_matches;

    use crate::{
        client::error::AdvancedClientError,
        test_multiple_clients,
        test_utils::{create_aliyun_client, EMPTY, GLOBAL_TEST_SECRETS, INVALID},
    };

    #[tokio::test]
    async fn test_sts_chain_calling() {
        // Use method chaining to simplify calls to the STS service.
        // The `sts()` method returns a lightweight wrapper that holds a reference to
        // the parent `AliyunClient`, avoiding clones and introducing minimal overhead.
        let result_body = create_aliyun_client::<GLOBAL_TEST_SECRETS>()
            .sts()
            .get_caller_identity()
            .await
            .unwrap();
        println!("{:?}", result_body);
    }

    #[tokio::test]
    async fn test_get_caller_identity() {
        test_multiple_clients! {
            [EMPTY => "EMPTY", INVALID => "INVALID"],
            |client, name| async {
                let result = client.sts().get_caller_identity().await;
                println!("{} Result: {:#?}", name, result);
                assert_matches!(result, Err(AdvancedClientError::AliyunRejectError(_)));
            }
        }
    }
}
