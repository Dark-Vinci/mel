#[derive(Debug)]
pub struct DBConfig {
    pub password: String,
    pub username: String,
    pub host: String,
    pub port: String,
    pub name: String,
}

impl DBConfig {
    pub fn new() -> Self {
        Self {
            password: "".into(),
            username: "".into(),
            host: "".into(),
            port: "".into(),
            name: "".into(),
        }
    }
}
