//! Service entry points.

mod billing;
mod ecs;
mod sts;

#[cfg(feature = "async")]
pub use billing::BillingService;
#[cfg(feature = "blocking")]
pub use billing::BlockingBillingService;

#[cfg(feature = "blocking")]
pub use ecs::BlockingEcsService;
#[cfg(feature = "async")]
pub use ecs::EcsService;

#[cfg(feature = "blocking")]
pub use sts::BlockingStsService;
#[cfg(feature = "async")]
pub use sts::StsService;
