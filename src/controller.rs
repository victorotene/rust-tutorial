use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;
use crate::services::ApiService;

#[get("/banks")]
pub async fn get_banks(api_service: web::Data<Arc<ApiService>>) -> impl Responder {
    match api_service.get_banks().await {
        Ok(banks) => HttpResponse::Ok().json(banks),  // Return the list of banks as JSON
        Err(err) => HttpResponse::InternalServerError().body(err),  // Return the error message
    }
}
