use async_graphql::{Context, Object};

use super::ocieguide_schema::OcieItem;

pub struct QueryRoot;
#[Object]
impl QueryRoot {
    async fn ocie_item(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "ID of the OcieItem")] id: String,
    ) -> OcieItem {
        todo!()
    }
}
