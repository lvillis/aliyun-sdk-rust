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

This project is an Aliyun API SDK written in Rust, designed to help developers integrate Aliyun Cloud services easily. The SDK leverages asynchronous programming (via Tokio) and encapsulates functionalities such as request signing (HMAC-SHA1), unified request handling, and modular service interfaces (e.g., ECS, Billing).

## Features

- **Asynchronous Support**: Built on Tokio for high concurrency.
- **Request Signing**: Implements Aliyun's API signature mechanism using HMAC-SHA1.
- **Modular Design**: The project is organized into multiple modules (e.g., ECS, Billing) with clear separation of concerns.
- **Detailed Documentation**: Each interface is documented with detailed input/output parameter tables.
- **Comprehensive Testing**: Each service interface includes test cases to ensure correct functionality.

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

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
alibabacloud = "0.1.0"
```

Then import and use the interfaces:

```rust
use alibabacloud::client::AliyunClient;
use alibabacloud::services::ecs::*;

#[tokio::main]
async fn main() {
    let client = AliyunClient::new("YourAccessKeyId".into(), "YourAccessKeySecret".into());
    // Query available regions
    let regions = describe_regions(&client, None).await.unwrap();
    println!("Available regions: {:?}", regions);
}
```
