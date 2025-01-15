use tokio::task;
use tonic::codegen::tokio_stream::StreamExt;
use {
    redis::{AsyncCommands, Client, RedisResult},
    tokio::sync::mpsc::Sender,
    tonic::async_trait,
};

pub struct MyRedis {
    client: Client,
}

#[async_trait]
pub trait RedisInterface: Send + Sync {
    async fn subscribe(&self, sender: Sender<Vec<u8>>, channel: &str);
    async fn publish(
        &self,
        chan: String,
        message: String,
    ) -> Result<(), String>;
    async fn delete(&self, key: String) -> bool;
    async fn get_value(&self, key: String) -> Result<String, String>;
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

        let conn = Client::open(connection_string).unwrap();

        // let pool = r2d2::Pool::builder().build(conn).unwrap();

        // let mut r2_conn = pool.get().unwrap();

        Self { client: conn }
    }
}

#[async_trait]
impl RedisInterface for MyRedis {
    async fn subscribe(&self, sender: Sender<Vec<u8>>, channel: &str) {
        let mut pub_sub = self.client.get_async_pubsub().await.unwrap();

        pub_sub.subscribe(channel).await.expect("TODO: panic message");

        task::spawn(async move {
            let mut message_stream = pub_sub.on_message();

            while let Some(msg) = message_stream.next().await {
                let payload = msg.get_payload().unwrap();
                println!("Received message: {:?}", payload);
                sender.send(payload).await.unwrap();
            }
        });
    }

    async fn publish(
        &self,
        chan: String,
        message: String,
    ) -> Result<(), String> {
        let mut connection = self.client.get_multiplexed_async_connection().await.unwrap();

        let _result: usize = connection.publish(chan, message).await.unwrap();

        Ok(())
    }

    async fn delete(&self, key: String) -> bool {
        let mut result = self
            .client
            .get_multiplexed_async_connection()
            .await
            .unwrap();

        let res: RedisResult<()> = result.del(key).await;

        if let Err(_err) = res {
            return false;
        }

        true
    }

    async fn get_value(&self, key: String) -> Result<String, String> {
        let mut result = self
            .client
            .get_multiplexed_async_connection()
            .await
            .unwrap();

        let res = result.get(key).await;

        if let Err(err) = res {
            println!("Failed to get value: {}", err);
            return Err(err.to_string());
        }

        Ok(res.unwrap())
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

        let res: RedisResult<()> = result.set(key, message).await;

        if let Err(err) = res {
            println!("Failed to get value: {}", err);
            return Err(err.to_string());
        }

        Ok(())
    }
}
