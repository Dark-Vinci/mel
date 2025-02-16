use uuid::Uuid;

pub mod app;
pub mod config;
mod downstream;
pub mod errors;
pub mod handlers;
mod middleware;
mod models;

// struct CTX<'a> {
//     user_agent: &'a str,
//     auth_token: Option<&'a str>,
//     refresh_token: Option<&'a str>,
//     user_id: Option<Uuid>,
//     time_zone: &'a str,
// }
