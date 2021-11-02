use sqlx::PgPool;

use super::model::OcieItem;
use crate::error_handling::error_chain_fmt;
use crate::persistence::model::{LineItemNumber, NationalStockNumber};

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

pub fn get(id: i32, conn: &PgPool) -> Result<OcieItem, sqlx::Error> {
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
