use {
    crate::{
        app::App,
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
    serde_json::json,
    std::collections::HashMap,
    tokio::sync::{broadcast, mpsc, Mutex},
    uuid::Uuid,
};

#[derive(Debug)]
pub struct Hub {
    pub users: Mutex<HashMap<String, Client>>, // userid, client
    pub app: App,
    pub broadcast: broadcast::Sender<MessageType>,
    pub broadcast_receiver: broadcast::Receiver<MessageType>,
    pub register: mpsc::Receiver<Client>,
    pub register_sender: mpsc::Sender<Client>,
    pub unregister: mpsc::Receiver<Client>, //change to uuid string
    pub unregister_sender: mpsc::Sender<Client>,
    pub redis: Box<dyn RedisInterface>,
    pub server_name: Uuid,
    pub client_listener_sender: mpsc::Sender<MessageType>,
    pub client_listener_receiver: mpsc::Receiver<MessageType>,
}

impl Hub {
    pub fn new(red: MyRedis, app: App) -> Self {
        let (tx_register, mut rx_register) = mpsc::channel(100);
        let (tx_un_register, mut rx_un_register) = mpsc::channel(100);
        let (tx_message, mut rx_message) =
            broadcast::channel::<MessageType>(10000);
        let (abc, mut bca) = mpsc::channel(10000);

        let n = Self {
            users: Mutex::new(HashMap::new()),
            redis: Box::new(red),
            server_name: Uuid::new_v4(),
            register: rx_register,
            register_sender: tx_register,
            unregister: rx_un_register,
            unregister_sender: tx_un_register,
            broadcast: tx_message,
            broadcast_receiver: rx_message,
            client_listener_receiver: bca,
            client_listener_sender: abc,
            app,
        };

        tokio::spawn(Box::pin(async move {
            n.subscribe().await;
        }));

        n
    }

    pub async fn register_client(&mut self, socket: WebSocket, id: Uuid) {
        let mut client = Client::new(
            socket,
            id,
            self.client_listener_sender.clone(),
            self.broadcast_receiver.resubscribe(),
        );

        // insert into memory DB
        self.users
            .lock()
            .await
            .insert(client.user_id.to_string(), client);

        let server_name = self.server_name.clone().to_string();

        // insert into redis
        self.redis
            .set_value(id.to_string(), server_name)
            .await
            .unwrap();

        // start reading message from the client
        tokio::spawn(async move {
            client.read_pump().await;
        })
        .await
        .expect("TODO: panic message");

        // start writing message to the client
        tokio::spawn(async move {
            client.write_pump().await;
        })
        .await
        .expect("TODO: panic message");
    }

    async fn subscribe(&self) {
        let (a, mut b) = mpsc::channel(1000);

        self.redis.subscribe(a, WS_CHANNEL).await;

        while let Some(msg) = b.recv().await {
            let message = deserialize::<MessageType>(msg);
            self.broadcast.send(message).await;
        }
    }

    async fn remove_client(&self, id: &Uuid) {
        // remove from connection hub
        self.users.lock().await.remove(&id.to_string());

        // remove from redis //todo: implement exponential backoff
        self.redis.delete(id.to_string())
    }
}
