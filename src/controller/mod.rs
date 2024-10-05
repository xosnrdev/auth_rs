mod health;
mod user;

use health::*;

pub struct Controllers;

impl Controllers {
    pub fn configure_routes(cfg: &mut actix_web::web::ServiceConfig) {
        cfg.service(health);
    }
}
