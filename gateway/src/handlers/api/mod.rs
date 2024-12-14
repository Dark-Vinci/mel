use axum::Router;
// use crate::app::App;
use crate::handlers::AppState;

mod messages;
mod auth;

pub fn endpoints(state: &AppState) -> Router {
    Router::new()
        .nest("/messages", auth::router(state))
        .nest("/auth", messages::router(state))
}
