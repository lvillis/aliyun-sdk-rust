use crate::{error::Error, types::billing::QueryAccountBalanceParams};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

const VERSION: &str = "2017-12-14";

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct BillingService {
    client: Client,
}

#[cfg(feature = "async")]
impl BillingService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn query_account_balance(
        &self,
        params: QueryAccountBalanceParams,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .rpc_json(
                self.client.endpoint_billing(),
                "QueryAccountBalance",
                VERSION,
                params.into_query(),
            )
            .await
    }
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingBillingService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingBillingService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn query_account_balance(
        &self,
        params: QueryAccountBalanceParams,
    ) -> Result<serde_json::Value, Error> {
        self.client.rpc_json(
            self.client.endpoint_billing(),
            "QueryAccountBalance",
            VERSION,
            params.into_query(),
        )
    }
}
