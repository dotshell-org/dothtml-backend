use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;
use crate::database::Database;

#[derive(Deserialize)]
pub struct RegisterRequest {
    identifier: String,
    public_key: String,
    device_name: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    identifier: String,
}

#[derive(Deserialize)]
pub struct AuthenticateRequest {
    identifier: String,
    signed_challenge: String,
}

/// # Registration consists of sending an identifier and a new public key.
///
/// * `identifier` (String) - Unique user identifier
/// * `public_key` (String) - New public key to register for WebAuthn authentication
/// * `device_name` (String) - Name of the device being registered
///
/// ---
///
/// If the identifier is not registered, the server will register the new public key
/// and device name in the database.
///
/// Otherwise, the server will send a confirmation message
/// to every device already registered with that identifier. Indeed, the server will send
/// a message via WebSocket to the devices, which will then display a confirmation dialog,
/// and when the user confirms, he proves his identity by JWT and the public key
/// is registered to the identifier.
///
pub async fn register(
    request: web::Json<RegisterRequest>,
    db: web::Data<Database>
) -> impl Responder {
    let identifier = request.identifier.clone();
    let public_key = request.public_key.clone();
    let device_name = request.device_name.clone();

    match db.contains_identifier(&identifier).await {
        Ok(true) => {
            match db.is_identifier_registered(&identifier).await {
                Ok(true) => {
                    // If the identifier is already linked, send a confirmation message
                    HttpResponse::Accepted().body(format!(
                        // TODO: Implement WebSocket message sending
                        "[FAKE] Identifier {} is already registered. A confirmation message has been sent to all devices associated with this identifier.",
                        identifier
                    ))
                },
                Ok(false) => {
                    db.link_public_key(&identifier, &public_key, &device_name).await.unwrap();
                    HttpResponse::Ok().body("New public key registered successfully.")
                },
                Err(_) => {
                    HttpResponse::InternalServerError().body("Database error occurred.")
                }
            }
        },
        Ok(false) => {
            HttpResponse::NotFound().body("Identifier not found. If you detect an issue, please contact your administrator.")
        },
        Err(_) => {
            HttpResponse::InternalServerError().body("Database error occurred")
        }
    }
}


/// # Login consists of sending an identifier to ask for a challenge.
///
/// - `identifier` (String) - Unique user identifier
///
/// ---
///
/// If the identifier is registered, the server will send a challenge.
///
/// Otherwise, the server will return an error message.
///
pub async fn login(request: web::Json<LoginRequest>) -> impl Responder {
    let message = format!("[FAKE] {}, here is your challenge : ...", request.identifier);
    HttpResponse::Ok().body(message)
}

/// # Authentication consists of sending an identifier and a signed challenge.
///
/// * `identifier` (String) - Unique user identifier
/// * `signed_challenge` (String) - Signed challenge from the client
///
/// ---
///
/// First, the server checks if the identifier is registered.
/// If not, it returns an error 404 Not Found.
///
/// If the identifier is registered, the server verifies the signed challenge with the public key
/// stored in the database. If the verification is successful, the server generates a JWT
/// with a secret key and returns it to the client. Otherwise, it returns an error 401 Unauthorized.
///
pub async fn authenticate(request: web::Json<AuthenticateRequest>) -> impl Responder {
    let message = format!("[FAKE] {}, your challenge '{}' is correctly signed", request.identifier, request.signed_challenge);
    HttpResponse::Ok().body(message)
}