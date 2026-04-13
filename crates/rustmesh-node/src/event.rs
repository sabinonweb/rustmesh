use libp2p::futures::StreamExt;
use libp2p::{gossipsub::IdentTopic, identity, PeerId, Swarm, SwarmBuilder};
use rustmesh_core::{behaviour::RustMeshBehaviour, config::NodeConfig, error::RustMeshError};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

use crate::handler::handle_events;

pub async fn event_loop(swarm: &mut Swarm<RustMeshBehaviour>, node_name: &str) {
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

pub fn get_swarm(node_name: &str) -> (Swarm<RustMeshBehaviour>, PeerId) {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from_public_key(&local_key.public());

    info!("Local Peer Address: {}", local_peer_id);

    let node_config = NodeConfig::new(node_name.to_string());

    let behaviour = RustMeshBehaviour::new(node_config, local_key.clone(), local_peer_id)
        .map_err(|e| {
            RustMeshError::ConfigError(format!("Error forming the behaviour: {}", e.to_string()))
        })
        .expect("Error forming the behaviour");

    (
        SwarmBuilder::with_existing_identity(local_key)
            .with_tokio()
            .with_quic()
            .with_behaviour(|_| behaviour)
            .unwrap()
            .build(),
        local_peer_id,
    )
}
