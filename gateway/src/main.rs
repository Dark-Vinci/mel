use {
    gateway::{
        app::app::App, config::config::Config, handlers::handler::Handlers,
    },
    sdk::{
        constants::constant::{
            LAGOS_TIME, LOCAL_HOST, LOG_DIR, LOG_FILE_NAME,
            LOG_WARNING_FILE_NAME, TIME_ZONE,
        },
        errors::AppError,
        utils::utility::graceful_shutdown,
    },
    std::{env, net::SocketAddr, panic::set_hook},
    tokio::net::TcpListener,
    tracing::level_filters::LevelFilter,
    tracing_appender::rolling,
    tracing_subscriber::{fmt::writer::MakeWriterExt, EnvFilter},
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    env::set_var(TIME_ZONE, LAGOS_TIME);

    let debug_logger = rolling::never(LOG_DIR, LOG_FILE_NAME);
    let warning_error_logger = rolling::never(LOG_DIR, LOG_WARNING_FILE_NAME);

    let file_writer = debug_logger.and(warning_error_logger);

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::TRACE.into())
        .from_env()?
        .add_directive("gateway=trace".parse()?);

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

    let app = App::new(config.clone()).await?;

    let handlers = Handlers::build(app).await?;

    let addr = format!("{}:{}", LOCAL_HOST, config.app.port)
        .parse::<SocketAddr>()
        .unwrap();

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, handlers)
        .with_graceful_shutdown(graceful_shutdown())
        .await
        .unwrap();

    Ok(())
}
