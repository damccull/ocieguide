use async_graphql::{EmptyMutation, EmptySubscription, MergedObject, Schema};

// pub mod ocieguide_model;
// pub mod ocieguide_schema;
pub mod health;
//pub mod ocieguide;

pub type OcieGuideSchema = Schema<Query, EmptyMutation, EmptySubscription>;

#[derive(Default, MergedObject)]
pub struct Query(health::HealthQuery);

pub fn build_schema() -> OcieGuideSchema {
    Schema::build(Query::default(), EmptyMutation, EmptySubscription).finish()
}
