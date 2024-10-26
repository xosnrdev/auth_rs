mod auth;
mod health_check;
mod token;

use auth::*;
use health_check::*;
use token::*;

use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(
                web::scope("/auth")
                    .route("/register", web::post().to(register))
                    .route("/login", web::post().to(login))
                    .route("/password", web::put().to(update_password))
                    .route("/email", web::put().to(update_email))
                    .route("/token/refresh", web::post().to(refresh_token))
                    .route("/logout", web::post().to(logout))
                    .route("/healthz", web::get().to(health_check)),
            )
            .service(
                web::scope("/users")
                    .route("/me", web::get().to(get_me))
                    .route("/me", web::delete().to(delete_me)),
            ),
    );
}
