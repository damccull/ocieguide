use ocieguide::telemetry::{get_subscriber, init_subscriber};

static TRACING: Lazy<()> = Lazy::new(|| {
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            "test".into(),
            "ocieguide=debug,info".into(),
            std::io::stdout,
        );
        init_subscriber(subscriber);
    } else {
        let subscriber =
            get_subscriber("test".into(), "ocieguide=debug,info".into(), std::io::sink);
        init_subscriber(subscriber);
    }
});

pub async fn spawn_app() -> TestApp {
    // Set up subscriber for logging, only the first time per test run.
    // Subsequent tests will use the existing subscriber.
    Lazy::force(&TRACING);

    let configuration = {
        let mut c = get_configuration().expect("failed to read the configuration");
        // Use a different database for each test to ensure no data cross-contamination
        c.database.database_name = Uuid::new_v4().to_string();
        // Use a random OS port
        c.application_port = 0;
        c
    };
}

pub struct TestApp {
    pub address: String,
    pub port: u16,
    pub db_pool: PgPool,
}
