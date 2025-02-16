use crate::client::AliyunClient;
use serde_json::Value;
use std::collections::BTreeMap;
use std::error::Error;

/// Describe Regions - 查询地域列表
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: This API queries the list of available regions for ECS.
///
/// **Input Parameters:**
/// | Parameter | Type    | Description                                     |
/// |-----------|---------|-------------------------------------------------|
/// | Action    | String  | Fixed value "DescribeRegions"                   |
/// | Format    | String  | Fixed value "JSON"                              |
/// | Version   | String  | Fixed value "2014-05-26"                        |
/// | RegionId  | String  | Optional, specify a region to filter results    |
///
/// **Output Parameters:**
/// | Field     | Type   | Description                                        |
/// |-----------|--------|----------------------------------------------------|
/// | Regions   | Array  | List of available regions                          |
/// | RequestId | String | Unique request ID                                  |
pub async fn describe_regions(
    client: &AliyunClient,
    region_id: Option<&str>,
) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert("Action".to_string(), "DescribeRegions".to_string());
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    if let Some(r) = region_id {
        params.insert("RegionId".to_string(), r.to_string());
    }
    client.send_request("ecs.aliyuncs.com", params).await
}

/// Describe Zones - 查询可用区列表
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: This API queries the list of available zones within a specified region.
///
/// **Input Parameters:**
/// | Parameter | Type    | Description                             |
/// |-----------|---------|-----------------------------------------|
/// | Action    | String  | Fixed value "DescribeZones"             |
/// | Format    | String  | Fixed value "JSON"                      |
/// | Version   | String  | Fixed value "2014-05-26"                |
/// | RegionId  | String  | Required, region ID                     |
///
/// **Output Parameters:**
/// | Field     | Type   | Description                                  |
/// |-----------|--------|----------------------------------------------|
/// | Zones     | Array  | List of available zones                      |
/// | RequestId | String | Unique request ID                            |
pub async fn describe_zones(
    client: &AliyunClient,
    region_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert("Action".to_string(), "DescribeZones".to_string());
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    params.insert("RegionId".to_string(), region_id.to_string());
    client.send_request("ecs.aliyuncs.com", params).await
}

/// Describe Available Resource - 查询可用区的资源库存
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: This API queries the resource inventory in a specified zone.
///
/// **Input Parameters:**
/// | Parameter | Type    | Description                            |
/// |-----------|---------|----------------------------------------|
/// | Action    | String  | Fixed value "DescribeAvailableResource"|
/// | Format    | String  | Fixed value "JSON"                     |
/// | Version   | String  | Fixed value "2014-05-26"               |
/// | RegionId  | String  | Required, region ID                    |
/// | ZoneId    | String  | Required, zone ID                      |
///
/// **Output Parameters:**
/// | Field                | Type   | Description                     |
/// |----------------------|--------|---------------------------------|
/// | AvailableResources   | Array  | Resource inventory details      |
/// | RequestId            | String | Unique request ID               |
pub async fn describe_available_resource(
    client: &AliyunClient,
    region_id: &str,
    zone_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert(
        "Action".to_string(),
        "DescribeAvailableResource".to_string(),
    );
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    params.insert("RegionId".to_string(), region_id.to_string());
    params.insert("ZoneId".to_string(), zone_id.to_string());
    client.send_request("ecs.aliyuncs.com", params).await
}

/// Describe Account Attributes - 查询资源配额
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: This API queries the ECS resource quotas for the account.
///
/// **Input Parameters:**
/// | Parameter | Type    | Description                          |
/// |-----------|---------|--------------------------------------|
/// | Action    | String  | Fixed value "DescribeAccountAttributes" |
/// | Format    | String  | Fixed value "JSON"                   |
/// | Version   | String  | Fixed value "2014-05-26"             |
///
/// **Output Parameters:**
/// | Field                | Type    | Description                |
/// |----------------------|---------|----------------------------|
/// | AccountAttributes    | Object  | ECS resource quota details |
/// | RequestId            | String  | Unique request ID          |
pub async fn describe_account_attributes(client: &AliyunClient) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert(
        "Action".to_string(),
        "DescribeAccountAttributes".to_string(),
    );
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    client.send_request("ecs.aliyuncs.com", params).await
}

