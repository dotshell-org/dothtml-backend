use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ConnectionRequest {
    identifier: String,
    public_key: String,
    signed_challenge: String,
}

/// # Registration consists of sending an identifier and a new public key.
///
/// * `identifier` (String) - Unique user identifier
/// * `public_key` (String) - New public key to register for WebAuthn authentication
///
/// ---
///
/// If the identifier is not registered, the server will register the new public key.
///
/// Otherwise, the server will send a confirmation message
/// to every device already registered with that identifier. Indeed, the server will send
/// a message via WebSocket to the devices, which will then display a confirmation dialog,
/// and when the user confirms, he proves his identity by JWT and the public key
/// is registered to the identifier.
///
pub async fn register(request: web::Json<ConnectionRequest>) -> impl Responder {
    let message = format!("public key '{}' assigned to {}", request.public_key, request.identifier);
    HttpResponse::Ok().body(message)
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
pub async fn login(request: web::Json<ConnectionRequest>) -> impl Responder {
    let message = format!("{}, here is your challenge : ...", request.identifier);
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
pub async fn authenticate(request: web::Json<ConnectionRequest>) -> impl Responder {
    let message = format!("{}, your challenge '{}' is correctly signed", request.identifier, request.signed_challenge);
    HttpResponse::Ok().body(message)
}