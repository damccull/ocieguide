use ocieguide::telemetry;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up tracing
    let subscriber = telemetry::get_subscriber("ocieguide".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    Ok(())
}
