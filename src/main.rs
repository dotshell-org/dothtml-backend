mod routes;
mod handlers;
mod database;
mod models;

use actix_web::{web, App, HttpServer};
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
    
    // Create messages table if it doesn't exist
    db.create_messages_table().await
        .expect("Failed to create messages table");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // Share database instance across handlers
            .configure(routes::config) // Configure routes from the routes module
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
