use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use ed25519_dalek::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct Identity {
    pub verifying_key: Vec<u8>,
}

impl Identity {
    pub fn generate() -> Self {
        let signing_key = SigningKey::generate(&mut OsRng);
        let verifying_key = signing_key.verifying_key().to_bytes().to_vec();
        Self { verifying_key }
    }

    pub fn encode(&self) -> String {
        hex::encode(&self.verifying_key)
    }

    pub fn identity(peer_id: String) -> Identity {
        Identity {
            verifying_key: hex::decode(peer_id).unwrap(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Peer {
    pub id: String,
    pub ip: String,
    pub port: u16,
}

impl Peer {
    pub fn new(ip: &str, port: u16) -> Peer {
        Peer {
            id: Identity::generate().encode(),
            ip: ip.to_string(),
            port,
        }
    }
}

pub type PeerTable = Arc<Mutex<HashMap<String, Peer>>>;
