use std::env;
use sdk::constants::constant::PORT;

#[derive(Debug, Clone)]
pub struct App {
    pub port: String,
}

impl App {
    pub fn new() -> App {
        Self {
            port: env::var(PORT).unwrap_or_else(|_| String::from("8080")),
        }
    }
}
