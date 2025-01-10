use {
    gateway::{
        app::app::App, config::config::Config, handlers::handler::Handlers,
    },
    panic::set_hook,
    sdk::constants::constant::LOCAL_HOST,
    std::{net::SocketAddr, panic},
    tokio::net::TcpListener,
    tracing::level_filters::LevelFilter,
    tracing_subscriber::EnvFilter,
};

#[tokio::main]
async fn main() {
    let file_writer = debug_logger.and(warning_error_logger);

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()?
        .add_directive("auth=debug".parse()?);

    tracing_subscriber::fmt()
        .pretty()
        .json()
        .with_max_level(tracing::Level::TRACE)
        .with_writer(file_writer)
        .with_env_filter(filter)
        .with_current_span(false)
        .with_file(true)
        .with_line_number(true)
        .init();

    set_hook(Box::new(|info| {
        if let Some(location) = info.location() {
            tracing::error!(
                message = %info,
                panic.file = location.file(),
                panic.line = location.line(),
                panic.column = location.column(),
            );
        } else {
            tracing::error!(message = %info);
        }
    }));

    let config = Config::new();

    let app = App::new(config.clone()).await;

    let handlers = Handlers::build(app);

    let addr = format!("{}:{}", LOCAL_HOST, config.app.port)
        .parse::<SocketAddr>()
        .unwrap();

    let listener = TcpListener::bind(addr).await.unwrap();

    if let Ok(res) = axum::serve(listener, handlers).await {
        tracing::info!("Application listening on {}", addr);
    }
}
