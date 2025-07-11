//! # Route Configuration
//! 
//! This module defines the HTTP routes for the application, organizing
//! them into logical groups for the website API and backoffice API.
//! 
//! ## Route Groups
//! 
//! ### Website API
//! - `POST /contact` - Handle contact form submissions
//! 
//! ### Backoffice API
//! - `GET /inbox` - Retrieve all messages
//! - `POST /inbox/{id}/assign` - Assign a message to a user
//! - `POST /inbox/{id}/release` - Release a message from assignment
//! - `POST /inbox/{id}/reply` - Reply to a message
//! - `DELETE /inbox/{id}` - Delete a message
//! 
//! ## Usage
//! 
//! This module is used in `main.rs` to configure the application routes:
//! 
//! ```rust
//! use actix_web::{App, web};
//! use dothtml_backend::routes;
//! 
//! let app = App::new()
//!     .configure(routes::config);
//! ```

use actix_web::web;

pub use crate::handlers::*;

/// Configures all HTTP routes for the application.
/// 
/// This function sets up the routing table by mapping HTTP methods
/// and paths to their corresponding handler functions. It organizes
/// routes into logical groups for better maintainability.
/// 
/// # Arguments
/// 
/// * `cfg` - Mutable reference to the service configuration
/// 
/// # Route Structure
/// 
/// ## Website API Routes
/// These routes are designed for public-facing website functionality:
/// - Contact form submissions
/// - Public inquiries
/// 
/// ## Backoffice API Routes
/// These routes are designed for internal management and administration:
/// - Message management (CRUD operations)
/// - Assignment and workflow management
/// - Administrative actions
/// 
/// # Examples
/// 
/// Using this configuration in an Actix Web application:
/// 
/// ```rust
/// use actix_web::{App, HttpServer, web};
/// use dothtml_backend::routes;
/// 
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     HttpServer::new(|| {
///         App::new()
///             .configure(routes::config)
///     })
///     .bind("127.0.0.1:8080")?
///     .run()
///     .await
/// }
/// ```
pub fn config(cfg: &mut web::ServiceConfig) {
    cfg
        // ========================= Website API ========================= //
        .route("/contact", web::post().to(contact))
        
        // ======================== Backoffice API ======================= //
        .route("/inbox/pending", web::get().to(pending))
        .route("/inbox/{id}", web::get().to(get_message_by_id));
}
