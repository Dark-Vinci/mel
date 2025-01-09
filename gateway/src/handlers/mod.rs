use crate::app::App;

pub mod api;
pub mod handler;
pub mod ws;

pub struct AppState {
    pub app: App,
}
