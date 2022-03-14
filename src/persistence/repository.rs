use async_trait::async_trait;
use bigdecimal::BigDecimal;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use uuid::Uuid;

use super::model::OcieItem;
use crate::configuration::DatabaseSettings;
use crate::error_handling::error_chain_fmt;
use crate::persistence::model::{LineItemNumber, NationalStockNumber};

#[async_trait]
pub trait OcieItemRepository: Send + Sync + 'static {
    type Error;
    //type Connection;
    type RecordIdType;
    async fn get_all(&self) -> Result<Vec<OcieItem>, Self::Error>;
    async fn get(&self, id: Self::RecordIdType) -> Result<OcieItem, Self::Error>;
    async fn add(&self, item: OcieItem) -> Result<OcieItem, Self::Error>;
    async fn update(&self, id: Self::RecordIdType, item: OcieItem)
        -> Result<OcieItem, Self::Error>;
}

pub struct PostgresOcieItemRepository {
    pool: PgPool,
}
impl PostgresOcieItemRepository {
    pub fn new(database_configuration: &DatabaseSettings) -> Self {
        Self {
            // Get a connection pool for the database
            pool: Self::get_connection_pool(database_configuration),
        }
    }

    /// Returns a `PgPool`
    ///
    /// Public so that the integration tests can use this too.
    fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
        PgPoolOptions::new()
            .connect_timeout(std::time::Duration::from_secs(2))
            .connect_lazy_with(configuration.with_db())
    }
}
#[async_trait]
impl OcieItemRepository for PostgresOcieItemRepository {
    type Error = RepositoryError;

    //type Connection = PgPool;

    type RecordIdType = Uuid;

    #[tracing::instrument(name = "API V1 - get_all", skip(self))]
    async fn get_all(&self) -> Result<Vec<OcieItem>, Self::Error> {
        let result = sqlx::query!(
            r#"SELECT id, nsn, lin, nomenclature, size, unit_of_issue, price
            FROM ocieitems"#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.into()))
        .into_iter()
        .flatten()
        .flat_map(|row| {
            let nsn = match NationalStockNumber::parse(row.nsn) {
                Ok(nsn) => nsn,
                Err(e) => {
                    tracing::error!("Error parsing NSN: {:?}", e);
                    return Err(e);
                }
            };
            let lin = match LineItemNumber::parse(row.lin) {
                Ok(lin) => lin,
                Err(e) => {
                    tracing::error!("Error parsing LIN: {:?}", e);
                    return Err(e);
                }
            };
            Ok(OcieItem {
                id: row.id,
                nsn,
                lin,
                nomenclature: row.nomenclature,
                size: row.size,
                unit_of_issue: row.unit_of_issue,
                price: row.price,
            })
        })
        .collect::<Vec<OcieItem>>();
        Ok(result)
    }

    #[tracing::instrument(name = "API V1 - get", skip(self))]
    async fn get(&self, id: Self::RecordIdType) -> Result<OcieItem, Self::Error> {
        struct Row {
            id: Uuid,
            nsn: String,
            lin: String,
            nomenclature: String,
            size: Option<String>,
            unit_of_issue: Option<String>,
            price: Option<BigDecimal>,
        }

        let result = sqlx::query_as!(
            Row,
            r#"SELECT id, nsn, lin, nomenclature, size, unit_of_issue, price
            FROM ocieitems
            WHERE id = $1"#,
            id
        )
        .map(|row| {
            let nsn = match NationalStockNumber::parse(row.nsn) {
                Ok(nsn) => nsn,
                Err(e) => {
                    tracing::error!("Error parsing NSN: {:?}", e);
                    return Err(e);
                }
            };
            let lin = match LineItemNumber::parse(row.lin) {
                Ok(lin) => lin,
                Err(e) => {
                    tracing::error!("Error parsing LIN: {:?}", e);
                    return Err(e);
                }
            };
            Ok(OcieItem {
                id: row.id,
                nsn,
                lin,
                nomenclature: row.nomenclature,
                size: row.size,
                unit_of_issue: row.unit_of_issue,
                price: row.price,
            })
        })
        .fetch_one(&self.pool)
        .await;

        match result {
            Ok(r) => match r {
                Ok(r) => Ok(r),
                Err(e) => return Err(RepositoryError::UnexpectedError(e)),
            },
            Err(e) => Err(RepositoryError::DatabaseError(e.into())),
        }
    }

    #[tracing::instrument(name = "API V1 - add", skip(self))]
    async fn add(&self, item: OcieItem) -> Result<OcieItem, Self::Error> {
        todo!()
    }

    #[tracing::instrument(name = "API V1 - update", skip(self))]
    async fn update(
        &self,
        id: Self::RecordIdType,
        item: OcieItem,
    ) -> Result<OcieItem, Self::Error> {
        todo!()
    }
}

#[tracing::instrument(name = "Get all OcieItems", skip(conn))]
pub async fn get_all(conn: &PgPool) -> Result<Vec<OcieItem>, anyhow::Error> {
    let result = sqlx::query!(
        r#"SELECT id, nsn, lin, nomenclature, size, unit_of_issue, price
        FROM ocieitems"#
    )
    .fetch_all(conn)
    .await?
    .into_iter()
    .flat_map(|row| {
        let nsn = match NationalStockNumber::parse(row.nsn) {
            Ok(nsn) => nsn,
            Err(e) => {
                tracing::error!("Error parsing NSN: {:?}", e);
                return None;
            }
        };
        let lin = match LineItemNumber::parse(row.lin) {
            Ok(lin) => lin,
            Err(e) => {
                tracing::error!("Error parsing LIN: {:?}", e);
                return None;
            }
        };
        Some(OcieItem {
            id: row.id,
            nsn,
            lin,
            nomenclature: row.nomenclature,
            size: row.size,
            unit_of_issue: row.unit_of_issue,
            price: row.price,
        })
    })
    .collect::<Vec<OcieItem>>();
    Ok(result)
}

pub fn get(_id: i32, _conn: &PgPool) -> Result<OcieItem, sqlx::Error> {
    //TODO: Run SQLX query to pull OcieItem
    todo!()
}

#[derive(thiserror::Error)]
pub enum RepositoryError {
    #[error("Database error.")]
    DatabaseError(#[source] anyhow::Error),
    #[error(transparent)]
    UnexpectedError(#[from] anyhow::Error),
}

impl std::fmt::Debug for RepositoryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        error_chain_fmt(self, f)
    }
}
