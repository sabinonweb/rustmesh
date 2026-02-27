use crate::add_peer;
use core::identity::{Identity, Peer, PeerTable};
use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

pub fn discover_services() -> Result<Peer, String> {
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");
    let my_id = Identity::generate();
    let service_type = "_mdns-sd-my-test._udp.local.";
    let receiver = mdns.browse(service_type).expect("Failed to browse");
    let result: Arc<Mutex<Option<Result<Peer, String>>>> = Arc::new(Mutex::new(None));
    let result_clone = Arc::clone(&result);

    std::thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            if let ServiceEvent::ServiceResolved(resolved) = event {
                println!("Resolved a full service: {}", resolved.fullname);
                let peer: Peer = resolved.get_property_val_str("peer").unwrap().into();
                println!("Service resolved from the peer: {:?}\n", peer);
                let peer_identity = Identity::identity(peer.id.to_string());

                if peer_identity == my_id {
                    continue;
                }

                add_peer(peer);

                if let Some(ip) = resolved.get_addresses().iter().next() {
                    let mut result = result_clone.lock().unwrap();
                    *result = Some(Ok(Peer {
                        id: my_id.clone().encode(),
                        ip: ip.to_string(),
                        port: resolved.port,
                        connection: None,
                    }));
                    break; // stop after first resolved peer
                }
            } else {
                println!("Ignoring event: {:?}", event);
            }
        }
    });
    std::thread::sleep(Duration::from_secs(10));

    match result.lock().unwrap().clone() {
        Some(p) => Ok(p?),
        None => {
            println!("No service received!");
            Err(format!("No service received!"))
        }
    }
}
