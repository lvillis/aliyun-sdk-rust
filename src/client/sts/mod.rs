pub mod caller_identity;

use crate::{
    client::{error::AdvancedClientError, sts::caller_identity::CallerIdentityBody, AliyunClient},
    services::sts::get_caller_identity,
};

pub struct STSClient<'a> {
    pub client: &'a AliyunClient,
}

impl<'a> STSClient<'a> {
    pub fn new(client: &'a AliyunClient) -> Self {
        Self { client }
    }

    pub async fn get_caller_identity(&self) -> Result<CallerIdentityBody, AdvancedClientError> {
        let response = get_caller_identity(&self.client).await?;
        let deserialized = serde_json::from_value::<CallerIdentityBody>(response)?;
        Result::Ok(deserialized)
    }
}

impl AliyunClient {
    pub fn sts<'a>(&'a self) -> STSClient<'a> {
        STSClient { client: self }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::create_aliyun_client;

    #[tokio::test]
    async fn test_get_caller_identity() {
        let client = create_aliyun_client();
        let sts_client = client.sts();
        let result_body = sts_client.get_caller_identity().await.unwrap();
        println!("{:?}", result_body);

        // use chain calling
        let result_body = client
            .sts()
            .get_caller_identity()
            .await
            .expect("Chain calling failed");
        println!("{:?}", result_body);
    }
}
