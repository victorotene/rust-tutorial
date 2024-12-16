use actix_web::{web, App, HttpServer};
use std::sync::Arc;

mod controller;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the API service (Arc to allow shared ownership across async tasks)
    let api_service = Arc::new(services::ApiService::new());

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(api_service.clone())) // Share the api_service across handlers
            .service(controller::get_access_token) // Expose the controller's endpoint
    })
    .bind("127.0.0.1:3000")? // Bind to the address and port
    .run() // Run the server
    .await
}
