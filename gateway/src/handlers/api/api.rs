use {
    crate::handlers::{
        api::{auth, messages},
        handler::AppState,
    },
    axum::Router,
};

pub fn endpoints(state: AppState) -> Router {
    Router::new()
        .nest("/messages", auth::router(state.clone()))
        .nest("/auth", messages::router(state))
}