/// Describe Resources Modification - 查询实例规格变更信息
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: Before changing instance type or system disk type, this API queries the available modification options in a zone.
///
/// **Input Parameters:**
/// | Parameter | Type    | Description                            |
/// |-----------|---------|----------------------------------------|
/// | Action    | String  | Fixed value "DescribeResourcesModification" |
/// | Format    | String  | Fixed value "JSON"                     |
/// | Version   | String  | Fixed value "2014-05-26"               |
/// | RegionId  | String  | Required, region ID                    |
/// | ZoneId    | String  | Required, zone ID                      |
///
/// **Output Parameters:**
/// | Field           | Type   | Description                      |
/// |-----------------|--------|----------------------------------|
/// | Modifications   | Array  | List of modification details   |
/// | RequestId       | String | Unique request ID                |
pub async fn describe_resources_modification(
    client: &AliyunClient,
    region_id: &str,
    zone_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert(
        "Action".to_string(),
        "DescribeResourcesModification".to_string(),
    );
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    params.insert("RegionId".to_string(), region_id.to_string());
    params.insert("ZoneId".to_string(), zone_id.to_string());
    client.send_request("ecs.aliyuncs.com", params).await
}

/// Describe Recommend Instance Type - 查询推荐实例规格
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: This API queries recommended instance types for pricing inquiries or usage recommendations.
///
/// **Input Parameters:**
/// | Parameter | Type    | Description                         |
/// |-----------|---------|-------------------------------------|
/// | Action    | String  | Fixed value "DescribeRecommendInstanceType" |
/// | Format    | String  | Fixed value "JSON"                  |
/// | Version   | String  | Fixed value "2014-05-26"            |
/// | RegionId  | String  | Required, region ID                 |
///
/// **Output Parameters:**
/// | Field          | Type   | Description                       |
/// |----------------|--------|-----------------------------------|
/// | InstanceTypes  | Array  | List of recommended instance types |
/// | RequestId      | String | Unique request ID                 |
pub async fn describe_recommend_instance_type(
    client: &AliyunClient,
    region_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert(
        "Action".to_string(),
        "DescribeRecommendInstanceType".to_string(),
    );
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    params.insert("RegionId".to_string(), region_id.to_string());
    client.send_request("ecs.aliyuncs.com", params).await
}

/// Run Instances - 批量创建实例
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: This API is used to create one or more ECS instances.
///
/// **Input Parameters:**
/// | Parameter     | Type    | Description                                    |
/// |---------------|---------|------------------------------------------------|
/// | Action        | String  | Fixed value "RunInstances"                     |
/// | Format        | String  | Fixed value "JSON"                             |
/// | Version       | String  | Fixed value "2014-05-26"                       |
/// | RegionId      | String  | Required, region ID                            |
/// | ImageId       | String  | Required, image ID                             |
/// | InstanceType  | String  | Required, instance type                        |
/// | ...           | ...     | Other parameters as needed                     |
///
/// **Output Parameters:**
/// | Field        | Type   | Description                                    |
/// |--------------|--------|------------------------------------------------|
/// | InstanceIds  | Array  | List of created instance IDs                   |
/// | RequestId    | String | Unique request ID                              |
pub async fn run_instances(
    client: &AliyunClient,
    region_id: &str,
    image_id: &str,
    instance_type: &str,
) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert("Action".to_string(), "RunInstances".to_string());
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    params.insert("RegionId".to_string(), region_id.to_string());
    params.insert("ImageId".to_string(), image_id.to_string());
    params.insert("InstanceType".to_string(), instance_type.to_string());
    // Add other required parameters as needed.
    client.send_request("ecs.aliyuncs.com", params).await
}

