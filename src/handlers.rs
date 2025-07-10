use actix_web::{web, HttpResponse, Responder};
use crate::database::Database;

// ========================= Website API ========================= //

use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct ContactForm {
    #[validate(length(min = 1, max = 100, message = "Name must be between 1 and 100 characters"))]
    pub name: String,

    #[validate(email(message = "Invalid email address"))]
    pub email: String,

    #[validate(length(max = 50))]
    #[serde(default)]
    pub country_region: String,

    #[validate(length(max = 20))]
    #[serde(default)]
    pub phone_number: String,

    #[validate(length(max = 100))]
    #[serde(default)]
    pub company: String,

    #[validate(length(min = 1, max = 2000, message = "Message must be between one and 2000 characters"))]
    pub message: String,
}

/// Handles contact form submissions from the website.
/// 
/// This endpoint processes and validates contact form data submitted by users,
/// storing the message in the database for later processing.
/// 
/// # Arguments
/// 
/// * `form` - JSON payload containing the contact form data
/// * `db` - Shared database connection instance
/// 
/// # Returns
/// 
/// Returns an HTTP response with:
/// - 201 Created when the message is successfully stored
/// - 400 Bad Request if the input data is invalid
/// - 500 Internal Server Error if database operation fails
/// 
/// # Examples
/// 
/// ```
/// POST /contact
/// Content-Type: application/json
/// 
/// {
///   "name": "John Doe",
///   "email": "john@example.com",
///   "country_region": "France",
///   "phone_number": "+33612345678",
///   "company": "ACME Corp",
///   "message": "Hello, I have a question..."
/// }
/// ```
/// 
/// Success Response:
/// ```
/// 201 Created
/// {
///   "status": "success",
///   "message": "Contact request received"
/// }
/// ```
pub async fn contact(
    form: web::Json<ContactForm>,
    db: web::Data<Database>
) -> impl Responder {
    // Validate form data
    if let Err(errors) = form.validate() {
        return HttpResponse::BadRequest().json(errors);
    }

    // Insert a message into the database
    match db.insert_message(
        &form.name,
        &form.email,
        &form.country_region,
        &form.phone_number,
        &form.company,
        &form.message
    ).await {
        Ok(_) => HttpResponse::Created().json(serde_json::json!({
            "status": "success",
            "message": "Contact request received"
        })),
        Err(_) => HttpResponse::InternalServerError().json(serde_json::json!({
            "status": "error",
            "message": "Failed to process the contact request"
        }))
    }
}

// ======================== Backoffice API ======================= //

/// Retrieves pending messages from the inbox.
/// 
/// This endpoint fetches up to 20 pending messages from the database, randomly shuffled,
/// and returns them to the backoffice interface. Each message includes basic information
/// like ID, name, email, and message content.
/// 
/// # Arguments
/// 
/// * `db` - Shared database connection instance
/// 
/// # Returns
/// 
/// Returns an HTTP response with either:
/// - 200 OK with JSON array of pending messages
/// - 500 Internal Server Error if database operation fails
/// 
/// # Examples
/// 
/// ```
/// GET /pending
/// ```
/// 
/// Response:
/// ```json
/// [
///   {
///     "id": "123e4567-e89b-12d3-a456-426614174000",
///     "name": "John Doe",
///     "email": "john@example.com",
///     "message": "Hello, I have a question..."
///   },
///   ...
/// ]
/// ```
pub async fn pending(db: web::Data<Database>) -> impl Responder {
    match db.list_pending_messages().await {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch pending messages")
    }
}

pub async fn assign(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("assign message {}", id))
}
pub async fn release(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("release message {}", id))
}
pub async fn reply(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("reply to message {}", id))
}
pub async fn delete(path: web::Path<String>) -> impl Responder {
    let id = path.into_inner();
    HttpResponse::Ok().body(format!("delete the message {}", id))
}
