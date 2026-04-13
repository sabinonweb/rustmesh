use libp2p::{swarm::SwarmEvent, Swarm};
use rustmesh_core::behaviour::{RustMeshBehaviour, RustMeshEvent};
use tracing::{error, info};

pub async fn handle_events(
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

pub async fn handle_behaviour(
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
