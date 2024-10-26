use actix_web::HttpResponse;

use crate::dto::{AuthResponse, AuthStatus};

pub async fn health_check() -> HttpResponse {
    let response = AuthResponse::new(AuthStatus::Success, "Service is operational");
    HttpResponse::Ok().json(response)
}
