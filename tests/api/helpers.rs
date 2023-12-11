use ocieguide::{
    application::Application,
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;

static TRACING: Lazy<()> = Lazy::new(|| {
    let tracing_level = "ocieguide=debug,info";
    let subscriber_name = "test";
    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(
            subscriber_name.into(),
            tracing_level.into(),
            std::io::stdout,
        );
        init_subscriber(subscriber);
    } else {
        let subscriber =
            get_subscriber(subscriber_name.into(), tracing_level.into(), std::io::sink);
        init_subscriber(subscriber);
    }
});

pub async fn spawn_app() -> TestApp {
    // Setup tracing for tests
    Lazy::force(&TRACING);

    let address = "127.0.0.1";
    let app = Application::build(address, 0)
        .await
        .expect("failed to build the application");

    let port = app.port().await;
    let address = format!("http://{}:{}", address, port);

    tokio::spawn(app.run_until_stopped());

    let test_app = TestApp { address, port };

    test_app
}

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
    pub port: u16,
}
