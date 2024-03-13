use ocieguide::{application::Application, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Set up tracing here so we can use tracing::instrument on the startup fn.
    let subscriber =
        telemetry::get_subscriber("ocieguide".to_string(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    startup().await
}

#[tracing::instrument]
async fn startup() -> std::io::Result<()> {
    tracing::info!("🌱Starting application");
    tracing::info!("📄Tracing enabled");

    let app = Application::build("127.0.0.1", 8000).await?;

    tracing::info!("🕸️Starting web server");
    app.run_until_stopped().await?;
    Ok(())
}
