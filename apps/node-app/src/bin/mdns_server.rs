use core::identity::Identity;
use mdns_sd::{ServiceDaemon, ServiceInfo};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Peer {
    pub id: Identity,
    pub ip: String,
    pub port: u16,
}

impl Peer {
    pub fn new(ip: &str, port: u16) -> Peer {
        Peer {
            id: Identity::generate(),
            ip: ip.to_string(),
            port,
        }
    }
}

fn main() {
    let mdns = ServiceDaemon::new().expect("Failed to create a daemon");
    let service_type = "_mdns-sd-my-test._udp.local.";
    // there are multiple devices that provide that service, so it is like id
    let ip = "127.0.0.1"; // localhost
    let host_name = "localhost.local.";
    let port = 5200;
    let peer_str = serde_json::to_string(&Peer::new(&ip, port)).unwrap();
    let instance_name = &peer_str.clone()[..8];

    // metadata of the service
    let properties = [("peer_id", peer_str)];

    let my_service = ServiceInfo::new(
        service_type,
        &instance_name,
        host_name,
        ip,
        port,
        &properties[..],
    )
    .unwrap();

    mdns.register(my_service)
        .expect("Failed to register the service");

    std::thread::sleep(std::time::Duration::from_secs(60));
    mdns.shutdown().unwrap();
}
