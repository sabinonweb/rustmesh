use quinn::Connection;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

pub type PeerTable: Arc<Mutex<HashMap<String, Connection>>>;

fn main() {
    println!("Hello, world!");
}
