use actix_web::{get, web, HttpResponse, Responder};
use std::sync::Arc;
use crate::services::ApiService;

#[get("/access_token")]
pub async fn get_access_token(api_service: web::Data<Arc<ApiService>>) -> impl Responder {
    match api_service.get_access_token().await {
        Ok(api_response) => HttpResponse::Ok().json(api_response),
        Err(error) => HttpResponse::InternalServerError().body(error),
    }
}


