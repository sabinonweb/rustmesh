use crate::discovery::events::RustMeshDiscoveryEvent;

pub mod ble;
pub mod events;
pub mod wifi;

pub trait RustMeshDiscovery {
    fn peer_discovered(&mut self, event: RustMeshDiscoveryEvent);
    fn peer_expired();
}
