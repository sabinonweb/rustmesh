use anyhow::Result;
use quinn::{Endpoint, ServerConfig};
use rcgen::generate_simple_self_signed;
use rustls::{Certificate, PrivateKey};
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cert = generate_simple_self_signed(vec!["localhost".into()]);
    let cert_der = cert.serialize_der()?;
    let key_der = cert.serialize_private_key_der();
}
