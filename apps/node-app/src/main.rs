use anyhow::Result;
use core::identity::Identity;
use quinn::{Endpoint, ServerConfig};
use rcgen::generate_simple_self_signed;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let my_id = Identity::generate();
    println!("Server Peer ID: {}", my_id.peer_id());

    let cert = generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = cert.cert.der();
    let key_der = cert.key_pair.serialize_der().to_vec();

    let cert_chain = vec![rustls::pki_types::CertificateDer::from(cert_der.to_vec())];
    let private_key = rustls::pki_types::PrivateKeyDer::try_from(key_der).unwrap();

    let server_config = ServerConfig::with_single_cert(cert_chain, private_key).unwrap();

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    let endpoint = Endpoint::server(server_config, addr).unwrap();

    println!("Server listening on {}", addr);

    if let Some(connection) = endpoint.accept().await {
        let connection = connection.await?;
        println!("Client connected: {}", connection.remote_address());
    }

    Ok(())
}

async fn client() -> Result<()> {
    let my_id = Identity::generate();
    println!("Client Peer ID: {}", my_id.peer_id());

    let endpoint = Endpoint::client("0.0.0.0".parse()?).unwrap();
    let server_addr: SocketAddr = "127.0.0.1:8080".parse()?;
    let connection = endpoint.connect(server_addr, "localhost")?;

    println!("Connected to {:?}", connection.remote_address());

    Ok(())
}
