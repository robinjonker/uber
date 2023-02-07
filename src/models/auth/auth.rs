use serde::{Deserialize, Serialize};

#[derive(Serialize)]
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