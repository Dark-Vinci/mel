use {
    crate::handlers::{
        api::{auth, media::media, messages},
        handler::AppState,
    },
    axum::Router,
};

pub fn endpoints(state: AppState) -> Router {
    Router::new()
        .nest("/messages", auth::router(state.clone()))
        .nest("/auth", messages::router(state.clone()))
        .nest("/media", media::router(state))
}
