use crate::models::{AuthResponse, BankResponse};
use reqwest::Client;

pub struct ApiService {
    client: Client,
    auth_url: String,
    bank_url: String,
    auth_header: String,
}

impl ApiService {
    /// Creates a new instance of `ApiService` with predefined URLs and credentials.
    pub fn new() -> Self {
        let client = Client::new();
        let auth_url = "https://sandbox.monnify.com/api/v1/auth/login".to_string();
        let bank_url = "https://sandbox.monnify.com/api/v1/banks".to_string();
        let auth_header = "TUtfVEVTVF8zVFNEWjdWN0JYOjFYSDM2TjhFWEREQlIzWDVNWjBSME5GRFdDODdOWTEy".to_string();

        ApiService {
            client,
            auth_url,
            bank_url,
            auth_header,
        }
    }

    /// Fetches an access token using the predefined `auth_url` and `auth_header`.
    pub async fn get_access_token(&self) -> Result<String, String> {
        let response = self
            .client
            .post(&self.auth_url)
            .header("Authorization", format!("Basic {}", self.auth_header))
            .send()
            .await
            .map_err(|e| format!("Request Error: {}", e))?;

        let response_body: AuthResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        // Return the access token
        Ok(response_body.response_body.access_token)
    }

    /// Fetches the list of banks using the predefined `bank_url` and the retrieved access token.
    pub async fn get_banks(&self) -> Result<BankResponse, String> {
        let token = self.get_access_token().await?; // Fetch the access token

        let response = self
            .client
            .get(&self.bank_url)
            .bearer_auth(token) // Use the token for authorization
            .send()
            .await
            .map_err(|e| format!("Request Error: {}", e))?;

        let response_body: BankResponse = response
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        // Return the deserialized response
        Ok(response_body)
    }
}
