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

pub async fn connect_to_peer() -> anyhow::Result<quinn::Connection> {
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

pub async fn server() -> anyhow::Result<()> {
    let my_id = register_service();
    let args = Args::parse();
    // let _ = rustls::crypto::ring::default_provider().install_default();
    let address = format!("{}:{}", args.ip, args.port).parse()?;

    let (server_config, _cert) = configure_server().unwrap();

    let endpoint = Endpoint::server(server_config, address)?;
    println!("Server listening on {}", address);

    while let Some(incoming) = endpoint.accept().await {
        let value = my_id.clone();

        println!("Connection incoming from {}", incoming.remote_address());

        tokio::spawn(async move {
            match incoming.await {
                Ok(connection) => {
                    println!("Connection established: {:?}", connection.remote_address());
                    tokio::spawn(handle_incoming_connection(connection, value.clone()));
                }
                Err(e) => {
                    eprintln!("Connection error: {:?}", e);
                }
            }
        });
    }

    Ok(())
}
