use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::types::{InstanceId, RegionId, ZoneId};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeRegionsParams {
    pub region_id: Option<RegionId>,
}

impl DescribeRegionsParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        if let Some(region_id) = self.region_id {
            map.insert("RegionId".to_owned(), region_id.to_string());
        }
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeZonesParams {
    pub region_id: RegionId,
}

impl DescribeZonesParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert("RegionId".to_owned(), self.region_id.to_string());
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeAvailableResourceParams {
    pub region_id: RegionId,
    pub zone_id: ZoneId,
}

impl DescribeAvailableResourceParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert("RegionId".to_owned(), self.region_id.to_string());
        map.insert("ZoneId".to_owned(), self.zone_id.to_string());
        map
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DescribeAccountAttributesParams {}

impl DescribeAccountAttributesParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        BTreeMap::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeResourcesModificationParams {
    pub region_id: RegionId,
    pub zone_id: ZoneId,
}

impl DescribeResourcesModificationParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert("RegionId".to_owned(), self.region_id.to_string());
        map.insert("ZoneId".to_owned(), self.zone_id.to_string());
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeRecommendInstanceTypeParams {
    pub region_id: RegionId,
}

impl DescribeRecommendInstanceTypeParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert("RegionId".to_owned(), self.region_id.to_string());
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunInstancesParams {
    pub region_id: RegionId,
    pub image_id: String,
    pub instance_type: String,
}

impl RunInstancesParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert("RegionId".to_owned(), self.region_id.to_string());
        map.insert("ImageId".to_owned(), self.image_id);
        map.insert("InstanceType".to_owned(), self.instance_type);
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartInstancesParams {
    pub instance_ids: Vec<InstanceId>,
}

impl StartInstancesParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert(
            "InstanceIds".to_owned(),
            instance_ids_json(self.instance_ids),
        );
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopInstancesParams {
    pub instance_ids: Vec<InstanceId>,
    pub force_stop: Option<bool>,
    pub dry_run: Option<bool>,
}

impl StopInstancesParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert(
            "InstanceIds".to_owned(),
            instance_ids_json(self.instance_ids),
        );
        if let Some(force_stop) = self.force_stop {
            map.insert("ForceStop".to_owned(), force_stop.to_string());
        }
        if let Some(dry_run) = self.dry_run {
            map.insert("DryRun".to_owned(), dry_run.to_string());
        }
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebootInstanceParams {
    pub instance_id: InstanceId,
    pub force_stop: Option<bool>,
    pub dry_run: Option<bool>,
}

impl RebootInstanceParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert("InstanceId".to_owned(), self.instance_id.to_string());
        if let Some(force_stop) = self.force_stop {
            map.insert("ForceStop".to_owned(), force_stop.to_string());
        }
        if let Some(dry_run) = self.dry_run {
            map.insert("DryRun".to_owned(), dry_run.to_string());
        }
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteInstanceParams {
    pub instance_id: InstanceId,
}

impl DeleteInstanceParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert("InstanceId".to_owned(), self.instance_id.to_string());
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeInstanceStatusParams {
    pub region_id: RegionId,
    pub instance_id: Option<InstanceId>,
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

impl DescribeInstanceStatusParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert("RegionId".to_owned(), self.region_id.to_string());
        if let Some(instance_id) = self.instance_id {
            map.insert("InstanceId".to_owned(), instance_id.to_string());
        }
        if let Some(page_number) = self.page_number {
            map.insert("PageNumber".to_owned(), page_number.to_string());
        }
        if let Some(page_size) = self.page_size {
            map.insert("PageSize".to_owned(), page_size.to_string());
        }
        map
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DescribeInstancesParams {
    pub region_id: RegionId,
    /// JSON string for the `Filters` parameter.
    pub filters: Option<String>,
    pub page_number: Option<u32>,
    pub page_size: Option<u32>,
}

impl DescribeInstancesParams {
    pub(crate) fn into_query(self) -> BTreeMap<String, String> {
        let mut map = BTreeMap::new();
        map.insert("RegionId".to_owned(), self.region_id.to_string());
        if let Some(filters) = self.filters {
            map.insert("Filters".to_owned(), filters);
        }
        if let Some(page_number) = self.page_number {
            map.insert("PageNumber".to_owned(), page_number.to_string());
        }
        if let Some(page_size) = self.page_size {
            map.insert("PageSize".to_owned(), page_size.to_string());
        }
        map
    }
}

fn instance_ids_json(instance_ids: Vec<InstanceId>) -> String {
    let ids = instance_ids
        .into_iter()
        .map(|id| id.to_string())
        .collect::<Vec<_>>();

    match serde_json::to_string(&ids) {
        Ok(json) => json,
        Err(_) => "[]".to_owned(),
    }
}
