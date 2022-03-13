use actix_web::{web, HttpResponse};

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api/v1"));
}

fn get_all_items() -> HttpResponse {

}
