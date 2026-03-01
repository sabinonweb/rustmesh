use anyhow::Ok;
use node::{connection::connect_to_peer, server::server};
use quinn::Connection;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = server().await;

    let _ = connect_to_peer().await;
    Ok(())
}