/// Start Instances - 启动实例
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: This API is used to start one or more stopped ECS instances.
///
/// **Input Parameters:**
/// | Parameter     | Type    | Description                                    |
/// |---------------|---------|------------------------------------------------|
/// | Action        | String  | Fixed value "StartInstances"                   |
/// | Format        | String  | Fixed value "JSON"                             |
/// | Version       | String  | Fixed value "2014-05-26"                       |
/// | InstanceIds   | String  | Required, list of instance IDs in JSON array format |
///
/// **Output Parameters:**
/// | Field      | Type   | Description                                    |
/// |------------|--------|------------------------------------------------|
/// | RequestId  | String | Unique request ID                              |
pub async fn start_instances(
    client: &AliyunClient,
    instance_ids: Vec<&str>,
) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert("Action".to_string(), "StartInstances".to_string());
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    let ids = format!(
        "[{}]",
        instance_ids
            .iter()
            .map(|id| format!("\"{}\"", id))
            .collect::<Vec<String>>()
            .join(",")
    );
    params.insert("InstanceIds".to_string(), ids);
    client.send_request("ecs.aliyuncs.com", params).await
}

/// Stop Instances - 停止实例
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: This API is used to stop running ECS instances.
///
/// **Input Parameters:**
/// | Parameter     | Type    | Description                                    |
/// |---------------|---------|------------------------------------------------|
/// | Action        | String  | Fixed value "StopInstances"                    |
/// | Format        | String  | Fixed value "JSON"                             |
/// | Version       | String  | Fixed value "2014-05-26"                       |
/// | InstanceIds   | String  | Required, list of instance IDs in JSON array format |
/// | ForceStop     | Boolean | Optional, whether to force stop, default false |
/// | DryRun        | Boolean | Optional, whether to perform a dry run         |
///
/// **Output Parameters:**
/// | Field      | Type   | Description                                    |
/// |------------|--------|------------------------------------------------|
/// | RequestId  | String | Unique request ID                              |
pub async fn stop_instances(
    client: &AliyunClient,
    instance_ids: Vec<&str>,
    force_stop: Option<bool>,
    dry_run: Option<bool>,
) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert("Action".to_string(), "StopInstances".to_string());
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    let ids = format!(
        "[{}]",
        instance_ids
            .iter()
            .map(|id| format!("\"{}\"", id))
            .collect::<Vec<String>>()
            .join(",")
    );
    params.insert("InstanceIds".to_string(), ids);
    if let Some(fs) = force_stop {
        params.insert("ForceStop".to_string(), fs.to_string());
    }
    if let Some(dr) = dry_run {
        params.insert("DryRun".to_string(), dr.to_string());
    }
    client.send_request("ecs.aliyuncs.com", params).await
}

/// Reboot Instance - 重启实例
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: This API is used to reboot a running ECS instance. It is asynchronous; after calling,
///   the instance status will first become "Starting", and when it becomes "Running", the reboot is successful.
///
/// **Input Parameters:**
/// | Parameter    | Type    | Description                                    |
/// |--------------|---------|------------------------------------------------|
/// | Action       | String  | Fixed value "RebootInstance"                   |
/// | Format       | String  | Fixed value "JSON"                             |
/// | Version      | String  | Fixed value "2014-05-26"                       |
/// | InstanceId   | String  | Required, the instance ID                      |
/// | ForceStop    | Boolean | Optional, whether to force stop before reboot, default false |
/// | DryRun       | Boolean | Optional, whether to perform a dry run         |
///
/// **Output Parameters:**
/// | Field      | Type   | Description                                    |
/// |------------|--------|------------------------------------------------|
/// | RequestId  | String | Unique request ID                              |
pub async fn reboot_instance(
    client: &AliyunClient,
    instance_id: &str,
    force_stop: Option<bool>,
    dry_run: Option<bool>,
) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert("Action".to_string(), "RebootInstance".to_string());
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    params.insert("InstanceId".to_string(), instance_id.to_string());
    if let Some(fs) = force_stop {
        params.insert("ForceStop".to_string(), fs.to_string());
    }
    if let Some(dr) = dry_run {
        params.insert("DryRun".to_string(), dr.to_string());
    }
    client.send_request("ecs.aliyuncs.com", params).await
}

