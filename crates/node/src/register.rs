use core::identity::{Identity, Peer};
use mdns_sd::{ServiceDaemon, ServiceInfo};

pub fn register_service() -> Identity {
    let mdns = ServiceDaemon::new().expect("Failed to create a daemon");
    let service_type = "_mdns-sd-my-test._udp.local.";
    let peer_id = Identity::generate();
    let ip = "127.0.0.1"; // localhost
    let host_name = "localhost.local.";
    let port = 8080;
    let peer = Peer {
        id: peer_id,
        ip,
        port,
    };
    let properties = [
        ("peer", serde_json::to_string(&peer).unwrap()),
        ("property_1", "test".to_string()),
        ("property_2", "1234".to_string()),
    ];

    let my_service = ServiceInfo::new(
        service_type,
        &peer_id.encode()[..10],
        host_name,
        ip,
        port,
        &properties[..],
    )
    .unwrap();

    mdns.register(my_service)
        .expect("Failed to register the service");

    std::thread::sleep(std::time::Duration::from_secs(10));

    peer_id
}
