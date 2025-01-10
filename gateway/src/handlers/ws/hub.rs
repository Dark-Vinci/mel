use {
    crate::{
        app::AppInterface,
        handlers::ws::client::{Client, MessageType},
    },
    axum::extract::ws::WebSocket,
    sdk::{
        constants::constant::WS_CHANNEL,
        utils::{
            redis::{MyRedis, RedisInterface},
            utility::deserialize,
        },
    },
    serde::Deserialize,
    std::collections::HashMap,
    tokio::sync::{broadcast, mpsc, Mutex},
    uuid::Uuid,
};

#[derive(Debug)]
pub struct Hub<'a> {
    pub users: HashMap<String, &'a Client>, // userid, client
    pub app: Box<dyn AppInterface>,
    pub broadcast: broadcast::Sender<MessageType>,
    pub broadcast_receiver: broadcast::Receiver<MessageType>,
    pub redis: Box<dyn RedisInterface>,
    pub server_name: Uuid,
    pub client_listener_sender: mpsc::Sender<MessageType>,
    pub client_listener_receiver: mpsc::Receiver<MessageType>,
}

impl Hub {
    pub fn new(red: MyRedis, app: Box<dyn AppInterface>) -> Self {
        let (tx_message, mut rx_message) = broadcast::channel(10000);
        let (abc, mut bca) = mpsc::channel(10000);

        let hub = Self {
            users: HashMap::new(),
            redis: Box::new(red),
            server_name: Uuid::new_v4(),
            broadcast: tx_message,
            broadcast_receiver: rx_message,
            client_listener_receiver: bca,
            client_listener_sender: abc,
            app,
        };

        tokio::spawn(Box::pin(async move {
            hub.subscribe().await;
        }));

        hub
    }

    pub async fn register_client(&mut self, socket: WebSocket, id: Uuid) {
        let mut client = Client::new(
            socket,
            id,
            self.client_listener_sender.clone(),
            self.broadcast_receiver.resubscribe(),
        );

        // insert into memory DB
        self.users.insert(client.user_id.to_string(), &client);

        let server_name = self.server_name.clone().to_string();

        // insert into redis
        self.redis
            .set_value(id.to_string(), server_name)
            .await
            .unwrap();

        // start reading message from the client
        tokio::spawn(async {
            client.read_pump().await;
        });

        // start writing message to the client
        tokio::spawn(async {
            client.write_pump().await;
        });
    }

    async fn subscribe(&self) {
        let (a, mut b) = mpsc::channel(1000);

        self.redis.subscribe(a, WS_CHANNEL).await;

        while let Some(msg) = b.recv().await {
            let message = deserialize::<MessageType>(msg);
            self.broadcast.send(message).await;
        }
    }

    async fn remove_client(&mut self, id: &Uuid) {
        // remove from connection hub
        self.users.remove(&id.to_string());

        // remove from redis //todo: implement exponential backoff
        self.redis.delete(id.to_string())
    }
}
