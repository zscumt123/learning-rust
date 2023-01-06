use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;

async fn hello(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new(Body::from("Hello World!")))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Sync + Send>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3333));
    let make_svc = make_service_fn(|_c| async { Ok::<_, Infallible>(service_fn(hello)) });
    let server = Server::bind(&addr).serve(make_svc);
    println!("Listening on http://{}", addr);

    server.await?;

    Ok(())
}
