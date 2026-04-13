use clap::Parser;
use libp2p::futures::StreamExt;
use libp2p::swarm::SwarmEvent;
use libp2p::{gossipsub::IdentTopic, identity, Multiaddr, PeerId, Swarm, SwarmBuilder};
use rustmesh_core::behaviour::RustMeshEvent;
use rustmesh_core::{
    behaviour::RustMeshBehaviour, config::NodeConfig, error::RustMeshError, init_tracing, Result,
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

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
    let mut swarm = get_swarm(&args.name);

    let listen_addr: Multiaddr = args.listen.parse().unwrap();

    swarm.listen_on(listen_addr.clone()).map_err(|e| {
        RustMeshError::ConfigError(format!("Error while listening: {:?}", e.to_string()))
    })?;

    info!("[{}] Listening on {}", args.name, listen_addr);

    event_loop(&mut swarm, &args.name).await;

    Ok(())
}

async fn event_loop(swarm: &mut Swarm<RustMeshBehaviour>, node_name: &str) {
    let mut publish_interval = tokio::time::interval(Duration::from_secs(2));

    loop {
        tokio::select! {
            event = swarm.select_next_some() => {
               handle_events(node_name, event, swarm).await;
           }

           _ = sleep(Duration::from_secs(30)) => {
               info!("[{}] Heartbeat from {}", node_name, node_name);
           },


           _ = publish_interval.tick() => {
                let topic = IdentTopic::new("mesh/messages");
                let message = format!("Hello from [{}]", node_name);
                match swarm.behaviour_mut().gossipsub.publish(topic, message.as_bytes().to_vec()) {
                     Ok(_) => info!("[{}] Published: {}", node_name, message),
                    Err(e) => error!("[{}] Publish failed: {}", node_name, e),
                 }
           }
        }
    }
}

fn get_swarm(node_name: &str) -> Swarm<RustMeshBehaviour> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from_public_key(&local_key.public());

    info!("Local Peer Address: {}", local_peer_id);

    let node_config = NodeConfig::new(node_name.to_string());

    let behaviour = RustMeshBehaviour::new(node_config, local_key.clone(), local_peer_id)
        .map_err(|e| {
            RustMeshError::ConfigError(format!("Error forming the behaviour: {}", e.to_string()))
        })
        .expect("Error forming the behaviour");

    SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_quic()
        .with_behaviour(|_| behaviour)
        .unwrap()
        .build()
}

async fn handle_events(
    node_name: &str,
    event: SwarmEvent<RustMeshEvent>,
    swarm: &mut Swarm<RustMeshBehaviour>,
) {
    match event {
        SwarmEvent::Dialing {
            peer_id,
            connection_id,
        } => {
            info!(
                "[{}] Dialing the peer {:?} at {}",
                node_name, peer_id, connection_id
            );
        }

        SwarmEvent::Behaviour(event) => handle_behaviour(event, node_name, swarm).await,

        SwarmEvent::NewListenAddr {
            listener_id,
            address,
        } => {
            info!(
                "[{}] New listener {} with address {}",
                node_name, listener_id, address
            );
        }

        _ => {}
    }
}

async fn handle_behaviour(
    event: RustMeshEvent,
    node_name: &str,
    swarm: &mut Swarm<RustMeshBehaviour>,
) {
    match event {
        RustMeshEvent::Mdns(mdns_event) => match mdns_event {
            libp2p::mdns::Event::Discovered(peer_info) => {
                for (_, (peer_id, multiaddr)) in peer_info.iter().enumerate() {
                    info!(
                        "[{}] Discovered peer {} [{}]",
                        node_name, peer_id, multiaddr
                    );

                    swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);

                    if swarm.is_connected(peer_id) {
                        info!("[{}] already connected to {}", node_name, peer_id);
                    } else {
                        match swarm.dial(*peer_id) {
                            Ok(()) => info!("[{}] dialing {}", node_name, peer_id),
                            Err(e) => error!(
                                "[{}] error while dialing {}: {}",
                                node_name,
                                peer_id,
                                e.to_string()
                            ),
                        }
                    }
                }
            }

            libp2p::mdns::Event::Expired(peer_info) => {
                for (peer_id, multiaddr) in peer_info {
                    info!("[{}] Expired peer {} [{}]", node_name, peer_id, multiaddr);
                }
            }
        },

        RustMeshEvent::Identify(identify_event) => match identify_event {
            libp2p::identify::Event::Pushed { peer_id, info } => {
                info!("[{}] pushed {}: {:?}", node_name, peer_id, info);
            }

            libp2p::identify::Event::Sent { peer_id } => {
                info!("[{}] identity sent to {}", node_name, peer_id);
            }

            libp2p::identify::Event::Received { peer_id, info } => {
                info!(
                    "[{}] identity received from {}: {:?}",
                    node_name, peer_id, info
                );
            }

            libp2p::identify::Event::Error { peer_id, error } => {
                error!(
                    "[{}] error while [identify] with {}: {}",
                    node_name, peer_id, error
                );
            }
        },

        RustMeshEvent::Kademlia(kad_event) => match kad_event {
            libp2p::kad::Event::ModeChanged { new_mode } => {
                info!("[{}] mode changed {}", node_name, new_mode);
            }

            _ => {}
        },

        RustMeshEvent::Gossipsub(gossip_event) => match gossip_event {
            libp2p::gossipsub::Event::Message {
                propagation_source,
                message_id,
                message,
            } => {
                info!(
                    "[{}] message {}: {:?} from {}",
                    node_name, message_id, message, propagation_source
                );
            }

            libp2p::gossipsub::Event::Subscribed { peer_id, topic } => {
                info!("[{}] {} Subscribed to {}", node_name, peer_id, topic);
            }

            libp2p::gossipsub::Event::Unsubscribed { peer_id, topic } => {
                info!("[{}] {} Unsubscribed from {}", node_name, peer_id, topic);
            }

            libp2p::gossipsub::Event::GossipsubNotSupported { peer_id } => {
                info!("[{}] gossip not supported by {}", node_name, peer_id);
            }
        },
    }
}
