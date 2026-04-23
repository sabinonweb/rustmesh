use btleplug::api::CentralEvent;
use btleplug::api::{
    bleuuid::uuid_from_u16, Central, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::Manager;
use futures::StreamExt;
use std::error::Error;
use std::time::Duration;
use wincode::containers::Box;

#[tokio::main]
async fn main() {
    let manager = Manager::new().await.unwrap();

    // central of the BLE
    let adapters = manager.adapters().await.expect("Couldn't find the central");
    let central = adapters.into_iter().nth(0).unwrap();

    central.start_scan(ScanFilter::default()).await.unwrap();

    tokio::time::sleep(Duration::from_secs(2)).await;
    let mut events = central.events().await.unwrap();

    while let Some(event) = events.next().await {
        match event {
            CentralEvent::DeviceDiscovered(id) => {
                let peripheral = central.peripheral(&id).await.unwrap();
                let properties = peripheral.properties().await.unwrap();
                let name = properties
                    .and_then(|p| p.local_name)
                    .map(|local_name| format!("Name: {local_name}"))
                    .unwrap_or_default();

                println!("DeviceDiscovered: {:?} Name: {}", id, name);
            }

            CentralEvent::DeviceConnected(id) => {
                println!("DeviceConnected: {:?}", id);
            }

            _ => {}
        }
    }
}
