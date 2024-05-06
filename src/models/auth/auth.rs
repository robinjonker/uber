use serde::{Deserialize, Serialize};

/// # Input Parameters
///
/// | Parameter                | Description                                                                                                                                                                             |
/// |--------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | client_id                | The Client ID of your application, retrieved from the Direct Dashboard.                                                                                                                 |
/// | client_secret            | The Client Secret of your application. This should be treated like your application password.                                                                                           |
/// | grant_type               | To access the Uber Direct API, authenticate your application by setting this to the client_credentials grant type. This will create an OAuth 2.0 access token with the specified scope. |
/// | scope                    | Specifies the Uber developer endpoints that this token has access to. For Uber Direct, the scope will always be “eats.deliveries”.                                                      |
///
#[derive(Serialize, Default, Debug)]
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