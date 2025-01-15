use {
    crate::handlers::handler::AppState,
    axum::{
        routing::{get, post},
        Router,
    },
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/show", get(|| async { "show" }))
        .route("/get", post(|| async { "get" }))
        .with_state(state)
}
