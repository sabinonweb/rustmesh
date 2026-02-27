use crate::{discover::discover_services, skip::SkipServerVerification};
use quinn::Endpoint;
use std::sync::Arc;

pub async fn connect_to_peer() -> anyhow::Result<quinn::Connection> {
    let peer = discover_services().expect("Failed to retrive peer");
    let _ = rustls::crypto::ring::default_provider().install_default();

    println!("Client Peer ID: {}", peer.id.peer_id());

    let client_config = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
        .with_no_client_auth();

    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;
    let client_config = quinn::ClientConfig::new(Arc::new(
        quinn::crypto::rustls::QuicClientConfig::try_from(client_config)?,
    ));
    endpoint.set_default_client_config(client_config);

    let server_addr = format!("{}:{}", peer.ip, peer.port).parse()?;
    // let server_addr = "127.0.0.1:8080".parse()?;
    println!("Connecting to {}...", server_addr);

    let connection = endpoint.connect(server_addr, "localhost")?.await?;
    println!("Connected to {:?}", connection.remote_address());
    Ok(connection)
}
