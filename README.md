<div align=right>Table of Contents↗️</div>

<h1 align=center><code>aliyun-sdk-rust</code></h1>

<p align=center>📦 Aliyun API SDK written in Rust</p>

<div align=center>
  <a href="https://crates.io/crates/alibabacloud">
    <img src="https://img.shields.io/crates/v/alibabacloud.svg" alt="crates.io version">
  </a>
  <a href="https://crates.io/crates/alibabacloud">
    <img src="https://img.shields.io/github/repo-size/lvillis/aliyun-sdk-rust?style=flat-square&color=328657" alt="crates.io version">
  </a>
  <a href="https://github.com/lvillis/aliyun-sdk-rust/actions">
    <img src="https://github.com/lvillis/aliyun-sdk-rust/actions/workflows/ci.yaml/badge.svg" alt="build status">
  </a>
  <a href="mailto:lvillis@outlook.com?subject=Thanks%20for%20aliyun-sdk-rust!">
    <img src="https://img.shields.io/badge/Say%20Thanks-!-1EAEDB.svg" alt="say thanks">
  </a>

</div>

---

This project is an Aliyun RPC API SDK written in Rust, with a consistent async/blocking API surface and shared types + error handling.

## Features

- **Async + Blocking**: `Client` (async) and `BlockingClient` (feature=`blocking`) share the same `types` and `Error`.
- **TLS Backend Selection**: Choose exactly one of `native-tls` (default) or `rustls`.
- **Tracing**: Enable feature=`tracing` to emit per-request spans (method/host/path/status/latency/retry_count/request_id), without logging sensitive query strings.
- **Request Signing**: Implements Aliyun's RPC signature mechanism (HMAC-SHA1).
- **Retry + Diagnostics**: Conservative retries for transient failures; error includes status/request-id/body snippet (redacted by default).

## Implemented Interfaces

- **ECS Module**
    - [x] DescribeRegions
    - [x] DescribeZones
    - [x] DescribeAvailableResource
    - [x] DescribeAccountAttributes
    - [x] DescribeResourcesModification
    - [x] DescribeRecommendInstanceType
    - [x] RunInstances
    - [x] StartInstances
    - [x] StopInstances
    - [x] RebootInstance
    - [x] DeleteInstance
    - [x] DescribeInstanceStatus
    - [x] DescribeInstances

- **Billing Module**
    - [x] QueryAccountBalance
- **STS Module**
    - [x] GetCallerIdentity
    - [ ] AssumeRole
    - [ ] AssumeRoleWithSAML
    - [ ] AssumeRoleWithOIDC 

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
# Async + native-tls (default)
alibabacloud = "0.1.1"
# Required to run the async example below
tokio = { version = "1", features = ["macros", "rt"] }

# Blocking only (no tokio/reqwest)
# alibabacloud = { version = "0.1.1", default-features = false, features = ["blocking", "native-tls"] }

# Async + rustls (no native-tls)
# alibabacloud = { version = "0.1.1", default-features = false, features = ["async", "rustls"] }
```

Then import and use the interfaces.

### Async

```rust
use alibabacloud::{Auth, Client};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), alibabacloud::Error> {
    let client = Client::builder()
        .auth(Auth::access_key("ACCESS_KEY_ID", "ACCESS_KEY_SECRET"))
        .build()?;

    let regions = client.ecs().describe_regions(Default::default()).await?;
    println!("{regions:#?}");

    let caller_identity = client.sts().get_caller_identity().await?;
    println!("{caller_identity:#?}");

    Ok(())
}
```

### Blocking

```rust
use alibabacloud::{Auth, BlockingClient};

fn main() -> Result<(), alibabacloud::Error> {
    let client = BlockingClient::builder()
        .auth(Auth::access_key("ACCESS_KEY_ID", "ACCESS_KEY_SECRET"))
        .build()?;

    let regions = client.ecs().describe_regions(Default::default())?;
    println!("{regions:#?}");

    let caller_identity = client.sts().get_caller_identity()?;
    println!("{caller_identity:#?}");

    Ok(())
}
```
