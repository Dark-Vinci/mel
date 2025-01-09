use {
    crate::handlers::{ws::hub::Hub, AppState},
    axum::{
        extract::{
            ws::{Message, WebSocket},
            State, WebSocketUpgrade,
        },
        response::IntoResponse,
        routing::any,
        Router,
    },
    futures_util::{SinkExt, StreamExt},
    serde::{Deserialize, Serialize},
    std::collections::HashMap,
    tokio::sync::broadcast,
    uuid::Uuid,
};

#[derive(Debug, Clone)]
enum Actions {
    Scream,
    Chat,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct UserMessage {
    user_id: String,
    to_user: String,
    content: String,
    action: Actions,
}

pub fn handler(state: &AppState) -> Router {
    Router::new().route("/", any(ws_handler)).with_state(state)
}

pub struct WebsocketHandler {
    hub: Hub,
}

impl WebsocketHandler {
    pub fn new() -> Self {
        Self { hub: Hub::new() }
    }

    // async fn ws_handler(&self, ws: WebSocketUpgrade, State(app): State<AppState>) -> impl IntoResponse {
    //
    //     let a = self.c
    //
    //     ws.on_upgrade(move |socket| {
    //
    //     })
    // }

    async fn handle_message(&mut self, ws: WebSocket) {
        self.hub.register_client(ws, Uuid::new_v4()).await;
    }
}
