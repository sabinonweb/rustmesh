use quinn::Endpoint;
use quinn::{ServerConfig, TransportConfig};
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use std::{error::Error, sync::Arc};

pub fn configure_server() -> Result<(ServerConfig, Vec<u8>), Box<dyn Error>> {
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

    Ok((config, cert_der.to_vec()))
}

pub fn configure_client() -> Result<quinn::ClientConfig, Box<dyn Error>> {
    let _ = rustls::crypto::ring::default_provider().install_default();

    let client_config = rustls::ClientConfig::builder()
        .dangerous()
        .with_custom_certificate_verifier(Arc::new(SkipServerVerification))
        .with_no_client_auth();

    let mut endpoint = Endpoint::client("0.0.0.0:0".parse()?)?;

    Ok(quinn::ClientConfig::new(Arc::new(
        quinn::crypto::rustls::QuicClientConfig::try_from(client_config)?,
    )))
}
