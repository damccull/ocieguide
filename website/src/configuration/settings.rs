use secrecy::Secret;
use serde::Deserialize;
use serde_aux::field_attributes::deserialize_number_from_string;

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
