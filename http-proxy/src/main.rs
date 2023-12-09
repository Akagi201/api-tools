use std::net::SocketAddr;

use clap::Parser;
use hyper::{service::service_fn, Body, Client, Request, Response, Server};
use tower::make::Shared;

mod cli;
use cli::Cli;

async fn log(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let path = req.uri().path();

    if path.starts_with("/api") {
        println!("API Path: {}", path);
    } else {
        println!("Generic Path: {}", path);
    }

    handle(req).await
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let client = Client::new();
    client.request(req).await
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!("cli: {:?}", cli);
    let addr: SocketAddr = format!("{}:{}", cli.host, cli.port).parse().unwrap();
    let make_service = Shared::new(service_fn(log));
    let server = Server::bind(&addr).serve(make_service);

    if let Err(e) = server.await {
        println!("error: {}", e);
    }
}
