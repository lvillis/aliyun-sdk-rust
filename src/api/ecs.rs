use std::collections::BTreeMap;

use crate::{
    error::Error,
    types::ecs::{
        DeleteInstanceParams, DescribeAccountAttributesParams, DescribeAvailableResourceParams,
        DescribeInstanceStatusParams, DescribeInstancesParams, DescribeRecommendInstanceTypeParams,
        DescribeRegionsParams, DescribeResourcesModificationParams, DescribeZonesParams,
        RebootInstanceParams, RunInstancesParams, StartInstancesParams, StopInstancesParams,
    },
};

#[cfg(feature = "blocking")]
use crate::client::BlockingClient;
#[cfg(feature = "async")]
use crate::client::Client;

const VERSION: &str = "2014-05-26";

#[cfg(feature = "async")]
#[derive(Clone)]
pub struct EcsService {
    client: Client,
}

#[cfg(feature = "async")]
impl EcsService {
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    async fn rpc_json_value(
        &self,
        action: &'static str,
        params: BTreeMap<String, String>,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .rpc_json(self.client.endpoint_ecs(), action, VERSION, params)
            .await
    }

    pub async fn describe_regions(
        &self,
        params: DescribeRegionsParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeRegions", params.into_query())
            .await
    }

    pub async fn describe_zones(
        &self,
        params: DescribeZonesParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeZones", params.into_query())
            .await
    }

    pub async fn describe_available_resource(
        &self,
        params: DescribeAvailableResourceParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeAvailableResource", params.into_query())
            .await
    }

    pub async fn describe_account_attributes(
        &self,
        params: DescribeAccountAttributesParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeAccountAttributes", params.into_query())
            .await
    }

    pub async fn describe_resources_modification(
        &self,
        params: DescribeResourcesModificationParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeResourcesModification", params.into_query())
            .await
    }

    pub async fn describe_recommend_instance_type(
        &self,
        params: DescribeRecommendInstanceTypeParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeRecommendInstanceType", params.into_query())
            .await
    }

    pub async fn run_instances(
        &self,
        params: RunInstancesParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("RunInstances", params.into_query())
            .await
    }

    pub async fn start_instances(
        &self,
        params: StartInstancesParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("StartInstances", params.into_query())
            .await
    }

    pub async fn stop_instances(
        &self,
        params: StopInstancesParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("StopInstances", params.into_query())
            .await
    }

    pub async fn reboot_instance(
        &self,
        params: RebootInstanceParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("RebootInstance", params.into_query())
            .await
    }

    pub async fn delete_instance(
        &self,
        params: DeleteInstanceParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DeleteInstance", params.into_query())
            .await
    }

    pub async fn describe_instance_status(
        &self,
        params: DescribeInstanceStatusParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeInstanceStatus", params.into_query())
            .await
    }

    pub async fn describe_instances(
        &self,
        params: DescribeInstancesParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeInstances", params.into_query())
            .await
    }
}

#[cfg(feature = "blocking")]
#[derive(Clone)]
pub struct BlockingEcsService {
    client: BlockingClient,
}

#[cfg(feature = "blocking")]
impl BlockingEcsService {
    pub(crate) fn new(client: BlockingClient) -> Self {
        Self { client }
    }

    fn rpc_json_value(
        &self,
        action: &'static str,
        params: BTreeMap<String, String>,
    ) -> Result<serde_json::Value, Error> {
        self.client
            .rpc_json(self.client.endpoint_ecs(), action, VERSION, params)
    }

    pub fn describe_regions(
        &self,
        params: DescribeRegionsParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeRegions", params.into_query())
    }

    pub fn describe_zones(&self, params: DescribeZonesParams) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeZones", params.into_query())
    }

    pub fn describe_available_resource(
        &self,
        params: DescribeAvailableResourceParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeAvailableResource", params.into_query())
    }

    pub fn describe_account_attributes(
        &self,
        params: DescribeAccountAttributesParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeAccountAttributes", params.into_query())
    }

    pub fn describe_resources_modification(
        &self,
        params: DescribeResourcesModificationParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeResourcesModification", params.into_query())
    }

    pub fn describe_recommend_instance_type(
        &self,
        params: DescribeRecommendInstanceTypeParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeRecommendInstanceType", params.into_query())
    }

    pub fn run_instances(&self, params: RunInstancesParams) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("RunInstances", params.into_query())
    }

    pub fn start_instances(
        &self,
        params: StartInstancesParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("StartInstances", params.into_query())
    }

    pub fn stop_instances(&self, params: StopInstancesParams) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("StopInstances", params.into_query())
    }

    pub fn reboot_instance(
        &self,
        params: RebootInstanceParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("RebootInstance", params.into_query())
    }

    pub fn delete_instance(
        &self,
        params: DeleteInstanceParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DeleteInstance", params.into_query())
    }

    pub fn describe_instance_status(
        &self,
        params: DescribeInstanceStatusParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeInstanceStatus", params.into_query())
    }

    pub fn describe_instances(
        &self,
        params: DescribeInstancesParams,
    ) -> Result<serde_json::Value, Error> {
        self.rpc_json_value("DescribeInstances", params.into_query())
    }
}
