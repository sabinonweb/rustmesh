use crate::{behaviour::RustMeshBehaviour, config::NodeConfig, error::RustMeshError};
use clap::Parser;
use libp2p::futures::StreamExt;
use libp2p::{identity, quic, swarm::SwarmEvent, Multiaddr, PeerId, SwarmBuilder};
use rustmesh_core::{behaviour::RustMeshEvent, *};
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
    let mut config = NodeConfig::new(args.name.clone());
    config.listen_addr =
        args.listen.clone().parse().map_err(|e| {
            RustMeshError::ConfigError(format!("Failed to create behaviour: {:?}", e))
        })?;

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    info!("Local Peer Id: {:?}", local_peer_id);

    // let quic_config = quic::Config::new(&local_key);
    // local key for identification and encrytion
    // let transport = quic::tokio::Transport::new(quic_config);

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

    loop {
        tokio::select! {
            event = swarm.select_next_some() => {
                handle_swarm_event(event, &args.name).await;
            },
            _ = tokio::time::sleep(std::time::Duration::from_secs(30)) => {
            info!("Heartbeat from: {:?}", &args.name);
        }

        }
    }
}

async fn handle_swarm_event(event: SwarmEvent<RustMeshEvent>, node_name: &str) {
    match event {
        SwarmEvent::ListenerError { listener_id, .. } => {
            info!("[{}] Listening on address {:?}", node_name, listener_id);
        }

        SwarmEvent::Dialing { connection_id, .. } => {
            info!("[{}] Dialing {:?}", node_name, connection_id);
        }

        SwarmEvent::ConnectionEstablished {
            peer_id,
            endpoint,
            num_established,
            ..
        } => {
            info!(
                "[{}] Connection established with {} ({}) [total: {}]",
                node_name,
                peer_id,
                endpoint.get_remote_address(),
                num_established
            );
        }

        SwarmEvent::ConnectionClosed {
            peer_id,
            num_established,
            cause,
            ..
        } => {
            info!(
                "[{}] Connection closed with {} [{:?}] [remaining: {}]",
                node_name, peer_id, cause, num_established
            );
        }

        SwarmEvent::IncomingConnection {
            local_addr,
            send_back_addr,
            ..
        } => {
            info!(
                "[{}] Incoming Connection from {} (local: {})",
                node_name, send_back_addr, local_addr
            );
        }

        SwarmEvent::Behaviour(event) => {
            handle_behaviour_event(event, node_name);
        }

        _ => {}
    }
}

fn handle_behaviour_event(event: RustMeshEvent, node_name: &str) {
    match event {
        RustMeshEvent::Gossipsub(gs_event) => match gs_event {
            libp2p::gossipsub::Event::Message {
                propagation_source,
                message_id,
                message,
            } => {
                let text = String::from_utf8_lossy(&message.data);
                info!(
                    "[{}] Message from {} [{}]: {}",
                    node_name, propagation_source, message_id, text
                );
            }

            libp2p::gossipsub::Event::Subscribed { peer_id, topic } => {
                info!("[{}] Peer {} Subscribed to {}", node_name, peer_id, topic);
            }

            libp2p::gossipsub::Event::Unsubscribed { peer_id, topic } => {
                info!(
                    "[{}] Peer {} Unsubscribed from {}",
                    node_name, peer_id, topic
                );
            }

            libp2p::gossipsub::Event::GossipsubNotSupported { peer_id } => {
                info!("[{}] Peer {} doesn't support Gossipsub", node_name, peer_id);
            }
        },

        RustMeshEvent::Kademlia(kademlia_event) => match kademlia_event {
            libp2p::kad::Event::RoutingUpdated { peer, old_peer, .. } => {
                info!(
                    "[{}] Routing updated {} (Old peer: {})",
                    node_name,
                    peer,
                    old_peer.unwrap()
                );
            }

            _ => {}
        },

        RustMeshEvent::Identify(identify_event) => match identify_event {
            libp2p::identify::Event::Received { peer_id, info } => {
                info!("[{}] Identified peer {} [{:?}]", node_name, peer_id, info);
            }

            _ => {}
        },
    }
}
