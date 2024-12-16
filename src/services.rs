use reqwest::Client;
use crate::models::ApiResponse;

pub struct ApiService {
    client: Client,
    auth_url: String,
    auth_header: String,
}

impl ApiService {
    /// Creates a new instance of `ApiService` with predefined URLs and credentials.
    pub fn new() -> Self {
        let client = Client::new();
        let auth_url = "https://sandbox.monnify.com/api/v1/auth/login".to_string();
        let auth_header = "TUtfVEVTVF8zVFNEWjdWN0JYOjFYSDM2TjhFWEREQlIzWDVNWjBSME5GRFdDODdOWTEy".to_string();

        ApiService {
            client,
            auth_url,
            auth_header,
        }
    }

    /// Fetches an access token from the Monnify API.
    pub async fn get_access_token(&self) -> Result<ApiResponse, String> {
        // Make the API request
        let response = self
            .client
            .post(&self.auth_url)
            .header("Authorization", format!("Basic {}", self.auth_header))
            .send()
            .await
            .map_err(|e| format!("Request Error: {}", e))?;
    
        // Check the response status
        if response.status().is_success() {
            // Log the raw response body to inspect it
            let response_body = response.text().await.map_err(|e| format!("Read Error: {}", e))?;
            println!("Response body: {}", response_body);
    
            // Deserialize the API response into `ApiResponse`
            let api_response: ApiResponse = serde_json::from_str(&response_body)
                .map_err(|e| format!("Parse Error: {}", e))?;
    
            Ok(api_response)
        } else {
            Err(format!("API Error: {}", response.status()))
        }
    }
    
}
