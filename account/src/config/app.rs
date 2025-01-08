#[derive(Debug, Clone)]
pub struct AppConfig {
    pub service_name: String,
    pub app_name: String,
    pub port: String,
}

impl AppConfig {
    pub fn new() -> Self {
        Self {
            service_name: "".into(),
            app_name: "".into(),
            port: "3000".into(),
        }
    }
}
