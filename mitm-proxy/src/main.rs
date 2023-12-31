use std::sync::Arc;

use clap::Parser;
use futures::StreamExt;
use http_mitm_proxy::MitmProxy;

mod cli;
use cli::Cli;

fn make_root_cert() -> rcgen::Certificate {
    let mut param = rcgen::CertificateParams::default();

    param.distinguished_name = rcgen::DistinguishedName::new();
    param.distinguished_name.push(
        rcgen::DnType::CommonName,
        rcgen::DnValue::Utf8String("<HTTP-MITM-PROXY CA>".to_string()),
    );
    param.key_usages = vec![
        rcgen::KeyUsagePurpose::KeyCertSign,
        rcgen::KeyUsagePurpose::CrlSign,
    ];
    param.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    rcgen::Certificate::from_params(param).unwrap()
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    println!("cli: {:?}", cli);
    let root_cert = Arc::new(make_root_cert());

    let proxy = MitmProxy::new(
        // This is the root cert that will be used to sign the fake certificates
        Some(root_cert.clone()),
        // This is the connector that will be used to connect to the upstream server from proxy
        tokio_native_tls::native_tls::TlsConnector::new().unwrap(),
    );

    let (mut communications, server) = proxy.bind((cli.host.clone(), cli.port)).await.unwrap();
    tokio::spawn(server);

    println!("HTTP Proxy is listening on {}:{}", cli.host, cli.port);

    println!();
    println!("Trust this cert if you want to use HTTPS");
    println!();
    println!("{}", root_cert.serialize_pem().unwrap());
    println!();

    // Save this cert to ca.crt and use it with curl like this:
    // curl https://www.google.com -x http://127.0.0.1:8088 --cacert ca.crt

    while let Some(comm) = communications.next().await {
        let uri = comm.request.uri().clone();
        // modify the request here if you want
        let _ = comm.request_back.send(comm.request);
        if let Ok(mut response) = comm.response.await {
            let mut len = 0;
            let body = response.body_mut();
            while let Some(frame) = body.next().await {
                if let Ok(frame) = frame {
                    if let Some(data) = frame.data_ref() {
                        len += data.len();
                    }
                }
            }
            println!(
                "{}\t{}\t{}\t{}",
                comm.client_addr,
                uri,
                response.status(),
                len
            );
        }
    }
}
