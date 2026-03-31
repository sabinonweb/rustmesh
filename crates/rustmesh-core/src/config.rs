use libp2p::Multiaddr;
use serde::{Deserialize, Serialize};

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

    pub with_addr(mut self, addr: &str) -> Self {
        self.listen_addr = addr.parse().expect("Invalid Multiaddr");

        self
    }

    pub fn with_bootstrap_nodes(mut self, peers: Vec<&str>) -> Self {
        self.bootstrap_nodes = peers.into_iter().map(|p| p.parse().expect("Invalid Multiaddr")).collect();

        self
    }

    pub fn with_topics(mut self, topics: Vec<&str>) -> Self {
        self.topics = topics.into_iter().map(|t| t.to_string()).collect();

        self
    }

    pub fn set_enable_ble(mut self) -> self {
        self.ble = true;
        self
    }

    pub fn disable_dht(mut self) -> self {
        self.enable_dht = false;
        self
    }
}
