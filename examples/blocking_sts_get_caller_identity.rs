#[cfg(feature = "blocking")]
fn main() -> Result<(), alibabacloud::Error> {
    use alibabacloud::{Auth, BlockingClient};

    let access_key_id = std::env::var("ALIBABA_CLOUD_ACCESS_KEY_ID").unwrap_or_default();
    let access_key_secret = std::env::var("ALIBABA_CLOUD_ACCESS_KEY_SECRET").unwrap_or_default();

    if access_key_id.is_empty() || access_key_secret.is_empty() {
        eprintln!("Set ALIBABA_CLOUD_ACCESS_KEY_ID and ALIBABA_CLOUD_ACCESS_KEY_SECRET");
        return Ok(());
    }

    let client = BlockingClient::builder()
        .auth(Auth::access_key(access_key_id, access_key_secret))
        .build()?;

    let identity = client.sts().get_caller_identity()?;
    println!("{identity:#?}");
    Ok(())
}

#[cfg(not(feature = "blocking"))]
fn main() {
    eprintln!(
        "This example requires feature `blocking` and a TLS backend (e.g. `--features blocking` or `--no-default-features --features blocking,rustls`)."
    );
}
