use super::{Link, LinkError};
use tokio::net::UdpSocket;

#[derive(Debug)]
pub enum WifiLinkError {
    BindingError(String),
}

#[derive(Debug)]
pub struct WifiLink {
    socket: UdpSocket,
    remote_addr: String,
}

impl WifiLink {
    pub async fn new(binding_adr: &str, remote_addr: &str) -> Result<Self, WifiLinkError> {
        let socket = UdpSocket::bind(binding_adr)
            .await
            .map_err(|e| WifiLinkError::BindingError(format!("Binding Error: {}", e)))?;

        Ok(Self {
            socket,
            remote_addr: remote_addr.to_string(),
        })
    }
}

#[async_trait::async_trait]
impl Link for WifiLink {
    async fn send(&self, data: &[u8]) -> Result<(), LinkError> {
        match self.socket.send_to(data, &self.remote_addr).await {
            Ok(_) => Ok(()),
            Err(e) => Err(LinkError::SendError(format!(
                "Error occured while sending data: {}",
                e
            ))),
        }
    }

    async fn recv(&self) -> Result<Option<Vec<u8>>, LinkError> {
        let mut buf = vec![0u8; 1024];

        let len = self
            .socket
            .recv(&mut buf)
            .await
            .map_err(|e| LinkError::RecvError(format!("Recv error: {}", e)))?;
        Ok(Some(buf[..len].to_vec()))
    }
}
