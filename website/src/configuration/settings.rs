use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;
use sqlx::{
    postgres::{PgConnectOptions, PgSslMode},
    ConnectOptions,
};

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
    // pub email_client: EmailClientSettings,
    // pub redis: RedisSettings,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ApplicationSettings {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub base_url: String,
    pub hmac_secret: Secret<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: Secret<String>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub port: u16,
    pub host: String,
    pub database_name: String,
    pub require_ssl: bool,
}

impl DatabaseSettings {
    pub fn with_db(&self) -> PgConnectOptions {
        let mut options = self.without_db().database(&self.database_name);

        // sqlx is noisy by default. Change default level to TRACE for sqlx.
        options = options.log_statements(tracing::log::LevelFilter::Trace);

        options
    }

    pub fn without_db(&self) -> PgConnectOptions {
        let ssl_mode = if self.require_ssl {
            PgSslMode::Require
        } else {
            PgSslMode::Disable
        };

        PgConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(self.password.expose_secret())
            .port(self.port)
            .ssl_mode(ssl_mode)
    }
}

// #[derive(Clone, Debug, Deserialize)]
// pub struct EmailClientSettings {
//     pub authorization_token: Secret<String>,
//     pub base_url: String,
//     pub sender_email: String,
//     pub timeout_milliseconds: u64,
// }
//
// impl EmailClientSettings {
//     pub fn client(self) -> EmailClient {
//         let sender_email = self.sender().expect("Invalid sender email address");
//         let timeout = self.timeout();
//         EmailClient::new(
//             self.base_url,
//             sender_email,
//             self.authorization_token,
//             timeout,
//         )
//     }
//
//     pub fn sender() -> Result<SubscriberEmail, String> {
//         SubscrierEam
//     }
// }
