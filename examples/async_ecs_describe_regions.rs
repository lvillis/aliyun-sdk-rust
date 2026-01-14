#[cfg(feature = "async")]
use alibabacloud::{Auth, Client};

#[cfg(feature = "async")]
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), alibabacloud::Error> {
    let access_key_id = std::env::var("ALIBABA_CLOUD_ACCESS_KEY_ID").unwrap_or_default();
    let access_key_secret = std::env::var("ALIBABA_CLOUD_ACCESS_KEY_SECRET").unwrap_or_default();

    if access_key_id.is_empty() || access_key_secret.is_empty() {
        eprintln!("Set ALIBABA_CLOUD_ACCESS_KEY_ID and ALIBABA_CLOUD_ACCESS_KEY_SECRET");
        return Ok(());
    }

    let client = Client::builder()
        .auth(Auth::access_key(access_key_id, access_key_secret))
        .build()?;

    let regions = client.ecs().describe_regions(Default::default()).await?;
    println!("{regions:#?}");
    Ok(())
}

#[cfg(not(feature = "async"))]
fn main() {
    eprintln!("This example requires the default feature `async` (or: --features async).");
}
