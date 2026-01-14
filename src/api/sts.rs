use std::collections::BTreeMap;

use crate::{error::Error, types::sts::CallerIdentity};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

const VERSION: &str = "2015-04-01";

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct StsService {
    client: Client,
}

#[cfg(feature = "async")]
impl StsService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    pub async fn get_caller_identity(&self) -> Result<CallerIdentity, Error> {
        self.client
            .rpc_json(
                self.client.endpoint_sts(),
                "GetCallerIdentity",
                VERSION,
                BTreeMap::new(),
            )
            .await
    }
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingStsService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingStsService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    pub fn get_caller_identity(&self) -> Result<CallerIdentity, Error> {
        self.client.rpc_json(
            self.client.endpoint_sts(),
            "GetCallerIdentity",
            VERSION,
            BTreeMap::new(),
        )
    }
}
