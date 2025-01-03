use {
    crate::{handlers::AppState, middleware::{request_id::append_request_id, request_response::{self, handle_print_request_response}}},
    axum::{middleware::from_fn, Router},
};

pub fn endpoints(state: &AppState) -> Router {
    Router::new()
        .nest("/messages", auth::router(state))
        .nest("/auth", messages::router(state))
        .layer(from_fn(request_response::handle_print_request_response))
        .layer(from_fn(append_request_id))
}