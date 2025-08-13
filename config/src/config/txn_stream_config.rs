use serde::{Deserialize, Serialize};

use super::indexer_grpc_config::DEFAULT_GRPC_STREAM_PORT;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TxnStreamConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub port: u16,
}

impl Default for TxnStreamConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            bind_address: "127.0.0.1".into(),
            port: DEFAULT_GRPC_STREAM_PORT,
        }
    }
}
