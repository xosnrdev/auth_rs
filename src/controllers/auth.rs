use actix_web::{web, HttpResponse};

use crate::{
    configuration::AppState,
    dto::{AuthResponse, AuthenticateDto, RegisterDto},
};

pub async fn register(app_state: web::Data<AppState>, dto: web::Json<RegisterDto>) -> HttpResponse {
    let response = app_state
        .get_services()
        .get_auth_service()
        .register(dto.into_inner())
        .await;

    match response {
        Ok(auth_response) => HttpResponse::Ok().json(auth_response),
        Err(e) => {
            let auth_response: AuthResponse = e.into();
            HttpResponse::BadRequest().json(auth_response)
        }
    }
}

pub async fn login(
    app_state: web::Data<AppState>,
    dto: web::Json<AuthenticateDto>,
) -> HttpResponse {
    let response = app_state
        .get_services()
        .get_auth_service()
        .login(dto.into_inner())
        .await;

    match response {
        Ok(auth_response) => HttpResponse::Ok().json(auth_response),
        Err(e) => {
            let auth_response: AuthResponse = e.into();
            HttpResponse::BadRequest().json(auth_response)
        }
    }
}
