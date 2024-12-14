use fred::clients::RedisClient;
use fred::interfaces::{EventInterface, KeysInterface, PubsubInterface};
use fred::types::{RedisConfig};
use tokio::sync::mpsc::{Sender};

pub struct MyRedis {
    publisher: RedisClient,
    subscriber: RedisClient,
    others: RedisClient,
    channel: String,
}

pub trait MyRedisImpl {
    async fn subscribe(&self, value: &[u8]) -> String;
    async fn publish(&self, message: String);
    async fn get_value(&self, key: String) -> &[u8];
    async fn set_value(&self, key: String, message: &[u8]);
}

impl MyRedis {
    pub fn new(url: String, channel: String) -> Self {
        let config = RedisConfig::from_url(&url).unwrap();

        let publ = RedisClient::new(config.clone(), None, None, None); // todo:: add tomer
        let subs = RedisClient::new(config.clone(), None, None, None);
        let others = RedisClient::new(config, None, None, None);

        Self {
            publisher: publ,
            subscriber: subs,
            others,
            channel,
        }
    }
}

impl MyRedisImpl for MyRedis {
    async fn subscribe(&self, sender: Sender<Vec<u8>>) {
        self.subscriber.on_message(async |sub| {
            if self.channel == sub.channel {
                let _ = sender.send(sub.value.as_bytes().unwrap().to_vec()).await.unwrap();
            }

            return Ok(())
        });
    }

    async fn publish(&self, message: String) -> Result<String, String> {
        if let Err(err) = self.publisher.publish(&self.channel, message).await {
            println!("Failed to publish message: {}", err);
            return Err("Failed to publish message".to_string());
        }

        Ok("success".to_string())
    }

    async fn get_value(&self, key: String) -> String {
        let result = self.others.get(key.as_bytes()).await;

        if let Err(err) = result {
            println!("Failed to get value: {}", err);
            return err.to_string();
        }

        result.unwrap()
    }

    async fn set_value(&self, key: String, message: String) -> Result<(), String> {
        let result: Result((), String) = self.others.set(key.as_bytes(), message.as_bytes(), None, None, false).await;

        if let Err(err) = result {
            println!("Failed to get value: {}", err);
            return Err("failed".into());
        }

        Ok("okay".into())
    }
}