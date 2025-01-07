use {
    account::{config::config::Config, app::app::App},
    sdk::{
        constants::constant::{
            LAGOS_TIME, LOG_DIR, LOG_FILE_NAME, LOG_WARNING_FILE_NAME,
            TIME_ZONE,
        },
        errors::AppError,
        utils::utility::graceful_shutdown,
        generated_proto_rs::mel_account::account_service_server::AccountServiceServer,
    },
    std::panic,
    tonic::transport::Server,
    tracing_appender::rolling,
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
    let config = Config::new();

    let addr: SocketAddr =
        format!("{0}:{1}", LOCAL_HOST, &config.app_port).parse()?;

    let app_name: &str = &config.app.app_name.clone();
    let service_name: &str = &config.app.service_name.clone();

    // bootstrap application
    let app = App::new(&config).await?;

    // bootstrap service controller
    let account_server = Account::new(app);

    debug!(
        "🚀{0} for {1} is listening on address {2} 🚀",
        app_name, service_name, addr
    );
    
    panic::set_hook(Box::new(|info| {
        // send notification
        println!("A panic has occured with info {}", info.to_string());
    }));

    // start service and listen to shut down hooks;
    Server::builder()
        .add_service(AccountServiceServer::new(account_server))
        .serve_with_shutdown(addr, graceful_shutdown())
        .await?;

    Ok(())
}
