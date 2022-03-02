use actix_web::web::Data;
use actix_web::Result;
use actix_web::{guard, web, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use crate::swgraphql::starwars_schema::{create_sw_schema_with_context, StarWarsSchema};

pub fn configure(cfg: &mut web::ServiceConfig) {
    let graphql_sw_schema = Data::new(create_sw_schema_with_context());

    cfg.service(
        web::resource("/sw_graphql")
            .guard(guard::Post())
            .to(sw_graphql),
    )
    .service(
        web::resource("/sw_graphql_playground")
            .guard(guard::Get())
            .to(sw_graphql_playground),
    )
    .app_data(graphql_sw_schema);
}

#[tracing::instrument(name = "Star Wars GraphQL Request", skip(schema, req))]
async fn sw_graphql(schema: web::Data<StarWarsSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[tracing::instrument(name = "Star Wars Serve playground to client")]
async fn sw_graphql_playground() -> Result<HttpResponse> {
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/sw_graphql").subscription_endpoint("/sw_graphql"),
    );
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}
