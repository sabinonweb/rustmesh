use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct NodeConfig {
    pub name: String,

    pub listen_addr: Multiaddr,

    pub bootstrap_nodes: Vec<Multiaddr>,

    pub topics: Vec<String>,

    pub enable_dht: bool,

    pub enable_quic: bool,

    pub enable_ble: bool,

    pub rpc_port: u16,

    pub key_path: Option<String>,

    pub enable_mdns: bool,

    pub log_level: String,

    pub gossipsub_heartbeart_ms: u64,

    pub max_message_size: usize,

    pub metrics_port: u16,

    pub data_dir: String,
}

impl Default for NodeConfig {
    fn default() -> Self {
        Self {
            name: "rustmesh-node".to_string(),
            listen_addr: "/ip4/0.0.0.0/udp/0/quic-v1"
                .parse()
                .expect("Valid Multiaddr"),
            bootstrap_nodes: vec![],
            topics: vec!["mesh/events".to_string(), "mesh/messages".to_string()],
            enable_dht: true,
            enable_quic: true,
            enable_ble: false,
            rpc_port: 8080,
            key_path: None,
            enable_mdns: true,
            log_level: "info".to_string(),
            gossipsub_heartbeart_ms: 500,
            max_message_size: 262144,
            metrics_port: 9090,
            data_dir: "/data".to_string(),
        }
    }
}

impl NodeConfig {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn with_addr(mut self, addr: &str) -> Self {
        self.listen_addr = addr.parse().expect("Invalid Multiaddr");

        self
    }

    pub fn with_bootstrap_nodes(mut self, peers: Vec<&str>) -> Self {
        self.bootstrap_nodes = peers
            .into_iter()
            .map(|p| p.parse().expect("Invalid Multiaddr"))
            .collect();

        self
    }

    pub fn with_topics(mut self, topics: Vec<&str>) -> Self {
        self.topics = topics.into_iter().map(|t| t.to_string()).collect();

        self
    }

    pub fn set_enable_ble(mut self) -> Self {
        self.enable_ble = true;
        self
    }

    pub fn disable_dht(mut self) -> Self {
        self.enable_dht = false;
        self
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> crate::Result<Self> {
        let contents =
            fs::read_to_string(path).map_err(|e| crate::RustMeshError::FileError(e.to_string()))?;
        let config: NodeConfig = toml::from_str(&contents)
            .map_err(|e| crate::RustMeshError::Deserialization(e.to_string()))?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        let node_config = NodeConfig::default();

        assert!(!node_config.name.is_empty());
        assert!(node_config.enable_quic);
        assert!(node_config.listen_addr.to_string().contains("quic"));
    }

    #[test]
    fn test_new() {
        let node_config = NodeConfig::new("onweb-node".to_string());

        assert_eq!(node_config.name, "onweb-node");
        assert!(node_config.enable_quic);
    }

    #[test]
    fn test_with_topics() {
        let node_config =
            NodeConfig::new("test-node".to_string()).with_topics(vec!["topic1", "topic2"]);

        assert_eq!(node_config.topics.len(), 2);
    }

    #[test]
    fn test_bootstrap() {
        let config = NodeConfig::new("test-node".to_string())
            .with_bootstrap_nodes(vec!["/ip4/0.0.0.0/udp/01/quic-v1"]);

        assert_eq!(
            config.bootstrap_nodes,
            vec!["/ip4/0.0.0.0/udp/01/quic-v1".parse().unwrap()]
        );
    }
}
