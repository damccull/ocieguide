use std::net::TcpListener;

use actix_cors::Cors;
use actix_web::{dev::Server, web::Data, App, HttpServer};

use tracing_actix_web::TracingLogger;

use crate::{
    api,
    configuration::Settings,
    persistence::repository::{OcieItemRepository, PostgresOcieItemRepository},
};

pub struct Application {
    port: u16,
    server: Server,
}
impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        // Create a PostgresOcieItemRespository
        let repository = PostgresOcieItemRepository::new(&configuration.database);

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

        let server = run(listener, repository, configuration.application.base_url)?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    // Name makes it clear this will run until stopped
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

fn run(
    listener: TcpListener,
    repository: impl OcieItemRepository,
    base_url: String,
) -> Result<Server, std::io::Error> {
    // Wrap shared things in smart pointers
    let repository = Data::new(repository);
    let base_url = Data::new(base_url);

    // Capture the connection from the surrounding environment
    let server = HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_header()
                    .allowed_methods(vec!["POST", "GET"])
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(TracingLogger::default())
            .configure(api::health_check::configure)
            .app_data(repository.clone())
            .app_data(base_url.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
