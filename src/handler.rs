use actix_web::{web, HttpRequest, HttpResponse, Responder};
use argon2::PasswordHash;
use sqlx::PgPool;
use validator::Validate;

use crate::models::{CreateUserDto, LoginDto};
use crate::services::{decode_jwt, encode_jwt, hash_password, verify_password, ServiceError};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/validate-token", web::get().to(validate_token))
            .route("/healthz", web::get().to(health_check)),
    );
}

async fn register(db_pool: web::Data<PgPool>, form: web::Json<CreateUserDto>) -> impl Responder {
    match form.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e.to_string()),
    }

    let hashed_password = match hash_password(&form.password) {
        Ok(p) => p,
        Err(_) => {
            return HttpResponse::InternalServerError()
                .json(ServiceError::InternalServerError.to_string())
        }
    };

    let result = sqlx::query!(
        "INSERT INTO users (id, email, password_hash) VALUES ($1, $2, $3)",
        uuid::Uuid::new_v4(),
        form.email,
        hashed_password
    )
    .execute(db_pool.get_ref())
    .await;

    match result {
        Ok(_) => HttpResponse::Ok().json("User registered"),
        Err(sqlx::Error::Database(_)) => {
            HttpResponse::Conflict().json(ServiceError::UserAlreadyExists.to_string())
        }
        Err(_) => {
            HttpResponse::InternalServerError().json(ServiceError::InternalServerError.to_string())
        }
    }
}

async fn login(db_pool: web::Data<PgPool>, form: web::Json<LoginDto>) -> impl Responder {
    match form.validate() {
        Ok(_) => (),
        Err(e) => return HttpResponse::BadRequest().json(e.to_string()),
    }

    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        form.email
    )
    .fetch_one(db_pool.get_ref())
    .await;

    match user {
        Ok(record) => {
            let parsed_hash = PasswordHash::new(&record.password_hash).unwrap();
            if verify_password(&form.password, &parsed_hash) {
                let token = encode_jwt(record.id).unwrap();
                HttpResponse::Ok().json(token)
            } else {
                HttpResponse::Unauthorized().json(ServiceError::InvalidCredentials.to_string())
            }
        }
        Err(_) => HttpResponse::Unauthorized().json(ServiceError::InvalidCredentials.to_string()),
    }
}

async fn validate_token(req: HttpRequest) -> impl Responder {
    let token = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .map(String::from);

    if let Some(token) = token {
        match decode_jwt(&token) {
            Ok(c) => HttpResponse::Ok().json(c.claims),
            Err(_) => HttpResponse::Unauthorized().json(ServiceError::InvalidToken.to_string()),
        }
    } else {
        HttpResponse::BadRequest().json("Missing token")
    }
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().json("Ok")
}
