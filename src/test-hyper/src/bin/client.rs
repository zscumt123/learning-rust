use hyper::{body::HttpBody as _, Client};
use tokio::io::{self, AsyncWriteExt as _};
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let client = Client::new();

    // Parse an `http::Uri`...
    let uri = "http://127.0.0.1:3333".parse()?;

    // Await the response...
    let mut resp = client.get(uri).await?;

    println!("Response: {:?}", resp.status());
    while let Some(chunk) = resp.data().await {
      io::stdout().write_all(&chunk?).await?;
  }

    // This is where we will setup our HTTP client requests.

    Ok(())
}
