use btleplug::api::{Central, Manager as _, Peripheral, ScanFilter};
use btleplug::platform::Manager;
use rustmesh::link::{wifi::WifiLink, Link};
use std::error::Error;
use std::time::Duration;
use tokio::net::UdpSocket;
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // let binding_addr = "192.168.110.134:8080";
    // let remote_addr = "192.168.110.18:8081";
    // let wifi_link = WifiLink::new(binding_addr, remote_addr).await.unwrap();
    //
    // wifi_link
    //     .send(b"sabinonwebbabybyby from macbook babby come on kr$na")
    //     .await
    //     .unwrap();
    // println!("Message Sent!");
    //
    // if let Some(data) = wifi_link.recv().await.unwrap() {
    //     println!("Received: {:?}", String::from_utf8_lossy(&data));
    // }

    let manager = Manager::new().await.unwrap();
    let adapter = manager.adapters().await?;
    let central = adapter.into_iter().nth(0).unwrap();
    central.start_scan(ScanFilter::default()).await?;
    time::sleep(Duration::from_secs(2)).await;

    for p in central.peripherals().await.unwrap() {
        if let Some(name) = p.properties().await.unwrap() {
            println!("Local name: {:?}", name.local_name);
            println!("Hello: {:?}", name.service_data);
            let data = name.manufacturer_data;
            println!("BDADDR: {:?}", name.address);
            for (uuid, val) in data {
                println!("UUID: {:?}", uuid);
                println!("Val: {:?}", val);
                // let res = String::from_utf8(val).unwrap();
                // println!("Data: {:?}", res);
            }
        }
    }

    Ok(())
}
