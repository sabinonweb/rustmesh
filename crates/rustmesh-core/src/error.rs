use thiserror::Error;

#[derive(Error, Debug)]
pub enum RustMeshError {
    #[error("IO Error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization Error: {0}")]
    Serialization(String),

    #[error("Deserialization Error: {0}")]
    Deserialization(String),

    #[error("libp2p Error: {0}")]
    LibP2P(String),

    #[error("Channel Error: {0}")]
    Channel(String),

    #[error("Timeout")]
    Timeout,

    #[error("Configuration Error: {0}")]
    Configuration(String),

    #[error("Not Found Error: {0}")]
    NotFound(String),

    #[error("Invalid State Error: {0}")]
    InvalidState(String),
}

pub type Result<T> = std::result::Result<T, RustMeshError>;

#[cfg(test)]
mod tests {
    use super::*;

    fn error_test() {
        let err = RustMeshError::Timeout;
        assert_eq!(err.to_string(), "Timeout");
    }
}
