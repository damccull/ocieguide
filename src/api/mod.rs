use actix_web::web;

pub mod health_check;
pub mod v1;

pub fn configure(cfg: &mut web::ServiceConfig) {
    v1::configure(cfg);
    health_check::configure(cfg);
}