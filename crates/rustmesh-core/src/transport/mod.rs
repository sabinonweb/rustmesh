pub mod ble;
pub mod quic;

use crate::error::RustMeshError;

pub trait Transport {
    fn listen(&mut self, addr: &str) -> Result<(), RustMeshError>;
    fn dial(&mut self, addr: &str) -> Result<(), RustMeshError>;
}
