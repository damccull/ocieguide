use ocieguide::{application::Application, configuration::get_configuration, telemetry};
use std::fmt::{Debug, Display};
use tokio::task::JoinError;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set up tracing
    let subscriber = telemetry::get_subscriber("ocieguide".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);

    // Set up configuration
    let configuration = get_configuration().expect("failed to read configuration");

    let app = Application::build(configuration.clone()).await?;
    let app_task = tokio::spawn(app.run_until_stopped());

    tokio::select! {
        o = app_task => report_exit("Website", o),
    }

    Ok(())
}

fn report_exit(task_name: &str, outcome: Result<Result<(), impl Debug + Display>, JoinError>) {
    match outcome {
        Ok(Ok(())) => {
            tracing::info!("{} has exited", task_name)
        }
        Ok(Err(e)) => {
            tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "{} failed",
                    task_name
            )
        }
        Err(e) => {
            tracing::error!(
                    error.cause_chain = ?e,
                    error.message = %e,
                    "{} task failed to complete",
                    task_name
            )
        }
    }
}
