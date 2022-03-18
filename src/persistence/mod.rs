use async_trait::async_trait;

use crate::configuration::DatabaseSettings;

use self::model::OcieItem;

pub mod model;
pub mod repository;
pub mod seed;

#[async_trait]
pub trait OcieItemRepository: Send + Sync + 'static {
    type Error;
    //type Connection;
    type RecordIdType;

    /// Returns a new database for use during a normal run.
    async fn new(database_settings: &DatabaseSettings) -> Self;

    /// Returns a new repository designed for use with integration tests.
    /// This allows implementations to define different settings or create
    /// the database differently than the standard [new] function.
    async fn new_test_repository(database_settings: &DatabaseSettings) -> Self;

    async fn get_all(&self) -> Result<Vec<OcieItem>, Self::Error>;
    async fn get(&self, id: Self::RecordIdType) -> Result<OcieItem, Self::Error>;
    async fn add(&self, item: OcieItem) -> Result<OcieItem, Self::Error>;
    async fn update(&self, id: Self::RecordIdType, item: OcieItem)
        -> Result<OcieItem, Self::Error>;
}
