use actix_web::{get, web::Path, HttpResponse, Responder};

#[get("/item/{id}")]
#[tracing::instrument]
pub async fn item(id: Path<i32>) -> impl Responder {
    let item = serde_json::json!(
          {
            "id": id.as_ref(),
            "lin": "04071N",
            "ets_transferrable": "N",
            "nsn": "7042-01-C10-5056",
            "nomenclature": "DIGITAL MUSIC DISPLAY SYSTEM WITH A",
            "size": "30GB",
            "cic": "",
            "ui": "EACH",
            "unit_price": "$200.00"
          }
    );
    HttpResponse::Ok().json(item)
}
