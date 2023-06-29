use std::net::TcpListener;

use axum::{
    extract::FromRef,
    routing::{get, IntoMakeService},
    Router, Server,
};
use hyper::server::conn::AddrIncoming;
use secrecy::Secret;
use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::{
    configuration::{DatabaseSettings, Settings},
    routes::health_check,
    telemetry::RouterExt,
};

pub type AppServer = Server<AddrIncoming, IntoMakeService<Router>>;
pub struct Application {
    port: u16,
    server: AppServer,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        // Get database pool
        let db_pool = get_db_pool(&configuration.database);

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address.to_string()).map_err(|e| {
            tracing::error!("failed to bind port {}", address);
            e
        })?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, db_pool, configuration.application.base_url);
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> hyper::Result<()> {
        self.server.await
    }
}

/// Get a database connection pool.
pub fn get_db_pool(configuration: &DatabaseSettings) -> PgPool {
    PgPoolOptions::new()
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_lazy_with(configuration.with_db())
}

pub fn run(listener: TcpListener, db_pool: PgPool, base_url: String) -> AppServer {
    // Build app state
    let app_state = AppState {
        db_pool,
        base_url: ApplicationBaseUrl(base_url),
    };

    // Create a router that will contain and match all routes for the application
    let app = Router::new()
        .route("/health_check", get(health_check))
        .add_axum_tracing_layer()
        .with_state(app_state);

    // Start the axum server and set up to use supplied listener
    axum::Server::from_tcp(listener)
        .expect("failed to create server from listener")
        .serve(app.into_make_service())
}

#[derive(Clone)]
pub struct AppState {
    db_pool: PgPool,
    base_url: ApplicationBaseUrl,
}

impl FromRef<AppState> for PgPool {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.db_pool.clone()
    }
}

impl FromRef<AppState> for ApplicationBaseUrl {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.base_url.clone()
    }
}

#[derive(Clone)]
pub struct ApplicationBaseUrl(pub String);

#[derive(Clone)]
pub struct HmacSecret(pub Secret<String>);
