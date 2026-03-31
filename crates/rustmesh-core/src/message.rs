use serde::{Deserialize, Serialize};
use wincode::{SchemaRead, SchemaWrite};

#[derive(Deserialize, Serialize, SchemaRead, SchemaWrite, Debug)]
pub struct PublishPayload {
    pub topic: String,
    pub data: Vec<u8>,
    #[serde(default)]
    pub timestamp: u64,
}

#[derive(Deserialize, Serialize, SchemaRead, SchemaWrite, Debug)]
pub struct SubscribePayload {
    pub topic: String,
}

#[derive(Deserialize, Serialize, SchemaRead, SchemaWrite, Debug)]
pub struct RequestPayload {
    pub request_id: u32,
    pub method: String,
    pub params: Vec<u8>,
}

#[derive(Deserialize, Serialize, SchemaRead, SchemaWrite, Debug)]
pub struct ResponsePayload {
    pub request_id: u32,
    pub result: Vec<u8>,
    pub error: String,
}

#[derive(Deserialize, Serialize, SchemaRead, SchemaWrite, Debug)]
pub struct BenchmarkPayload {
    pub id: u32,
    pub timestamp: u64,
    pub payload: Vec<u8>,
}

#[derive(Deserialize, Serialize, SchemaRead, SchemaWrite, Debug)]
pub enum Message {
    Publish(PublishPayload),
    Subscribe(SubscribePayload),
    Request(RequestPayload),
    Response(ResponsePayload),
    Benchmark(BenchmarkPayload),
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
        let msg = Message::Publish(PublishPayload {
            topic: "test".to_string(),
            data: vec![1, 2, 3],
            timestamp: 123456789,
        });

        let result = msg.to_bytes().unwrap();
        let deserialized = Message::from_bytes(&result).unwrap();

        match deserialized {
            Message::Publish(PublishPayload {
                topic,
                data,
                timestamp,
            }) => {
                assert_eq!(topic, "test");
                assert_eq!(data, vec![1, 2, 3]);
                assert_eq!(timestamp, 123456789);
            }
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_message_name() {
        let msg = Message::Publish(PublishPayload {
            topic: "test".to_string(),
            data: vec![1, 2, 3],
            timestamp: 123456789,
        });

        assert_eq!(msg.type_name(), "Publish");
    }
}
