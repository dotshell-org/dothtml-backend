use actix_web::web;

pub use crate::handlers::*;

/// Function to configure routes, to be used in main.rs
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(hello));
    cfg.route("/ping", web::get().to(ping));
}
