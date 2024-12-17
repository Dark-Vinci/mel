use mockall::automock;
use tonic::async_trait;
use {
    redis::{AsyncCommands, Client},
    tokio::sync::mpsc::Sender,
};

pub struct MyRedis {
    client: Client,
}

#[async_trait]
pub trait RedisInterface {
    async fn subscribe(&self, sender: Sender<Vec<u8>>, channel: &str);
    async fn publish(&self, chan: String, message: String);
    async fn get_value(&self, key: String) -> &[u8];
    async fn set_value(
        &self,
        key: String,
        message: String,
    ) -> Result<(), String>;
}

impl MyRedis {
    pub async fn new(
        username: &str,
        password: &str,
        host: &str,
        port: &str,
        db: &str,
    ) -> Self {
        let connection_string = format!(
            "redis://{}:{}@{}:{}/{}",
            username, password, host, port, db
        );

        let mut conn = redis::Client::open(connection_string).unwrap();

        Self { client: conn }
    }
}

impl RedisInterface for MyRedis {
    async fn subscribe(&self, sender: Sender<Vec<u8>>, channel: &str) {
        let config =
            redis::AsyncConnectionConfig::new().set_push_sender(sender);
        let mut con = self
            .client
            .get_multiplexed_async_connection_with_config(&config)
            .await
            .unwrap();

        con.subscribe(channel).await.unwrap();
    }

    async fn publish(
        &self,
        chan: String,
        message: String,
    ) -> Result<(), String> {
        let mut con = self
            .client
            .get_multiplexed_async_connection()
            .await
            .unwrap();

        let res = con.publish(chan, message).await;

        if let Err(e) = res {
            return Err(e.to_string());
        }

        Ok(())
    }

    async fn get_value(&self, key: String) -> String {
        let mut result = self
            .client
            .get_multiplexed_async_connection()
            .await
            .unwrap();

        let res = result.get(key).await;

        if let Err(err) = res {
            println!("Failed to get value: {}", err);
            return err.to_string();
        }

        res.unwrap()
    }

    async fn set_value(
        &self,
        key: String,
        message: String,
    ) -> Result<(), String> {
        let mut result = self
            .client
            .get_multiplexed_async_connection()
            .await
            .unwrap();

        let res = result.set(key, message).await;

        if let Err(err) = result {
            println!("Failed to get value: {}", err);
            return Err("failed".into());
        }

        Ok(())
    }
}
