use axum::extract::State;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::{get, post};
use crate::handlers::AppState;

pub fn router(state: &AppState) -> Router {
    Router::new()
        .route("/show", get(login))
        .route("/get", post(signup))
        .with_state(state)
}

fn signup(
    State(app): State<AppState>
) -> impl IntoResponse {

}

fn login(
    State(app): State<AppState>
) -> impl IntoResponse {

}