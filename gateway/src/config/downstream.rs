use {
    sdk::constants::constant::{ACCOUNT_URL, MESSAGE_URL},
    std::env,
};

#[derive(Debug, Clone)]
pub struct Downstream {
    pub account_grpc_address: String,
    pub channel_grpc_address: String,
    pub extras_grpc_address: String,
    pub messaging_grpc_address: String,
}

impl Downstream {
    pub fn new() -> Self {
        Self {
            account_grpc_address: env::var(ACCOUNT_URL).unwrap_or_default(),
            channel_grpc_address: env::var(MESSAGE_URL).unwrap_or_default(),
            extras_grpc_address: env::var(ACCOUNT_URL).unwrap_or_default(),
            messaging_grpc_address: env::var(MESSAGE_URL).unwrap_or_default(),
        }
    }
}
