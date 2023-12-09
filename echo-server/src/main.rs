use clap::Parser;
use http::Response;
use hyper::{server, service::service_fn};
use hyper_util::rt::TokioIo;

mod cli;
use cli::Cli;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!("cli: {:?}", cli);
    let addr = format!("{}:{}", cli.host, cli.port);

    let socket = tokio::net::TcpListener::bind(&addr).await.unwrap();
    println!("Echo server listening on {}", addr);

    loop {
        if let Ok((stream, _)) = socket.accept().await {
            let io = TokioIo::new(stream);
            let service =
                service_fn(|req| async { Ok::<_, hyper::Error>(Response::new(req.into_body())) });
            tokio::spawn(async move {
                if let Err(err) = server::conn::http1::Builder::new()
                    .preserve_header_case(true)
                    .title_case_headers(true)
                    .keep_alive(true)
                    .serve_connection(io, service)
                    .with_upgrades()
                    .await
                {
                    eprintln!("server error: {}", err);
                };
            });
        }
    }
}
