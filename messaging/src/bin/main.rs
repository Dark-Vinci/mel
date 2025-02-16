use {
    messaging::{
        app::app::App, config::config::Config, server::server::Messaging,
    },
    sdk::{
        constants::constant::{
            LAGOS_TIME, LOCAL_HOST, LOG_DIR, LOG_FILE_NAME,
            LOG_WARNING_FILE_NAME, TIME_ZONE,
        },
        errors::AppError,
        generated_proto_rs::mel_account::account_service_server::AccountServiceServer,
        utils::utility::graceful_shutdown,
    },
    std::{env, net::SocketAddr, panic},
    tonic::transport::Server,
    tracing::{error, info},
    tracing_appender::rolling,
    tracing_core::LevelFilter,
    tracing_subscriber::{fmt::writer::MakeWriterExt, EnvFilter},
};

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // set time zone
    env::set_var(TIME_ZONE, LAGOS_TIME);

    let debug_logger = rolling::never(LOG_DIR, LOG_FILE_NAME);
    let warning_error_logger = rolling::never(LOG_DIR, LOG_WARNING_FILE_NAME);

    let file_writer = debug_logger.and(warning_error_logger);

    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()?
        .add_directive("account=debug".parse()?);

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

    panic::set_hook(Box::new(|info| {
        if let Some(location) = info.location() {
            error!(
                message = %info,
                panic.file = location.file(),
                panic.line = location.line(),
                panic.column = location.column(),
            );
        } else {
            error!(message = %info);
        }
    }));

    let config = Config::new();

    let addr: SocketAddr =
        format!("{0}:{1}", LOCAL_HOST, &config.app.port).parse()?;

    let app_name: &str = &config.app.app_name.clone();
    let service_name: &str = &config.app.service_name.clone();

    // bootstrap application
    let app = App::new(&config).await;

    // bootstrap service controller
    let account_server = Messaging::new(app);

    info!(
        "🚀{0} for {1} is listening on address {2} 🚀",
        app_name, service_name, addr
    );

    // start service and listen to shut down hooks;
    if let Err(err) = Server::builder()
        .add_service(AccountServiceServer::new(account_server))
        .serve_with_shutdown(addr, graceful_shutdown())
        .await
    {
        error!("error:{}", err);
    }

    Ok(())
}
