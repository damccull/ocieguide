use std::sync::Arc;

use async_graphql::{Context, EmptyMutation, EmptySubscription, Schema};
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::persistence::{model::{LineItemNumber, NationalStockNumber}, repository};

pub type OcieGuideSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    async fn get_items(&self, ctx: &Context<'_>) -> Vec<OcieItem> {
        repository::get_all(&get_conn_from_ctx(ctx))
            .expect("Can't get planets")
            .iter()
            .map(OcieItem::from)
            .collect()
    }

    async fn get_item(&self, ctx: &Context<'_>, id: ID) -> Option<OcieItem> {
        find_ocieitem_by_id_internal(ctx, id)
    }

    async fn find_item_by_id(&self, ctx: &Context<'_>, id: ID) -> Option<OcieItem> {
        find_ocieitem_by_id_internal(ctx, id)
    }
}

fn find_ocieitem_by_id_internal(ctx: &Context<'_>, id: ID) -> Option<OcieItem> {
    let id = id
        .to_string()
        .parse::<i32>()
        .expect("Can't get id from String");

    repository::get(id, &get_conn_from_ctx(ctx))
        .ok()
        .map(|i| OcieItem::from(&i))
}

type Conn = PgConnection;

pub fn get_conn_from_ctx(ctx: &Context<'_>) -> Conn {
    ctx.data::<Arc<PgPool>>()
        .expect("Can't get pool")
        .get()
        .expect("Can't get DB connection")
}

pub fn create_schema_with_context(pool: PgPool) -> OcieGuideSchema {
    let arc_pool = Arc::new(pool);
    // let cloned_pool = Arc::clone(&arc_pool);
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
pub struct OcieItem {
    id: Uuid,
    nsn: NationalStockNumber,
    lin: LineItemNumber,
    nomenclature: String,
    size: Option<String>,
    unit_of_issue: Option<String>,
    price: Option<f32>,
}
impl OcieItem {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn nsn(&self) -> &NationalStockNumber {
        &self.nsn
    }

    async fn lin(&self) -> &LineItemNumber {
        &self.lin
    }

    async fn nomenclature(&self) -> &String {
        &self.nomenclature
    }

    async fn size(&self) -> &Option<String> {
        &self.size
    }

    async fn unit_of_issue(&self) -> &Option<String> {
        &self.unit_of_issue
    }

    async fn price(&self) -> &Option<f32> {
        &self.price
    }
}
impl Default for OcieItem {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            nsn: None,
            lin: None,
            nomenclature: "".to_owned(),
            size: None,
            unit_of_issue: None,
            price: None,
        }
    }
}

