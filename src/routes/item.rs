use actix_web::{get, web::Path, HttpResponse, Responder};

#[get("/item/{id}")]
#[tracing::instrument]
pub async fn item(id: Path<i32>) -> impl Responder {
    let message = format!("Item list for {}", id.as_ref());
    HttpResponse::Ok().body(message)
}
