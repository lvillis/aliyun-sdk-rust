//! Request/response types.

pub mod billing;
pub mod ecs;
pub mod sts;

mod common;

pub use common::{InstanceId, RegionId, ZoneId};
