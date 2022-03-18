use ocieguide::{
    application::Application,
    configuration::{get_configuration, DatabaseSettings},
    persistence::OcieItemRepository,
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;

use uuid::Uuid;

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

pub struct TestApp<TRepository> {
    pub address: String,
    pub repository: TRepository,
    pub port: u16,
}

impl<TRepository: OcieItemRepository> TestApp<TRepository>

{
    pub async fn spawn() -> Self {
        // Set up logging
        Lazy::force(&TRACING);

        // Set up config with random db name
        let configuration = {
            let mut c = get_configuration().expect("Failed to load configuration.");
            c.database.database_name = Uuid::new_v4().to_string();
            c.application.port = 0;
            c
        };

        let repository = configure_database::<TRepository>(&configuration.database).await;

        // Launch the application as a background task
        let application = Application::build(configuration.clone())
            .await
            .expect("Failed to build the application.");

        let application_port = application.port();

        // Run the server in an executor
        let _ = tokio::spawn(application.run_until_stopped());

        let test_app = TestApp {
            address: format!("http://localhost:{}", application_port),
            repository,
            port: application_port,
        };

        test_app
    }
}

async fn configure_database<TRepository: OcieItemRepository>(
    config: &DatabaseSettings,
) -> TRepository {
    // Create a new database
    TRepository::new(config).await
}
