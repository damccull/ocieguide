use std::net::TcpListener;

use actix_web::{App, HttpServer, dev::Server, web::{self, Data}};
use sqlx::{PgPool, postgres::PgPoolOptions};
use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}
impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        // Get a connection pool for the database
        let connection_pool = get_connection_pool(&configuration.database);

        // Get and store the application's host and port
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );

        // Create a TCP listener to host the webserver
        let listener = TcpListener::bind(address)?;

        // Store the port the listener actually has for future use
        // This is for compability with the testing framework so it
        // can use random ports
        let port = listener.local_addr().unwrap().port();

        let server = run(
            listener,
            connection_pool,
            configuration.application.base_url,
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // Name makes it clear this will run until stopped
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {}
}

fn run(listener: TcpListener, db_pool: PgPool, base_url: String) -> Result<Server, std::io::Error> {
    // Wrap shared things in smart pointers
    let db_pool = Data::new(db_pool);
    let base_url = Data::new(base_url);

    // Capture the connection from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)
    .run();
    Ok(server)
}

fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}