/// Delete Instance - 删除实例
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: This API is used to delete a specified ECS instance. Once deleted, the instance cannot be recovered.
///
/// **Input Parameters:**
/// | Parameter    | Type    | Description                                    |
/// |--------------|---------|------------------------------------------------|
/// | Action       | String  | Fixed value "DeleteInstance"                   |
/// | Format       | String  | Fixed value "JSON"                             |
/// | Version      | String  | Fixed value "2014-05-26"                       |
/// | InstanceId   | String  | Required, the instance ID                      |
///
/// **Output Parameters:**
/// | Field      | Type   | Description                                    |
/// |------------|--------|------------------------------------------------|
/// | RequestId  | String | Unique request ID                              |
pub async fn delete_instance(
    client: &AliyunClient,
    instance_id: &str,
) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert("Action".to_string(), "DeleteInstance".to_string());
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    params.insert("InstanceId".to_string(), instance_id.to_string());
    client.send_request("ecs.aliyuncs.com", params).await
}

/// Describe Instance Status - 查询实例状态信息列表
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: This API queries the status information of one or more ECS instances.
///
/// **Input Parameters:**
/// | Parameter     | Type     | Description                                        |
/// |---------------|----------|----------------------------------------------------|
/// | Action        | String   | Fixed value "DescribeInstanceStatus"               |
/// | Format        | String   | Fixed value "JSON"                                 |
/// | Version       | String   | Fixed value "2014-05-26"                           |
/// | RegionId      | String   | Required, the region ID                            |
/// | InstanceId    | String   | Optional, specify instance ID(s) in JSON array format |
/// | PageNumber    | Integer  | Optional, page number                              |
/// | PageSize      | Integer  | Optional, page size                                |
///
/// **Output Parameters:**
/// | Field                | Type    | Description                                    |
/// |----------------------|---------|------------------------------------------------|
/// | InstanceStatusSet    | Array   | List of instance status information            |
/// | RequestId            | String  | Unique request ID                              |
pub async fn describe_instance_status(
    client: &AliyunClient,
    region_id: &str,
    instance_id: Option<&str>,
    page_number: Option<u32>,
    page_size: Option<u32>,
) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert("Action".to_string(), "DescribeInstanceStatus".to_string());
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    params.insert("RegionId".to_string(), region_id.to_string());
    if let Some(id) = instance_id {
        params.insert("InstanceId".to_string(), id.to_string());
    }
    if let Some(page) = page_number {
        params.insert("PageNumber".to_string(), page.to_string());
    }
    if let Some(size) = page_size {
        params.insert("PageSize".to_string(), size.to_string());
    }
    client.send_request("ecs.aliyuncs.com", params).await
}

