use actix_web::{get, HttpResponse, Responder};

#[get("/")]
#[tracing::instrument]
pub async fn home() -> impl Responder {
    let home_message = "Welcome to ocieguide.com";
    HttpResponse::Ok().body(home_message)
}
