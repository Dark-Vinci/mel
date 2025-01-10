use {
    crate::{
        app::interfaces::AppInterface,
        handlers::{handler::AppState, ws::hub::Hub},
    },
    axum::{
        extract::{ws::WebSocket, WebSocketUpgrade},
        response::IntoResponse,
        routing::any,
        Router,
    },
    sdk::utils::redis::MyRedis,
    uuid::Uuid,
};

pub struct WebsocketHandler<'a> {
    hub: Hub<'a>,
    state: AppState,
}

impl<'a> WebsocketHandler<'a> {
    pub fn new(
        app: Box<dyn AppInterface>,
        red: MyRedis,
        state: AppState,
    ) -> Self {
        Self {
            hub: Hub::new(red, app),
            state,
        }
    }

    fn w_handler(&mut self, ws: WebSocketUpgrade) -> impl IntoResponse {
        ws.on_upgrade(|ws| {
            async move {
                self.handle_message(ws).await;
            }
        })
    }

    pub fn build(&mut self) -> Router {
        Router::new()
            .route("/", any(Self::w_handler).with_state(self.state.clone()))
    }

    async fn handle_message(&mut self, ws: WebSocket) {
        self.hub.register_client(ws, Uuid::new_v4()).await;
    }
}
