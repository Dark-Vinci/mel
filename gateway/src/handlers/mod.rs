use axum::Router;
use crate::app;

pub mod ws;
pub mod api;

pub struct AppState {
    pub ws_hub: ws::Hub,
    pub app: app::App,
}

impl AppState {
    fn new() -> Self {
        Self{
            ws_hub: ws::Hub::new(),
            app: Default::default(),
        }
    }
}

fn handlers() -> Router {
    let state = AppState::new();

    Router::new()
        .nest("/api", api::endpoints(&state))
        .nest("/ws", ws::handler(&state))
}
