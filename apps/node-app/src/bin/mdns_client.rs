use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::time::Duration;

fn main() {
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    let service_type = "_mdns-sd-my-test._udp.local.";
    // searches for service type
    let receiver = mdns.browse(service_type).expect("Failed to browse");

    std::thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(resolved) => {
                    let ip = resolved.get_addresses().iter().next();
                    let port = resolved.port;
                    let peer_id = resolved.get_property_val_str("peer_id").unwrap();
                    println!("Resolved a full service: {}", resolved.fullname);
                }
                other_event => {
                    println!("Received other event: {:?}", &other_event);
                }
            }
        }
    });

    std::thread::sleep(Duration::from_secs(10));
    mdns.shutdown().unwrap();
}
