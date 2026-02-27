use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Identity {
    pub verifying_key: Vec<u8>,
}

impl Identity {
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key().to_bytes().to_vec();
        Self { verifying_key }
    }

    pub fn peer_id(&self) -> String {
        hex::encode(&self.verifying_key)
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Peer {
    pub id: Identity,
    pub ip: String,
    pub port: u16,
}

impl Peer {
    pub fn new(ip: &str, port: u16) -> Peer {
        Peer {
            id: Identity::generate(),
            ip: ip.to_string(),
            port,
        }
    }
}
