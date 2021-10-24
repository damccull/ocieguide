use actix_web::Result;
use actix_web::{web, HttpResponse};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{Request, Response};

use crate::graphql::starwars_schema::StarWarsSchema;

#[tracing::instrument(name = "GraphQL Request", skip(schema, req))]
pub async fn graphql(schema: web::Data<StarWarsSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

#[tracing::instrument(name = "Serve playground to client")]
pub async fn graphql_playground() -> Result<HttpResponse> {
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/graphql").subscription_endpoint("/graphql"),
    );
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[tracing::instrument(name = "Star Wars GraphQL Request", skip(schema, req))]
pub async fn sw_graphql(schema: web::Data<StarWarsSchema>, req: Request) -> Response {
    schema.execute(req.into_inner()).await.into()
}

#[tracing::instrument(name = "Star Wars Serve playground to client")]
pub async fn sw_graphql_playground() -> Result<HttpResponse> {
    let source = playground_source(
        GraphQLPlaygroundConfig::new("/sw_graphql").subscription_endpoint("/sw_graphql"),
    );
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}
