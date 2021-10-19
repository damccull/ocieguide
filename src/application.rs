use std::net::TcpListener;

use actix_cors::Cors;
use actix_web::{
    dev::Server,
    guard,
    web::{self, Data},
    App, HttpServer,
};
use async_graphql::{EmptyMutation, EmptySubscription};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing_actix_web::TracingLogger;

use crate::{
    configuration::{DatabaseSettings, Settings},
    graphql::{
        starwars_schema::{StarWars, StarWarsSchema},
        QueryRoot,
    },
    routes::{graphql, graphql_playground, health_check},
};

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

        let graphql_schema = StarWarsSchema::build(QueryRoot, EmptyMutation, EmptySubscription)
            .data(StarWars::new())
            .finish();

        let server = run(
            listener,
            connection_pool,
            graphql_schema,
            configuration.application.base_url,
        )?;

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
    db_pool: PgPool,
    graphql_schema: StarWarsSchema,
    base_url: String,
) -> Result<Server, std::io::Error> {
    // Wrap shared things in smart pointers
    let db_pool = Data::new(db_pool);
    let base_url = Data::new(base_url);
    let graphql_schema = Data::new(graphql_schema);

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
            .route("/health_check", web::get().to(health_check))
            .service(web::resource("/graphql").guard(guard::Post()).to(graphql))
            .service(
                web::resource("/graphql_playground")
                    .guard(guard::Get())
                    .to(graphql_playground),
            )
            .app_data(db_pool.clone())
            .app_data(base_url.clone())
            .app_data(graphql_schema.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

/// Returns a `PgPool`
///
/// Public so that the integration tests can use this too.
pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}
