use clap::Parser;
use libp2p::Multiaddr;
use rustmesh_core::{error::RustMeshError, init_tracing, Result};
use rustmesh_node::event::{event_loop, get_swarm};
use tracing::info;

#[derive(Parser, Debug, Clone)]
#[command(name = "RustMesh Node")]
#[command(about = "Peer-to-peer mesh network node", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "rustmesh-node")]
    name: String,

    #[arg(short, long, default_value = "/ip4/0.0.0.0/udp/0/quic-v1")]
    listen: String,

    #[arg(short = 'g', long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    init_tracing(&args.log_level);
    let (mut swarm, _peer_id) = get_swarm(&args.name);

    let listen_addr: Multiaddr = args.listen.parse().unwrap();

    swarm.listen_on(listen_addr.clone()).map_err(|e| {
        RustMeshError::ConfigError(format!("Error while listening: {:?}", e.to_string()))
    })?;

    info!("[{}] Listening on {}", args.name, listen_addr);

    event_loop(&mut swarm, &args.name).await;

    Ok(())
}
