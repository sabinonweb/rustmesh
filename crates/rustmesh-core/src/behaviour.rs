use std::time::Duration;

use crate::{config::NodeConfig, error::RustMeshError};
use libp2p::{
    gossipsub::{self, IdentTopic, MessageAuthenticity, ValidationMode},
    identify, kad, mdns,
    swarm::NetworkBehaviour,
    PeerId,
};
use tracing::info;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "RustMeshEvent")]
pub struct RustMeshBehaviour {
    pub gossipsub: gossipsub::Behaviour,

    pub identify: identify::Behaviour,

    pub kademlia: kad::Behaviour<kad::store::MemoryStore>,

    pub mdns: mdns::tokio::Behaviour,
}

pub enum RustMeshEvent {
    Gossipsub(gossipsub::Event),
    Identify(identify::Event),
    Kademlia(kad::Event),
    Mdns(mdns::Event),
}

impl From<gossipsub::Event> for RustMeshEvent {
    fn from(event: gossipsub::Event) -> Self {
        RustMeshEvent::Gossipsub(event)
    }
}

impl From<identify::Event> for RustMeshEvent {
    fn from(event: identify::Event) -> Self {
        RustMeshEvent::Identify(event)
    }
}

impl From<kad::Event> for RustMeshEvent {
    fn from(event: kad::Event) -> Self {
        RustMeshEvent::Kademlia(event)
    }
}

impl From<mdns::Event> for RustMeshEvent {
    fn from(event: mdns::Event) -> Self {
        RustMeshEvent::Mdns(event)
    }
}

impl RustMeshBehaviour {
    pub fn new(
        config: NodeConfig,
        local_key: libp2p::identity::Keypair,
        local_peer_id: PeerId,
    ) -> crate::Result<Self> {
        info!("Initializing RustMesh: {}", config.name);

        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .validation_mode(ValidationMode::Strict)
            .max_transmit_size(config.max_message_size)
            .history_length(120) // Each peer remembers 120 messages per topic
            .history_gossip(10) // Gossip 10 messages in one heartbeat
            .history_gossip(10)
            .max_ihave_length(100) // maximum number of IHAVE messagesID it can send
            .max_ihave_messages(500) // maximum number of IHAVE messages it will keep track of
            .heartbeat_interval(std::time::Duration::from_millis(
                config.gossipsub_heartbeat_ms,
            ))
            .build()
            .map_err(|e| RustMeshError::LibP2P(e.to_string()));

        let gossipsub = gossipsub::Behaviour::new(
            MessageAuthenticity::Signed(local_key.clone()),
            gossipsub_config?,
        )
        .map_err(|e| RustMeshError::LibP2P(e.to_string()))?;

        let kademlia = kad::Behaviour::with_config(
            local_peer_id,
            kad::store::MemoryStore::new(local_peer_id),
            Default::default(),
        );

        let identify = identify::Behaviour::new(identify::Config::new(
            "/rustmesh/1.0.0".to_string(),
            local_key.public(),
        ));

        let mdns_config = mdns::Config {
            ttl: Duration::from_secs(60),
            query_interval: Duration::from_secs(5),
            enable_ipv6: false,
        };

        let mdns = mdns::tokio::Behaviour::new(mdns_config, local_peer_id)?;

        let mut behaviour = Self {
            gossipsub,
            kademlia,
            identify,
            mdns,
        };

        for topic in &config.topics {
            let topic = IdentTopic::new(topic);

            behaviour
                .gossipsub
                .subscribe(&topic)
                .map_err(|e| RustMeshError::LibP2P(e.to_string()))?;
            info!("Subscribed to: {}", topic);
        }

        info!("Rustmesh Initialized successfully!");
        Ok(behaviour)
    }

    pub fn publish(&mut self, topic: &str, data: Vec<u8>) -> crate::Result<()> {
        let topic = IdentTopic::new(topic);

        self.gossipsub
            .publish(topic.clone(), data)
            .map_err(|e| RustMeshError::LibP2P(e.to_string()))?;
        info!("Subscribed to: {}", topic);

        Ok(())
    }

    pub fn subscribe(&mut self, topic: &str) -> crate::Result<()> {
        let topic = IdentTopic::new(topic);

        self.gossipsub
            .subscribe(&topic)
            .map_err(|e| RustMeshError::LibP2P(e.to_string()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity;

    #[test]
    fn test_behaviour_creation() {
        let config = NodeConfig::new("on-test".to_string());
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        let result = RustMeshBehaviour::new(config, local_key, local_peer_id);
        assert!(result.is_ok());
    }

    #[test]
    fn test_behaviour_creation_with_topics() {
        let config = NodeConfig::new("on-test".to_string()).with_topics(vec!["topic1", "topic2"]);
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        let result = RustMeshBehaviour::new(config.clone(), local_key, local_peer_id);
        assert_eq!(config.topics.len(), 2);
    }
}
