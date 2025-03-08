use {
    crate::handlers::{
        api::{auth, messages},
        handler::AppState,
    },
    axum::Router,
};
use crate::handlers::api::media;

pub fn endpoints(state: AppState) -> Router {
    Router::new()
        .nest("/messages", auth::router(state.clone()))
        .nest("/auth", messages::router(state))
        .nest("/media", media::router())
}
