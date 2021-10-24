use async_graphql::{
    connection::{query, Connection, Edge, EmptyFields},
    Context, EmptyMutation, EmptySubscription, Object, Result, Schema,
};

use super::ocieguide_schema::{OcieGuide, OcieItem};

pub type OcieGuideSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;

pub struct QueryRoot;
#[Object]
impl QueryRoot {
    async fn ocie_item(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "ID of the OcieItem")] id: String,
    ) -> Option<OcieItem> {
        ctx.data_unchecked::<OcieGuide>().item(&id).map(OcieItem2)
    }

    async fn humans(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> Result<Connection<usize, OcieItem2, EmptyFields, EmptyFields>> {
        let items = ctx
            .data_unchecked::<OcieGuide>()
            .items()
            .iter()
            .copied()
            .collect::<Vec<_>>();

        query_items(after, before, first, last, &items)
            .await
            .map(|conn| conn.map_node(OcieItem2))
    }
}



async fn query_items(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    items: &[usize],
) -> Result<Connection<usize, usize, EmptyFields, EmptyFields>> {
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start = 0usize;
            let mut end = 0usize;

            if let Some(after) = after {
                if after >= items.len() {
                    return Ok(Connection::new(false, false));
                }
                start = after + 1;
            }

            if let Some(before) = before {
                if before == 0 {
                    return Ok(Connection::new(false, false));
                }
                end = before;
            }

            let mut slice = &items[start..end];

            if let Some(first) = first {
                slice = &slice[..first.min(slice.len())];
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                slice = &slice[slice.len() - last.min(slice.len())..];
                start = end - last.min(slice.len());
            }

            let mut connection = Connection::new(start > 0, end < items.len());
            connection.append(
                slice
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| Edge::new(start + idx, *item)),
            );

            Ok(connection)
        },
    )
    .await
}
