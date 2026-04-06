use crate::{behaviour::RustMeshBehaviour, config::NodeConfig, error::RustMeshError};
use clap::Parser;
use libp2p::{identity, quic, Multiaddr, PeerId, SwarmBuilder};
use rustmesh_core::*;
use tracing::info;

#[derive(Parser, Debug, Clone)]
#[command(name = "RustMesh Node")]
#[command(about = "Peer-to-peer mesh network node", long_about = None)]
struct Args {
    #[arg(short, long, default_value = "rustmesh-node")]
    name: String,

    #[arg(short, long, default_value = "/ip4/0.0.0.0/udp/0/quic-v1")]
    listen: String,

    #[arg(short, long, default_value = "info")]
    log_level: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    init_tracing(&args.log_level);
    let mut config = NodeConfig::new(args.name);
    config.listen_addr =
        args.listen.clone().parse().map_err(|e| {
            RustMeshError::ConfigError(format!("Failed to create behaviour: {:?}", e))
        })?;

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    info!("Local Peer Id: {:?}", local_peer_id);

    let quic_config = quic::Config::new(&local_key);
    // local key for identification and encrytion
    let transport = quic::tokio::Transport::new(quic_config);

    let behaviour =
        RustMeshBehaviour::new(config, local_key.clone(), local_peer_id).map_err(|e| {
            RustMeshError::ConfigError(format!("Failed to create a behaviour: {:?}", e))
        })?;

    let mut swarm = SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_quic()
        .with_behaviour(|_| behaviour)
        .unwrap()
        .build();

    let listen_addr: Multiaddr = args
        .listen
        .parse()
        .map_err(|e| RustMeshError::ConfigError(format!("Invalid address: {:?}", e)))?;

    swarm.listen_on(listen_addr).map_err(|e| {
        RustMeshError::ConfigError(format!(
            "Couldn't listen on address {:?}: {:?}",
            args.listen, e
        ))
    })?;

    info!("Node listening on {:?}", args.listen);

    Ok(())
}
