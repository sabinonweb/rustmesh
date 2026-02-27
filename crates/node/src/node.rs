use std::sync::Arc;

use quinn::Endpoint;

use crate::discover::discover_services;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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

    let (mut send, mut recv) = connection.open_bi().await?;
    let message = format!("Hello from client {}", peer.id.peer_id());

    send.write_all(message.as_bytes()).await?;

    let mut buf = vec![0; 1024];
    match recv.read(&mut buf).await {
        Ok(Some(n)) => {
            let reply = String::from_utf8_lossy(&buf[..n]);
            println!("Reply: {:?}", reply);
        }
        Ok(None) => {
            println!("Stream closed by server");
        }
        Err(e) => {
            println!("Error reading from stream: {}", e);
        }
    }

    Ok(())
}
