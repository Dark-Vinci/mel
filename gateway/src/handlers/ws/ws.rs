use {
    crate::{
        app::{app::App},
        handlers::{ws::hub::Hub},
    },
    axum::{
        extract::{ws::WebSocket, WebSocketUpgrade},
        response::IntoResponse,
        routing::get,
        Router,
    },
    sdk::utils::redis::MyRedis,
    std::sync::{Arc, Mutex},
    uuid::Uuid,
};

pub struct WebsocketHandler {
    hub: Arc<Mutex<Hub>>,
}

impl WebsocketHandler {
    pub async fn new(app: App, red: MyRedis) -> Result<Self, String> {
        let hub = Hub::new(red, app).await?;

        Ok(Self {
            hub: Arc::new(Mutex::new(hub)),
        })
    }

    fn w_handler(self: Arc<Self>, ws: WebSocketUpgrade) -> impl IntoResponse {
        ws.on_upgrade(move |ws| {
            let mut handler = self.clone();
            async move {
                handler.handle_message(ws).await;
            }
        })
    }

    pub async fn build(self: Arc<Self>) -> Router {
        Router::new().route("/", get(move |ws| self.clone().w_handler(ws)))
    }

    async fn handle_message(self: Arc<Self>, ws: WebSocket) {
        let client_id = Uuid::new_v4();
        let hub = self.hub.clone();

        tokio::spawn(async move {
            let mut hub = hub.lock().unwrap();
            let _ = hub.register_client(ws, client_id).await;
        });
    }
}
