use serde::{Deserialize, Serialize};

#[derive(Serialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AuthRequest {
    pub client_id: String,
    pub client_secret: String,
    pub grant_type: Option<String>,
    pub scope: Option<String>,
}

#[derive(Deserialize ,Debug)]
#[serde(rename_all = "snake_case")]
pub struct AuthResponse {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: String,
}

impl AuthRequest {
    pub fn new<T: Into<String>>(client_id: T, client_secret: T) -> Self {
        AuthRequest {
            client_id: client_id.into(),
            client_secret: client_secret.into(),
            grant_type: Some("client_credentials".to_string()),
            scope: Some("eats.deliveries".to_string()),
        }
    }
}