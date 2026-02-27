use core::identity::Identity;
use mdns_sd::{ServiceDaemon, ServiceEvent};
use quinn::Endpoint;
use rustls::pki_types::CertificateDer;
use std::{
    fmt::format,
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Clone, Debug)]
struct Peer {
    id: Identity,
    ip: String,
    port: u16,
}

#[derive(Debug)]
struct SkipServerVerification;

impl rustls::client::danger::ServerCertVerifier for SkipServerVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &rustls::pki_types::ServerName<'_>,
        _ocsp_response: &[u8],
        _now: rustls::pki_types::UnixTime,
    ) -> Result<rustls::client::danger::ServerCertVerified, rustls::Error> {
        Ok(rustls::client::danger::ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<rustls::client::danger::HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::client::danger::HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ED25519,
        ]
    }
}

fn discover_services() -> Result<Peer, String> {
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");
    let my_id = Identity::generate();
    let service_type = "_mdns-sd-my-test._udp.local.";
    let receiver = mdns.browse(service_type).expect("Failed to browse");
    let result: Arc<Mutex<Option<Result<Peer, String>>>> = Arc::new(Mutex::new(None));
    let result_clone = Arc::clone(&result);

    std::thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            if let ServiceEvent::ServiceResolved(resolved) = event {
                println!("Resolved a full service: {}", resolved.fullname);
                let peer_id = resolved.get_property_val_str("peer_id").unwrap();
                println!("Service resolved from the peer: {:?}\n", peer_id);

                if let Some(ip) = resolved.get_addresses().iter().next() {
                    let mut result = result_clone.lock().unwrap();
                    *result = Some(Ok(Peer {
                        id: my_id.clone(),
                        ip: ip.to_string(),
                        port: resolved.port,
                    }));
                    break; // stop after first resolved peer
                }
            } else {
                println!("Ignoring event: {:?}", event);
            }
        }
    });
    std::thread::sleep(Duration::from_secs(10));

    match result.lock().unwrap().clone() {
        Some(p) => Ok(p?),
        None => {
            println!("No service received!");
            Err(format!("No service received!"))
        }
    }
}

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
