use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub enum Message {
    Publish {
        topic: String,
        data: Vec<u8>,
        #[serde(default)]
        timestamp: u64,
    },

    Subscribe {
        topic: String,
    },

    Request {
        request_id: u32,
        method: String,
        params: Vec<u8>,
    },

    Response {
        request_id: u32,
        result: Vec<u8>,
        error: Option<String>,
    },

    Benchmark {
        id: u32,
        timestamp: u64,
        payload: Vec<u8>,
    },
}

impl Message {
    pub fn to_bytes(&self) -> crate::Result<Vec<u8>> {
        wincode::serialize(self).map_err(|e| crate::RustMeshError::Serialization(e.to_string()))
    }

    pub fn from_bytes(data: &[u8]) -> crate::Result<Self> {
        wincode::deserialize(data).map_err(|e| crate::RustMeshError::Deserialization(e.to_string()))
    }

    pub fn type_name(&self) -> &str {
        match self {
            Message::Publish { .. } => "Publish",
            Message::Subscribe { .. } => "Subscribe",
            Message::Request { .. } => "Request",
            Message::Response { .. } => "Response",
            Message::Benchmark { .. } => "Benchmark",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_serialization() {
        let msg = Message::Publish {
            topic: "test".to_string(),
            data: vec![1, 2, 3],
            timestamp: 123456789,
        };

        let result = msg.to_bytes().unwrap();
        let deserialized = Message::from_bytes(&bytes).unwrap();
        
        match deserialized {
            Message::Publish { topic, data, timestamp } => {
                assert_eq!(topic, "test");
                assert_eq!(data, vec![1, 2, 3]);
                assert_eq!(timestamp, 123456789);
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test] 
    fn test_message_name() {
        let msg = Message::Publish {
            topic: "test".to_string(),
            data: vec![1, 2, 3],
            timestamp: 123456789,
        };

        assert_eq!(msg.type_name(), "Publish");
    }
