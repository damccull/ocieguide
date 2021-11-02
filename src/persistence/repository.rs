use sqlx::PgPool;

use crate::persistence::model::{LineItemNumber, NationalStockNumber};

use super::model::OcieItemEntity;

pub async fn get_all(conn: &PgPool) -> Result<Vec<OcieItemEntity>, anyhow::Error> {
    let result = sqlx::query!(
        r#"SELECT id, nsn, lin, nomenclature, size, unit_of_issue, price
        FROM ocieitems"#
    )
    .fetch_all(conn)
    .await?
    .into_iter()
    .map(|row| {
        let nsn = match row.nsn {
            Some(nsn) => NationalStockNumber::parse(nsn),
            None => None,
        };
        Ok(OcieItemEntity {
            id: row.id,
            nsn,
            lin: LineItemNumber::parse(row.lin)?,
            nomenclature: row.nomenclature,
            size: row.size,
            unit_of_issue: row.unit_of_issue,
            price: row.price,
        })
    })
    .collect();
    Ok(result)
}

pub fn get(id: i32, conn: &PgPool) -> Result<OcieItemEntity, sqlx::Error> {
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
