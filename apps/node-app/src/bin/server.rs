use core::identity::Identity;
use quinn::{Connection, Endpoint, ServerConfig, TransportConfig};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::{error::Error, sync::Arc};
use tokio::io::AsyncReadExt;

fn configure_server() -> Result<(ServerConfig, Vec<u8>), Box<dyn Error>> {
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let cert_der = cert.cert.der();
    let priv_key = cert.key_pair.serialize_der();
    let certificate = CertificateDer::from(cert_der.to_vec());
    let private_key = PrivateKeyDer::try_from(priv_key).unwrap();

    let mut transport_config = TransportConfig::default();
    transport_config.max_idle_timeout(Some(std::time::Duration::from_secs(60).try_into()?));
    let transport_config = Arc::new(transport_config);

    let mut config = ServerConfig::with_single_cert(vec![certificate], private_key)?;
    config.transport = transport_config;

    std::fs::write("cert.pem", &cert.cert.pem())?;

    Ok((config, cert_der.to_vec()))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let my_id = Identity::generate();
    // let _ = rustls::crypto::ring::default_provider().install_default();
    let address = "127.0.0.1:8080".parse()?;

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
                    tokio::spawn(handle_connection(connection, value.clone()));
                }
                Err(e) => {
                    eprintln!("Connection error: {:?}", e);
                }
            }
        });
    }

    Ok(())
}

async fn handle_connection(connection: Connection, my_id: Identity) {
    if let Ok((mut send, mut recv)) = connection.accept_bi().await {
        let mut buf = vec![0; 1024];
        if let Ok(n) = recv.read(&mut buf).await {
            let n = n.unwrap();
            let message = String::from_utf8_lossy(&buf[..n]);
            println!("Message from client: {:?}", message);

            let reply = format!("{} says hello to you!", my_id.peer_id());
            let _ = send.write_all(reply.as_bytes()).await;

            let mut buf = vec![0; 1024];
            let n = recv.read(&mut buf).await.unwrap().unwrap();
            let reply = String::from_utf8_lossy(&buf[..n]);
            println!("Reply: {:?}", reply);
        }
    }
}
