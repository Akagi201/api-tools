use std::net::SocketAddr;

use clap::Parser;
use hyper::{body, server::conn::http1, service::service_fn, Request, Response};
use hyper_util::{
    client::legacy::Client,
    rt::{TokioExecutor, TokioIo},
};
use tokio::net::TcpListener;

mod cli;
use cli::Cli;

async fn log(
    req: Request<body::Incoming>,
) -> Result<Response<hyper::body::Incoming>, hyper_util::client::legacy::Error> {
    let path = req.uri().path();

    if path.starts_with("/api") {
        println!("API Path: {}", path);
    } else {
        println!("Generic Path: {}", path);
    }

    handle(req).await
}

async fn handle(
    req: Request<body::Incoming>,
) -> Result<Response<hyper::body::Incoming>, hyper_util::client::legacy::Error> {
    let client = Client::builder(TokioExecutor::new()).build_http();
    client.request(req).await
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let cli = Cli::parse();
    println!("cli: {:?}", cli);
    let addr: SocketAddr = format!("{}:{}", cli.host, cli.port).parse().unwrap();

    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(log))
                .await
            {
                println!("error: {}", err);
            }
        });
    }
}
