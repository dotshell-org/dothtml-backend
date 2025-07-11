use actix_web::{web, HttpResponse, Responder};
use base64::{engine::general_purpose, Engine as _};
use rand::Rng;
use serde::Deserialize;
use serde_json::json;
use sqlx::Error;
use crate::database::Database;

#[derive(Deserialize)]
pub struct RegisterRequest {
    identifier: String,
    public_key: String,
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

    match db.contains_identifier(&identifier).await {
        Ok(true) => {
            match db.is_identifier_registered(&identifier).await {
                Ok(true) => {
                    // If the identifier is already linked, send an error response
                    HttpResponse::Conflict().body("Identifier is already registered with a public key.")
                },
                Ok(false) => {
                    db.link_public_key(&identifier, &public_key).await.unwrap();
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
pub async fn login(
    request: web::Json<LoginRequest>,
    db: web::Data<Database>
) -> impl Responder {
    let identifier = request.identifier.clone();

    match db.get_public_key(&identifier).await {
        Ok(_public_key) => {
            // Generate a random challenge
            let mut rng = rand::rng();
            let challenge_bytes: [u8; 32] = rng.random();
            let challenge = general_purpose::STANDARD.encode(challenge_bytes);

            db.store_challenge_for_user(&identifier, &challenge).await.unwrap();

            HttpResponse::Ok().json(json!({ "challenge": challenge }))
        }
        Err(Error::RowNotFound) => {
            HttpResponse::NotFound().body("Identifier not found.")
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Database error occurred")
        }
    }
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
pub async fn authenticate(
    request: web::Json<AuthenticateRequest>,
    db: web::Data<Database>
) -> impl Responder {
    let identifier = request.identifier.clone();
    let signed_challenge = request.signed_challenge.clone();

    // Retrieve the public key from the database
    match db.get_public_key(&identifier).await {
        Ok(_public_key) => {
            // Verify the signed challenge with the public key
            HttpResponse::Ok().body("[FAKE] Authentication successful.")
        }
        Err(Error::RowNotFound) => {
            HttpResponse::NotFound().body("Identifier not found.")
        }
        Err(_) => {
            HttpResponse::InternalServerError().body("Database error occurred")
        }
    }
}