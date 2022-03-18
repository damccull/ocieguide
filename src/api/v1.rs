use actix_web::{web, HttpResponse};

use crate::persistence::OcieItemRepository;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/v1"));
}

#[tracing::instrument(name = "Get All Items", skip(repository))]
async fn get_all_items(repository: &impl OcieItemRepository) -> HttpResponse {
    match repository.get_all().await {
        Ok(results) => HttpResponse::Ok().json(results),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
