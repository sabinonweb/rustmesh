use libp2p::mdns;

pub enum RustMeshDiscoveryEvent {
    MDNSEvent(mdns::Event),
    BLEEvent,
}

impl From<mdns::Event> for RustMeshDiscoveryEvent {
    fn from(value: mdns::Event) -> Self {
        RustMeshDiscoveryEvent::MDNSEvent(value)
    }
}
