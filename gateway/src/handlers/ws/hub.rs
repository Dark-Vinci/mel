use {
    crate::{
        app::{app::App, interfaces::AppInterface},
        handlers::ws::client::{Client, MessageType},
    },
    axum::extract::ws::WebSocket,
    sdk::{
        constants::constant::WS_CHANNEL,
        utils::redis::{MyRedis, RedisInterface},
    },
    serde_json::json,
    std::{collections::HashMap, sync::Arc},
    tokio::sync::{broadcast, mpsc, Mutex},
    uuid::Uuid,
};

#[derive(Clone)]
pub struct Hub {
    pub users: Arc<Mutex<HashMap<Uuid, Client>>>, // userid, client
    pub app: Arc<dyn AppInterface>,
    pub broadcast_transmitter: broadcast::Sender<MessageType>,
    pub broadcast_receiver: Arc<broadcast::Receiver<MessageType>>,
    pub redis: Arc<dyn RedisInterface>,
    pub server_name: Uuid,
    pub client_listener_sender: mpsc::Sender<MessageType>,
    pub client_listener_receiver: Arc<mpsc::Receiver<MessageType>>,
}

impl Hub {
    pub async fn new(red: MyRedis, app: App) -> Result<Self, String> {
        let (broadcast_transmitter, broadcast_receiver) =
            broadcast::channel(10000);
        let (abc, bca) = mpsc::channel(10000);

        let this = Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            redis: Arc::new(red),
            server_name: Uuid::new_v4(),
            broadcast_transmitter,
            broadcast_receiver: Arc::new(broadcast_receiver),
            client_listener_receiver: Arc::new(bca),
            client_listener_sender: abc,
            app: Arc::new(app),
        };

        this.subscribe().await;

        Ok(this)
    }

    #[tracing::instrument(skip(self))]
    pub async fn register_client(&mut self, socket: WebSocket, id: Uuid) {
        tracing::info!("Got a request to register client with the server: {socket}, {id}");

        let mut client = Client::new(
            socket,
            id,
            self.client_listener_sender.clone(),
            self.broadcast_receiver.resubscribe(),
        );

        {
            let mut users = self.users.lock().await;
            client = users.insert(client.user_id, client).unwrap();
        }

        let server_name = &self.server_name.to_string();

        // insert into redis
        self.redis // user-id is connected to server-id
            .set_value(id.to_string(), server_name.into())
            .await
            .unwrap();

        tokio::spawn(async move {
            client.pumper().await;
        });
    }

    async fn subscribe(&self) {
        let (sender, mut receiver) = mpsc::channel(1000);

        let redis = self.redis.clone();
        let broadcast_transmitter = self.broadcast_transmitter.clone();

        redis.subscribe(sender, WS_CHANNEL).await;

        tokio::spawn(async move {
            while let Some(msg) = receiver.recv().await {
                let str = String::from_utf8(msg).unwrap();

                let j = json!(str);

                let message: MessageType = serde_json::from_value(j).unwrap();

                broadcast_transmitter
                    .send(message)
                    .expect("Failed to send message to broadcast transmitter");
            }
        });
    }

    async fn remove_client(&mut self, id: &Uuid) {
        // remove from connection hub
        {
            let mut users = self.users.lock().await;
            users.remove(id);
        }

        // remove from redis //todo: implement exponential backoff
        self.redis.delete(id.to_string()).await;
    }
}
