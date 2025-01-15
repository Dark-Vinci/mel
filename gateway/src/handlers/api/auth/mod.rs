use {
    crate::handlers::handler::AppState,
    axum::{routing::get, Router},
};

pub fn router(_state: AppState) -> Router {
    Router::new()
        .route("/login", get(|| async { "login failed" }))
        .route("/sign-up", get(|| async { "sign-up failed" }))
    // .with_state(state)
}
