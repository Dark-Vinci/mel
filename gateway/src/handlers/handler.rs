use {
    crate::{
        app::{App, AppInterface},
        handlers::{api, ws::ws::WebsocketHandler},
    },
    axum::{http::Method, Router},
    tower_http::cors::{Any, CorsLayer},
};

#[derive(Clone)]
pub struct AppState {
    pub app: Box<dyn AppInterface>,
}

impl AppState {
    fn new(app: Box<dyn AppInterface>) -> Self {
        Self { app }
    }
}

pub struct Handlers;

impl Handlers {
    pub fn build(app: App) -> Router {
        let state = AppState::new(Box::new(app.clone()));

        let rest = api::api::endpoints(&state);
        let mut real_time = WebsocketHandler::new(Box::new(app), (), state);

        let cors = CorsLayer::new()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
            ])
            .allow_origin(Any)
            .expose_headers(Any);

        Router::new()
            .nest("/api", rest)
            .nest("/ws", real_time.build())
            // .route_layer(from_extractor::<RequestID>())
            .layer(cors)
    }
}
