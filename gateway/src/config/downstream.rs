use std::env;
use sdk::constants::constant::{ACCOUNT_URL, MESSAGE_URL};

#[derive(Debug, Clone)]
pub struct Downstream {
    pub account_url: String,
    pub message_url: String,
}

impl Downstream {
    pub fn new() -> Self {
        Self {
            account_url: env::var(ACCOUNT_URL).unwrap_or_default(),
            message_url: env::var(MESSAGE_URL).unwrap_or_default(),
        }
    }
}
