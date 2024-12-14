use axum::extract::State;
use axum::response::IntoResponse;
use axum::Router;
use axum::routing::get;
use crate::handlers::AppState;

pub fn router(state: &AppState) -> Router {
    Router::new()
        .route("/login", get(login))
        .route("/sign-up", get(signup))
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