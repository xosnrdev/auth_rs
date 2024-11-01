use actix_web::{
    web::{self},
    HttpResponse,
};

use crate::{
    configuration::AppState,
    dto::{AuthResponse, RefreshTokenDto, UpdateEmailDto, UpdatePasswordDto},
    utils::TokenExtract,
};

pub async fn refresh_token(
    app_state: web::Data<AppState>,
    dto: web::Json<RefreshTokenDto>,
) -> HttpResponse {
    let response = app_state
        .get_services()
        .get_auth_service()
        .refresh_token(&dto.refresh_token)
        .await;

    match response {
        Ok(token) => HttpResponse::Ok().json(token),
        Err(e) => HttpResponse::Unauthorized().json(AuthResponse::from(e)),
    }
}

pub async fn logout(app_state: web::Data<AppState>, token: TokenExtract) -> HttpResponse {
    let response = app_state
        .get_services()
        .get_auth_service()
        .logout(&token.get_token())
        .await;

    match response {
        Ok(auth_response) => HttpResponse::Ok().json(auth_response),
        Err(e) => HttpResponse::Unauthorized().json(AuthResponse::from(e)),
    }
}

pub async fn delete_me(token: TokenExtract, app_state: web::Data<AppState>) -> HttpResponse {
    let response = app_state
        .get_services()
        .get_auth_service()
        .delete_me(&token.get_token())
        .await;

    match response {
        Ok(auth_response) => HttpResponse::Ok().json(auth_response),
        Err(e) => HttpResponse::Unauthorized().json(AuthResponse::from(e)),
    }
}

pub async fn get_me(app_state: web::Data<AppState>, token: TokenExtract) -> HttpResponse {
    let response = app_state
        .get_services()
        .get_auth_service()
        .get_me(&token.get_token())
        .await;

    match response {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::Unauthorized().json(AuthResponse::from(e)),
    }
}

pub async fn update_password(
    token: TokenExtract,
    app_state: web::Data<AppState>,
    dto: web::Json<UpdatePasswordDto>,
) -> HttpResponse {
    let response = app_state
        .get_services()
        .get_auth_service()
        .update_password(dto.into_inner(), &token.get_token())
        .await;

    match response {
        Ok(auth_response) => HttpResponse::Ok().json(auth_response),
        Err(e) => HttpResponse::BadRequest().json(AuthResponse::from(e)),
    }
}

pub async fn update_email(
    token: TokenExtract,
    app_state: web::Data<AppState>,
    dto: web::Json<UpdateEmailDto>,
) -> HttpResponse {
    let response = app_state
        .get_services()
        .get_auth_service()
        .update_email(dto.into_inner(), &token.get_token())
        .await;

    match response {
        Ok(auth_response) => HttpResponse::Ok().json(auth_response),
        Err(e) => HttpResponse::BadRequest().json(AuthResponse::from(e)),
    }
}
