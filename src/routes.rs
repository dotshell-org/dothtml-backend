use actix_web::web;

pub use crate::handlers::*;

/// Function to configure routes, to be used in main.rs
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        // Website API
        .route("/contact", web::post().to(contact))
        // Backoffice API
        .route("/inbox", web::get().to(get))
        .route("/inbox/{id}/assign", web::post().to(assign))
        .route("/inbox/{id}/release", web::post().to(release))
        .route("/inbox/{id}/reply", web::post().to(reply))
        .route("/inbox/{id}", web::delete().to(delete));
}
