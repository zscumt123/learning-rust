use anyhow::Result;
use hyper::body::HttpBody;
use hyper::Client;
use tokio::io::{stdout, AsyncWriteExt as _};
#[tokio::main]
async fn main() -> Result<()> {
  let client = Client::new();

  // Parse an `http::Uri`...
  let uri = "http://httpbin.org/ip".parse()?;
  
  // Await the response...
  let mut resp = client.get(uri).await?;
  println!("Response: {}", resp.status());
  while let Some(chunk) = resp.body_mut().data().await {
    stdout().write_all(&chunk?).await?;
}
  Ok(())
  
}
