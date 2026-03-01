use clap::Parser;
use core::args::Args;
use node::{
    config::configure_server, handler::handle_incoming_connection, register::register_service,
};
use quinn::Endpoint;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
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
