use core::identity::{Peer, PeerTable};
use once_cell::sync::Lazy;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub static GLOBAL_PEER_TABLE: Lazy<PeerTable> = Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

pub fn add_peer(peer: Peer) {
    let mut table = GLOBAL_PEER_TABLE.lock().unwrap();
    table.insert(peer.id.clone(), peer);
}

pub fn remove_peer(peer: Peer) {
    let mut table = GLOBAL_PEER_TABLE.lock().unwrap();
    table.remove(&peer.id);
}

pub fn list_peers() -> Vec<String> {
    let table = GLOBAL_PEER_TABLE.lock().unwrap();
    table.keys().cloned().collect()
}

pub mod config;
pub mod connection;
pub mod discover;
pub mod handler;
pub mod register;
pub mod skip;
