use {
    crate::handlers::handler::AppState,
    axum::{extract::State, response::IntoResponse, routing::get, Router},
};

pub fn router(state: &AppState) -> Router {
    Router::new()
        .route("/login", get(login))
        .route("/sign-up", get(signup))
        .with_state(state)
}

fn signup(State(app): State<AppState>) -> impl IntoResponse {}

fn login(State(app): State<AppState>) -> impl IntoResponse {}
