mod routes;
mod handlers;
mod database;
mod models;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use database::Database;

/// Main application entry point.
/// 
/// This function initializes the database connection, tests connectivity,
/// and starts the HTTP server with all configured routes and middleware.
/// 
/// # Returns
/// 
/// Returns a `Result` indicating success or failure of the server startup.
/// 
/// # Errors
/// 
/// This function will return an error if:
/// - Database connection cannot be established
/// - Server binding fails
/// - Configuration issues occur
/// 
/// # Examples
/// 
/// Run the application:
/// ```bash
/// cargo run
/// ```
/// 
/// The server will start on `http://127.0.0.1:8080`
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize database connection
    let db = Database::new().await
        .expect("Failed to connect to database");
    
    // Test database connectivity
    db.test_connection().await
        .expect("Database connection test failed");
    
    // Try to create messages table, ignore if it already exists
    if let Err(e) = db.create_messages_table().await {
        match e {
            sqlx::Error::Database(ref err) if err.code().as_deref() == Some("42P07") => {
                println!("Messages table already exists, continuing...");
            }
            _ => return Err(std::io::Error::new(std::io::ErrorKind::Other, e)),
        }
    }

    // Start HTTP server
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("https://dotshell.eu")  // Production domain
            .allowed_origin("http://dotshell.ddns.net:4000")  // Development domain
            .allowed_origin("http://localhost:4000")  // Local development
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec!["Content-Type"])
            .max_age(3600)
            .supports_credentials();

        App::new()
            .wrap(cors)  // Ajouter le middleware CORS
            .app_data(web::Data::new(db.clone())) // Share database instance across handlers
            .configure(routes::config) // Configure routes from the routes module
    })
        .bind("0.0.0.0:8080")?  // Bind to all network interfaces
        .run()
        .await
}
