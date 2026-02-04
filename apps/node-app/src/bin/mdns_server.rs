use mdns_sd::{ServiceDaemon, ServiceInfo};

fn main() {
    let mdns = ServiceDaemon::new().expect("Failed to create a daemon");
    let service_type = "_mdns-sd-my-test._udp.local.";
    let instance_name = "my_instance";
    let ip = "127.0.0.1"; // localhost
    let host_name = "localhost.local.";
    let port = 5200;
    let properties = [("property_1", "test"), ("property_2", "1234")];

    let my_service = ServiceInfo::new(
        service_type,
        instance_name,
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
