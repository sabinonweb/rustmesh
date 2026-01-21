pub mod bluetooth;
pub mod wifi;

#[derive(Debug)]
pub enum LinkError {
    SendError(String),
    RecvError(String),
}

#[async_trait::async_trait]
pub trait Link {
    async fn send(&self, data: &[u8]) -> Result<(), LinkError>;
    async fn recv(&self) -> Result<Option<Vec<u8>>, LinkError>;
}
