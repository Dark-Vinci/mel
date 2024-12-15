#[derive(Debug)]
pub struct KafkaConfig {
    pub url: String,
    pub topic: String,
    pub consumer_group: String,
    pub broker: String,
    pub group_id: String,
    pub username: String,
    pub password: String,
    pub port: String,
    pub host: String,
}
