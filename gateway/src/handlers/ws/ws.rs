use {
    crate::handlers::ws::hub::Hub,
    axum::{
        extract::{State, WebSocketUpgrade},
        response::IntoResponse,
        routing::get,
        Router,
    },
    uuid::Uuid,
};

pub fn build(state: Hub) -> Router {
    Router::new()
        .route("/", get(websocket_handler))
        .with_state(state)
}

async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(mut hub): State<Hub>,
) -> impl IntoResponse {
    ws.on_upgrade(move |ws| {
        async move {
            let client_id = Uuid::new_v4();

            tokio::spawn(Box::pin(async move {
                let _ = hub.register_client(ws, client_id).await;
            }));
        }
    })
}
