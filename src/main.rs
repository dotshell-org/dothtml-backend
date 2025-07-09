mod routes;
mod handlers;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(routes::config) // Add routes via the config function from the routes module
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
