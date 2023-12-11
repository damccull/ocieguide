use ocieguide::{application::Application, telemetry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = telemetry::get_subscriber("ocieguide", "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    tracing::info!("🪵Tracing enabled");

    let app = Application::build("127.0.0.1", 8000).await?;

    tracing::info!("🕸️Starting web server");
    app.run_until_stopped().await?;
    Ok(())
}
