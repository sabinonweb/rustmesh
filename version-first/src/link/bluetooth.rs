use btleplug::api::{Central as _, Manager as _, Peripheral as _};
use btleplug::platform::{Manager, Peripheral};
use uuid::Uuid;

use super::Link;

pub struct BLE {
    peripheral: Peripheral,
    peripheral_rx: Uuid,
}

#[derive(Debug)]
pub enum BLEError {
    BindingError(String),
}

impl BLE {
    pub async fn init(uuid: &str) -> Result<Self, BLEError> {
        let manager = Manager::new().await.unwrap();
        let adapters = manager.adapters().await.unwrap();
        let central = adapters.iter().nth(0).unwrap();
        let peripheral = central
            .start_scan(btleplug::api::ScanFilter::default())
            .await
            .unwrap();
        let peripherals = central.peripherals().await.unwrap();

        for peripheral in peripherals {
            peripheral.discover_services().await;
            let services = peripheral.services().iter().map(|service| service.uuid);
        }

        Ok(Self {
            peripheral: central.peripherals(),
        })
    }
}

#[async_trait::async_trait]
impl Link for BLE {
    async fn send(&self, data: &[u8]) -> Result<(), super::LinkError> {}

    async fn recv(&self) -> Result<Option<Vec<u8>>, super::LinkError> {}
}
