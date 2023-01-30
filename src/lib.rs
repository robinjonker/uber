use reqwest::Client;
use reqwest::header::{HeaderValue, CONTENT_TYPE};

// Auth

// 1. Auth: POST https://login.uber.com/oauth/v2/token
/// Authentication - returns access token
/// 
/// # Input Parameters
/// 
/// | Parameter                | Description                                                                                                                                                                             |
/// |--------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|
/// | client_id (required)     | The Client ID of your application, retrieved from the Direct Dashboard.                                                                                                                 |
/// | client_secret (required) | The Client Secret of your application. This should be treated like your application password.                                                                                           |
/// | grant_type               | To access the Uber Direct API, authenticate your application by setting this to the client_credentials grant type. This will create an OAuth 2.0 access token with the specified scope. |
/// | scope                    | Specifies the Uber developer endpoints that this token has access to. For Uber Direct, the scope will always be “eats.deliveries”.                                                      |
/// 
pub async fn auth(client_id: &str, client_secret: &str, grant_type: Option<&str>, scope: Option<&str>) -> Result<String, reqwest::Error> {
    let client = Client::new();
    let url = "https://login.uber.com/oauth/v2/token";
    let grant_type = grant_type.unwrap_or("client_credentials");
    let scope = scope.unwrap_or("eats.deliveries");
    let payload = format!("client_id={}&client_secret={}&grant_type={}&scope={}", client_id, client_secret, grant_type, scope);
    let content_type = HeaderValue::from_str("application/x-www-form-urlencoded").unwrap();

    let res = client.post(url)
        .header(CONTENT_TYPE, content_type)
        .body(payload)
        .send()
        .await?;

    let text = res.text().await?;
    Ok(text)
}

// DaaS

// 2. Create Quote: POST https://api.uber.com/v1/customers/{customer_id}/delivery_quotes
/// Create Quote - returns quote details for trips betweeen two locations
/// 
/// # Request Path Parameters
/// 
/// | Name        | Type   | Description                                                                |
/// |-------------|--------|----------------------------------------------------------------------------|
/// | customer_id | string | Unique identifier for the organization. Either UUID or starts with `cus_`. |
/// 
/// # Request Body Parameters


// 3. Create Delivery POST https://api.uber.com/v1/customers/{customer_id}/deliveries
// 4. Get Delivery GET https://api.uber.com/v1/customers/{customer_id}/deliveries/{delivery_id}
// 5. Update Delivery POST https://api.uber.com/v1/customers/{customer_id}/deliveries/{delivery_id}
// 6. Cancel Delivery POST https://api.uber.com/v1/customers/{customer_id}/deliveries/{delivery_id}/cancel
// 7. List Deliveries GET https://api.uber.com/v1/customers/{customer_id}/deliveries
// 8. POD Retrieval POST https://api.uber.com/v1/customers/{customer_id}/deliveries/{delivery_id}/proof-of-delivery
// 9. Delivery Status Notification WEBHOOK: POST https://<YOUR_WEBHOOK_URI> event_type: event.delivery_status
// 10. Courier Update Notification WEBHOOK: POST https://<YOUR_WEBHOOK_URI> event_type: event.courier_update

// Direct

// 11. Find Deliverable Stores GET https://api.uber.com/v1/eats/deliveries/stores
// 12. Get Delivery Estimate POST https://api.uber.com/v1/eats/deliveries/estimates
// 13. Create Delivery POST https://api.uber.com/v1/eats/deliveries/orders
// 14. Get Delivery Status GET https://api.uber.com/v1/eats/deliveries/orders/{order_id}
// 15. Cancel Order POST https://api.uber.com/v1/eats/orders/{order_id}/cancel
// 16. Delivery Status Notification WEBHOOK: POST https://<YOUR_WEBHOOK_URI> event_type: dapi.status_changed
// 17. Refund Request Notification WEBHOOK: POST https://<YOUR_WEBHOOK_URI> event_type: dapi.refund_requested