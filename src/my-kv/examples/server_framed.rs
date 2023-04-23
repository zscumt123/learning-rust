use anyhow::Result;
use bytes::Bytes;
use futures::prelude::*;
use kv::{CommandRequest, Service, ServiceInner, SledDb};
use prost::Message;
use tokio::net::TcpListener;
use tokio_util::codec::{Framed, LengthDelimitedCodec};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let service: Service<SledDb> = ServiceInner::new(SledDb::new("/tmp/kvserver"))
        .fn_before_send(|res| match res.message.as_ref() {
            "" => res.message = "altered. Original message is empty.".into(),
            s => res.message = format!("altered: {}", s),
        })
        .into();
    let addr = "127.0.0.1:9527";
    let listener = TcpListener::bind(addr).await?;
    info!("Start listening on {}", addr);
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Client {:?} connected", addr);
        let svc = service.clone();
        let mut stream = Framed::new(stream, LengthDelimitedCodec::new());

        tokio::spawn(async move {
            while let Some(Ok(cmd)) = stream.next().await {
                let res = svc.execute(CommandRequest::decode(cmd).unwrap());
                stream.send(Bytes::from(res.encode_to_vec())).await.unwrap();
            }
            info!("Client {:?} disconnected", addr);
        });
    }
}
