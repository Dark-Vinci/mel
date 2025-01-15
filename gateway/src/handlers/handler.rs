use {
    crate::{
        app::{app::App, interfaces::AppInterface},
        handlers::{api},
        middleware::{
            request_id::RequestID,
            request_response::handle_print_request_response,
        },
    },
    axum::{
        http::Method,
        middleware::{from_extractor, from_fn},
        Router,
    },
    std::{sync::Arc, time::Duration},
    tower::ServiceBuilder,
    tower_http::{
        compression::CompressionLayer,
        cors::{Any, CorsLayer},
        decompression::RequestDecompressionLayer,
        timeout::TimeoutLayer,
    },
};

#[derive(Clone)]
pub struct AppState {
    pub app: Arc<dyn AppInterface>,
}

impl AppState {
    fn new(app: App) -> Self {
        Self { app: Arc::new(app) }
    }
}

pub struct Handlers;

impl Handlers {
    pub async fn build(app: App) -> Result<Router, String> {
        let state = AppState::new(app);

        let rest = api::api::endpoints(state);

        // let redid = MyRedis::new("", "", "", "", "").await;
        // let mut real_time = WebsocketHandler::new(app, redid).await?;

        let cors = CorsLayer::new()
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::DELETE,
            ])
            .allow_origin(Any)
            .expose_headers(Any);

        Ok(Router::new()
            .nest("/api", rest)
            // .nest("/ws", real_time.build())
            .route_layer(from_extractor::<RequestID>())
            .layer(
                ServiceBuilder::new()
                    .layer(RequestDecompressionLayer::new())
                    .layer(CompressionLayer::new()),
            )
            .layer(from_fn(handle_print_request_response))
            .layer(TimeoutLayer::new(Duration::from_secs(10)))
            .layer(cors))
    }
}
