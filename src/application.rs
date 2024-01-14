use std::net::TcpListener;

use actix_web::{
    dev::Server,
    web::{self, ServiceConfig},
    App, HttpServer,
};

use crate::routes::{app_health, home};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    #[tracing::instrument(level = "debug")]
    pub async fn build(listen_address: &str, listen_port: u16) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", listen_address, listen_port);
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = build_server(listener)?;
        Ok(Self { port, server })
    }

    pub async fn port(&self) -> u16 {
        self.port
    }

    #[tracing::instrument(skip(self), level = "debug")]
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        tracing::info!("👟 Server is running");
        self.server.await
    }
}

#[tracing::instrument(level = "debug")]
pub fn build_server(listener: TcpListener) -> Result<Server, std::io::Error> {
    let config = move |cfg: &mut ServiceConfig| {
        configure_services(cfg);
    };

    let server = HttpServer::new(move || App::new().configure(config))
        .listen(listener)?
        .run();
    Ok(server)
}

pub fn configure_services(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("")
            .service(app_health::health_check)
            .service(home::home),
    );
}
