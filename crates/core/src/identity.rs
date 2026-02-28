use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use sha2::{Digest, Sha256};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identity {
    pub signing_key: SigningKey,
    pub verifying_key: VerifyingKey,
}
impl Identity {
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key();
        Self {
            signing_key,
            verifying_key,
        }
    }

    pub fn encode(&self) -> String {
        let mut hasher = Sha256::new();
        hasher.update(self.verifying_key.clone());
        hex::encode(&hasher.finalize())
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        self.verifying_key.to_bytes().to_vec()
    }
}

#[derive(Clone, Debug)]
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

pub type PeerTable = Arc<Mutex<HashMap<String, Peer>>>;
