pub mod three_nodes_gossipsub;

use libp2p::{identity, quic, PeerId, Swarm, SwarmBuilder};
use rustmesh_core::{behaviour::RustMeshBehaviour, error::RustMeshError};

pub async fn create_test_node(name: &str) -> (Swarm<RustMeshBehaviour>, PeerId) {
    let node_config = rustmesh_core::config::NodeConfig::new(name.to_string());
    println!("Config Topics : {:?}", node_config.topics);

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    // let quic_config = quic::Config::new(&local_key);
    // let transport = quic::tokio::Transport::new(quic_config);

    let behaviour = RustMeshBehaviour::new(node_config, local_key.clone(), local_peer_id)
        .map_err(|e| {
            RustMeshError::ConfigError(format!("Failed to create a config: {:?}", e.to_string()))
        })
        .unwrap();

    let swarm = SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_quic()
        .with_behaviour(|_| behaviour)
        .unwrap()
        .build();

    (swarm, local_peer_id)
}

pub async fn create_test_nodes(n: usize) -> Vec<(Swarm<RustMeshBehaviour>, PeerId)> {
    let mut nodes = Vec::new();

    for i in 0..n {
        let (swarm, peer_id) = create_test_node(&format!("test-node-{}", i)).await;
        nodes.push((swarm, peer_id));
    }

    nodes
}
