use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BankResponse {
    pub request_successful: bool,
    pub response_message: String,
    pub response_code: String,
    pub response_body: Vec<Bank>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Bank {
    pub name: String,
    pub code: String,
    pub ussd_template: Option<String>,
    pub base_ussd_code: Option<String>,
    pub transfer_ussd_template: Option<String>,
    pub bank_id: Option<String>,
    pub nip_bank_code: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponse {
    //pub request_successful: String, // Note: Changed to snake_case to match your services.rs
    //pub response_message: String,
    //pub response_code: String,
    pub response_body: AuthResponseBody,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResponseBody {
    pub access_token: String,
    // Note: Added `expires_in` to match the JSON structure
    //pub expires_in: Option<u32>, 
}