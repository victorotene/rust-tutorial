use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use actix_web::middleware::Logger;
use std::sync::Arc;
use flexi_logger::{Logger, WriteMode};
use std::fs::File;
use std::io::{self, Write};

mod controller;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Test if we can write to Downloads directory
    if let Err(e) = File::create("/Users/victorotene/Downloads/test.log") {
        eprintln!("Error creating test file: {}", e);
    }

    // Initialize Logger (write logs to the Downloads directory)
    Logger::try_with_str("info") // Adjust log level as needed
        .unwrap()
        .log_to_file()
        .write_mode(WriteMode::BufferAndFlush)
        .directory("/Users/victorotene/Downloads") // Log files will go here
        .basename("application_logs") // Base name for the log files
        .suffix("log") // Log file extension
        .start()
        .unwrap();

    // Initialize the API service (Arc to allow shared ownership across async tasks)
    let api_service = Arc::new(services::ApiService::new());

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(api_service.clone()))  // Share the api_service across handlers
            .wrap(Logger::default())  // Optional: adds logging middleware for requests
            .service(controller::get_banks)  // Expose the controller's endpoint
    })
    .bind("127.0.0.1:3000")?  // Bind to the address and port
    .run()  // Run the server
    .await
}
