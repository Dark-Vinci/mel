use {
    crate::handlers::handler::AppState,
    axum::{
        extract::State,
        response::IntoResponse,
        routing::{get, post},
        Router,
    },
};

pub fn router(state: &AppState) -> Router {
    Router::new()
        .route("/show", get(login))
        .route("/get", post(signup))
        .with_state(state)
}

fn signup(State(app): State<AppState>) -> impl IntoResponse {}

fn login(State(app): State<AppState>) -> impl IntoResponse {}
