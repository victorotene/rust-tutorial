use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]  // Apply camelCase renaming to all fields
pub struct ApiResponse {
    pub request_successful: Option<bool>,
    pub response_message: Option<String>,
    pub response_code: Option<String>,
    pub response_body: Option<ResponseBody>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseBody {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub expires_in: i64,
}
