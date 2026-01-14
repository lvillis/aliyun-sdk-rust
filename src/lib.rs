//! Alibaba Cloud (Aliyun) RPC API SDK.
//!
//! ## Quick start (async)
//! ```no_run
//! # #[cfg(feature = "async")]
//! use alibabacloud::{Auth, Client};
//!
//! # #[cfg(feature = "async")]
//! # async fn demo() -> Result<(), alibabacloud::Error> {
//! let client = Client::builder()
//!     .auth(Auth::access_key("ACCESS_KEY_ID", "ACCESS_KEY_SECRET"))
//!     .build()?;
//!
//! let regions = client.ecs().describe_regions(Default::default()).await?;
//! println!("{regions:#?}");
//! # Ok(())
//! # }
//! ```
//!
//! ## Quick start (blocking)
//! ```no_run
//! # #[cfg(feature = "blocking")]
//! # fn demo() -> Result<(), alibabacloud::Error> {
//! use alibabacloud::{Auth, BlockingClient};
//!
//! let client = BlockingClient::builder()
//!     .auth(Auth::access_key("ACCESS_KEY_ID", "ACCESS_KEY_SECRET"))
//!     .build()?;
//!
//! let regions = client.ecs().describe_regions(Default::default())?;
//! println!("{regions:#?}");
//! # Ok(())
//! # }
//! ```

#![forbid(unsafe_code)]
#![cfg_attr(
    not(test),
    deny(
        clippy::panic,
        clippy::unwrap_used,
        clippy::expect_used,
        clippy::todo,
        clippy::unimplemented,
        clippy::dbg_macro,
    )
)]

#[cfg(not(any(feature = "async", feature = "blocking")))]
compile_error!("Enable at least one of: `async`, `blocking`");

#[cfg(all(feature = "rustls", feature = "native-tls"))]
compile_error!("Enable only one TLS backend: `rustls` or `native-tls`");

#[cfg(all(
    any(feature = "async", feature = "blocking"),
    not(any(feature = "rustls", feature = "native-tls"))
))]
compile_error!("Enable one TLS backend: `native-tls` (default) or `rustls`");

mod transport;
mod util;

pub mod api;
pub mod auth;
pub mod client;
pub mod error;
pub mod types;

pub use auth::Auth;
#[cfg(feature = "blocking")]
pub use client::BlockingClient;
#[cfg(feature = "async")]
pub use client::Client;
pub use error::Error;

pub type Result<T> = std::result::Result<T, Error>;
