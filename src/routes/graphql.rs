use actix_web::web::Data;
use actix_web::{guard, Result};
use actix_web::{web, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::graphql::{build_schema, OcieGuideSchema};
//use crate::graphql::ocieguide::OcieGuideSchema;

pub fn configure(cfg: &mut web::ServiceConfig) {
    let graphql_schema = Data::new(build_schema());
    cfg.service(web::resource("/graphql").guard(guard::Post()).to(graphql))
        .service(
            web::resource("/graphql_playground")
                .guard(guard::Get())
                .to(graphql_playground),
        )
        .app_data(graphql_schema);
}

#[tracing::instrument(name = "GraphQL Request", skip(schema, req))]
async fn graphql(schema: web::Data<OcieGuideSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tracing::instrument(name = "Serve playground to client")]
async fn graphql_playground() -> Result<HttpResponse> {
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    );
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}
