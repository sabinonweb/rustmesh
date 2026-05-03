use crate::{
    behaviour::{RustMeshBehaviour, RustMeshEvent},
    discovery::{events::RustMeshDiscoveryEvent, RustMeshDiscovery},
};
use libp2p::Swarm;
use tracing::{error, info};

pub struct WifiDiscovery {
    pub node_name: String,
    pub swarm: Swarm<RustMeshBehaviour>,
}

impl WifiDiscovery {
    pub fn new(node_name: &str) -> Self {
        WifiDiscovery {
            node_name: node_name.to_string(),
            swarm,
        }
    }
}

impl RustMeshDiscovery for WifiDiscovery {
    fn peer_discovered(&mut self, event: RustMeshDiscoveryEvent) {
        match event {
            RustMeshDiscoveryEvent::MDNSEvent(mdns_event) => match mdns_event {
                libp2p::mdns::Event::Discovered(peer_info) => {
                    for (peer_id, multiaddr) in peer_info {
                        info!(
                            "[{}] Discovered peer {} [{}]",
                            self.node_name, peer_id, multiaddr
                        );

                        self.swarm
                            .behaviour_mut()
                            .gossipsub
                            .add_explicit_peer(&peer_id);

                        if self.swarm.is_connected(&peer_id) {
                            info!(
                                "[{}] Peer already connected {} [{}]",
                                self.node_name, peer_id, multiaddr
                            );
                        } else {
                            info!(
                                "[{}] Connecting to {} [{}]",
                                self.node_name, peer_id, multiaddr
                            );

                            match self.swarm.dial(peer_id) {
                                Ok(()) => info!("[{}] dialing {}", self.node_name, peer_id),
                                Err(e) => error!(
                                    "[{}] error while dialing {}: {}",
                                    self.node_name,
                                    peer_id,
                                    e.to_string()
                                ),
                            }
                        }
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn peer_expired() {
        todo!()
    }
}
