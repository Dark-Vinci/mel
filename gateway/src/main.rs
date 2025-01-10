use {
    gateway::{app::App, handlers::handler::Handlers},
    panic::set_hook,
    std::{net::SocketAddr, panic},
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

    let app = App::new().await;

    let handlers = Handlers::build(app);

    let addr = format!("0.0.0.0:{}", 3000).parse::<SocketAddr>().unwrap();

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    if let Ok(res) = axum::serve(listener, handlers).await {
        tracing::info!("Application listening on {}", addr);
    }
}