/// Describe Instances - 查询实例详细信息列表
///
/// **API Description:**
/// - Request Domain: ecs.aliyuncs.com
/// - API Version: 2014-05-26
/// - Description: This API queries detailed information of ECS instances with filtering support.
///
/// **Input Parameters:**
/// | Parameter     | Type    | Description                                        |
/// |---------------|---------|----------------------------------------------------|
/// | Action        | String  | Fixed value "DescribeInstances"                    |
/// | Format        | String  | Fixed value "JSON"                                 |
/// | Version       | String  | Fixed value "2014-05-26"                           |
/// | RegionId      | String  | Required, the region ID                            |
/// | Filters       | String  | Optional, filtering conditions in JSON string format |
/// | PageNumber    | Integer | Optional, page number                              |
/// | PageSize      | Integer | Optional, page size                                |
///
/// **Output Parameters:**
/// | Field      | Type    | Description                                    |
/// |------------|---------|------------------------------------------------|
/// | Instances  | Array   | List of instance detailed information           |
/// | TotalCount | Integer | Total number of instances                        |
/// | RequestId  | String  | Unique request ID                                |
pub async fn describe_instances(
    client: &AliyunClient,
    region_id: &str,
    filters: Option<&str>,
    page_number: Option<u32>,
    page_size: Option<u32>,
) -> Result<Value, Box<dyn Error>> {
    let mut params = BTreeMap::new();
    params.insert("Action".to_string(), "DescribeInstances".to_string());
    params.insert("Format".to_string(), "JSON".to_string());
    params.insert("Version".to_string(), "2014-05-26".to_string());
    params.insert("RegionId".to_string(), region_id.to_string());
    if let Some(f) = filters {
        params.insert("Filters".to_string(), f.to_string());
    }
    if let Some(page) = page_number {
        params.insert("PageNumber".to_string(), page.to_string());
    }
    if let Some(size) = page_size {
        params.insert("PageSize".to_string(), size.to_string());
    }
    client.send_request("ecs.aliyuncs.com", params).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::AliyunClient;
    use tokio;

    const TEST_ACCESS_KEY_ID: &str = "access_key_id";
    const TEST_ACCESS_KEY_SECRET: &str = "access_key_secret";
    const TEST_REGION: &str = "cn-hangzhou";
    const TEST_ZONE: &str = "cn-hangzhou-d";
    const TEST_INSTANCE_ID: &str = "YourInstanceId";

    #[tokio::test]
    async fn test_describe_regions() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let result = describe_regions(&client, Some(TEST_REGION)).await;
        println!("describe_regions: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_describe_zones() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let result = describe_zones(&client, TEST_REGION).await;
        println!("describe_zones: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_describe_available_resource() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let result = describe_available_resource(&client, TEST_REGION, TEST_ZONE).await;
        println!("describe_available_resource: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_describe_account_attributes() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let result = describe_account_attributes(&client).await;
        println!("describe_account_attributes: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_describe_resources_modification() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let result = describe_resources_modification(&client, TEST_REGION, TEST_ZONE).await;
        println!("describe_resources_modification: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_describe_recommend_instance_type() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let result = describe_recommend_instance_type(&client, TEST_REGION).await;
        println!("describe_recommend_instance_type: {:?}", result);
        assert!(result.is_ok());
    }

    // 以下接口可能会产生实例，故测试时建议谨慎操作，已添加 #[ignore] 属性

    #[tokio::test]
    #[ignore]
    async fn test_run_instances() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let image_id = "YourImageId";
        let instance_type = "ecs.g5.large";
        let result = run_instances(&client, TEST_REGION, image_id, instance_type).await;
        println!("run_instances: {:?}", result);
        // assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_start_instances() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let instance_ids = vec![TEST_INSTANCE_ID];
        let result = start_instances(&client, instance_ids).await;
        println!("start_instances: {:?}", result);
        // assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_stop_instances() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let instance_ids = vec![TEST_INSTANCE_ID];
        let result = stop_instances(&client, instance_ids, Some(false), Some(true)).await;
        println!("stop_instances: {:?}", result);
        // assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_reboot_instance() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let result = reboot_instance(&client, TEST_INSTANCE_ID, Some(false), Some(false)).await;
        println!("reboot_instance: {:?}", result);
        // assert!(result.is_ok());
    }

    #[tokio::test]
    #[ignore]
    async fn test_delete_instance() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let result = delete_instance(&client, TEST_INSTANCE_ID).await;
        println!("delete_instance: {:?}", result);
        // assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_instance_status() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let result = describe_instance_status(&client, TEST_REGION, None, Some(1), Some(10)).await;
        println!("describe_instance_status: {:?}", result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_describe_instances() {
        let client = AliyunClient::new(TEST_ACCESS_KEY_ID.into(), TEST_ACCESS_KEY_SECRET.into());
        let result = describe_instances(&client, TEST_REGION, None, Some(1), Some(10)).await;
        println!("describe_instances: {:?}", result);
        assert!(result.is_ok());
    }
}
