use crate::{behaviour::RustMeshBehaviour, error::RustMeshError, transport::Transport};
use libp2p::{identity, Multiaddr, PeerId, Swarm, SwarmBuilder};
use tracing::info;

pub struct QuicTransport {
    pub swarm: Swarm<RustMeshBehaviour>,
    pub peer_id: PeerId,
}

impl QuicTransport {
    pub fn new(node_name: &str) -> Self {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from_public_key(&local_key.public());

        info!("Local Peer Address: {}", local_peer_id);

        let node_config = crate::config::NodeConfig::new(node_name.to_string());

        let behaviour = RustMeshBehaviour::new(node_config, local_key.clone(), local_peer_id)
            .map_err(|e| {
                RustMeshError::ConfigError(format!(
                    "Error forming the behaviour: {}",
                    e.to_string()
                ))
            })
            .expect("Error forming the behaviour");

        Self {
            swarm: SwarmBuilder::with_existing_identity(local_key)
                .with_tokio()
                .with_quic()
                .with_behaviour(|_| behaviour)
                .unwrap()
                .build(),
            peer_id: local_peer_id,
        }
    }
}

impl Transport for QuicTransport {
    fn listen(&mut self, addr: &str) -> Result<(), RustMeshError> {
        let listen_addr: Multiaddr = addr.parse().expect("Failed to parse the address");

        self.swarm.listen_on(listen_addr.clone()).map_err(|e| {
            RustMeshError::ConfigError(format!("Error while listening: {:?}", e.to_string()))
        })?;

        Ok(())
    }

    fn dial(&mut self, addr: &str) -> Result<(), RustMeshError> {
        let dial_addr: Multiaddr = addr.parse().expect("Failed to parse the address");

        self.swarm.dial(dial_addr).map_err(|e| {
            RustMeshError::ConfigError(format!("Error while dialing: {:?}", e.to_string()))
        })?;

        Ok(())
    }
}
