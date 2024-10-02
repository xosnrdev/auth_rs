use actix_web::{get, HttpResponse};

#[get("/health")]
pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json("I'm alive!")
}
