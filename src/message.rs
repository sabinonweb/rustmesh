use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Message {
    Publish {
        topic: String,
        data: Vec<u8>,
        #[serde(default)]
        timestamp: u64,
    },

    Subcribe {
        topic: String,
    },

    Request {request_id
        request_id: u64,
        method: String,
        params: Vec<u8>,
    },

    Response {
        request_id: u64,
        result: Vec<u8>,
        error: Option<String>,
    },

    Benchmark {
        id: u32,
        timestamp: u64, 
        payload: Vec<u8>
    }
}

impl Message {
   pub fn to_bytes(&self) -> crate 
}
