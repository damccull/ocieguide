use std::{str::FromStr, sync::Arc};

use async_graphql::{
    Context, EmptyMutation, EmptySubscription, InputValueError, InputValueResult, Object, Scalar,
    ScalarType, Schema, Value, ID,
};
use serde::{Deserialize, Serialize};
use sqlx::types::BigDecimal;
use sqlx::PgPool;
use uuid::Uuid;

use crate::persistence::{
    model::{LineItemNumber, NationalStockNumber, OcieItem},
    repository,
};

use super::starwars_schema::StarWarsSchema;

pub type OcieGuideSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn get_items(&self, ctx: &Context<'_>) -> Vec<OcieItemApi> {
        repository::get_all(&get_conn_from_ctx(ctx))
            .await
            .expect("Can't get OCIE items")
            .iter()
            .map(|i| OcieItemApi::from(i))
            .collect()
    }

    // async fn get_item(&self, ctx: &Context<'_>, id: ID) -> Option<OcieItemApi> {
    //     find_ocieitem_by_id_internal(ctx, id)
    // }

    // async fn find_item_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<OcieItemApi> {
    //     find_ocieitem_by_id_internal(ctx, id)
    // }
}

fn find_ocieitem_by_id_internal(ctx: &Context<'_>, id: ID) -> Option<OcieItemApi> {
    let id = id
        .to_string()
        .parse::<i32>()
        .expect("Can't get id from String");

    repository::get(id, &get_conn_from_ctx(ctx))
        .ok()
        .map(|i| OcieItemApi::from(&i))
}

pub fn get_conn_from_ctx<'a>(ctx: &Context<'a>) -> &'a PgPool {
    let x = ctx.data::<PgPool>().expect("Can't get pool");
    x
}

pub fn create_schema_with_context(pool: PgPool) -> OcieGuideSchema {
    let arc_pool = Arc::new(pool);
    let cloned_pool = Arc::clone(&arc_pool);
    // let details_data_loader =
    //     DataLoader::new(DetailsLoader { pool: cloned_pool }).max_batch_size(10);

    Schema::build(Query, EmptyMutation, EmptySubscription)
        // limits are commented out, because otherwise introspection query won't work
        // .limit_depth(3)
        // .limit_complexity(15)
        .data(arc_pool)
        // .data(details_data_loader)
        .finish()
}

pub struct Mutation;

pub struct Subscription;

#[derive(Debug, Serialize, Deserialize)]
pub struct OcieItemApi {
    id: Uuid,
    nsn: NationalStockNumber,
    lin: LineItemNumber,
    nomenclature: String,
    size: Option<String>,
    unit_of_issue: Option<String>,
    price: Option<CustomBigDecimal>,
}

#[Object]
impl OcieItemApi {
    async fn id(&self) -> ID {
        self.id.into()
    }

    async fn nsn(&self) -> String {
        self.nsn.as_ref().to_string()
    }

    async fn lin(&self) -> String {
        self.lin.as_ref().to_string()
    }

    async fn nomenclature(&self) -> String {
        self.nomenclature.clone()
    }

    async fn size(&self) -> Option<String> {
        self.size.clone()
    }

    async fn unit_of_issue(&self) -> Option<String> {
        self.unit_of_issue.clone()
    }

    async fn price(&self) -> Option<CustomBigDecimal> {
        self.price.clone()
    }
}
impl From<&OcieItem> for OcieItemApi {
    fn from(i: &OcieItem) -> Self {
        Self {
            id: i.id,
            nsn: i.nsn.clone(),
            lin: i.lin.clone(),
            nomenclature: i.nomenclature.clone(),
            size: i.size.clone(),
            unit_of_issue: i.unit_of_issue.clone(),
            price: CustomBigDecimal::parse(async_graphql::Value::String(
                i.price.clone().unwrap().to_string(),
            ))
            .ok(),
        }
    }
}
// impl Default for OcieItem {
//     fn default() -> Self {
//         Self {
//             id: Uuid::new_v4(),
//             nsn: None,
//             lin: None,
//             nomenclature: "".to_owned(),
//             size: None,
//             unit_of_issue: None,
//             price: None,
//         }
//     }
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomBigDecimal(BigDecimal);

#[Scalar(name = "BigDecimal")]
impl ScalarType for CustomBigDecimal {
    fn parse(value: Value) -> InputValueResult<Self> {
        match value {
            Value::String(s) => {
                let parsed_value = BigDecimal::from_str(&s)?;
                Ok(CustomBigDecimal(parsed_value))
            }
            _ => Err(InputValueError::expected_type(value)),
        }
    }

    fn to_value(&self) -> Value {
        Value::String(self.0.to_string())
    }
}
