use crate::handler::handle_outgoing_connection;
use crate::{
    config::configure_server, discover::discover_services, handler::handle_incoming_connection,
    register::register_service, skip::SkipServerVerification,
};
use clap::Parser;
use core::args::Args;
use core::identity::PeerTable;
use quinn::Endpoint;
use std::sync::Arc;

#[tokio::main]
pub async fn main() -> anyhow::Result<quinn::Connection> {
    let peer = discover_services().expect("Failed to retrive peer");
    let _ = rustls::crypto::ring::default_provider().install_default();

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
