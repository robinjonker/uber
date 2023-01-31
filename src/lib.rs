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
/// | client_id                | The Client ID of your application, retrieved from the Direct Dashboard.                                                                                                                 |
/// | client_secret            | The Client Secret of your application. This should be treated like your application password.                                                                                           |
/// | grant_type               | To access the Uber Direct API, authenticate your application by setting this to the client_credentials grant type. This will create an OAuth 2.0 access token with the specified scope. |
/// | scope                    | Specifies the Uber developer endpoints that this token has access to. For Uber Direct, the scope will always be “eats.deliveries”.                                                      |
/// 
/// # Authentication Error Codes
/// 
/// | Parameter         | Description                                   |
/// |-------------------|-----------------------------------------------|
/// | invalid_request   | Required parameters were not provided.        |
/// | invalid_client    | The client ID or secret provided is invalid.  |
/// | invalid_scope     | The scope provided is invalid                 |
/// | server_error      | The server returned an unknown error.         |
/// | unauthorized      | Invalid OAuth 2.0 credentials provided.       |
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

// 2. Create Quote: # POST https://api.uber.com/v1/customers/{customer_id}/delivery_quotes
/// Create Quote - returns quote details for trips betweeen two locations
/// 
/// # Request Path Parameters
/// 
/// | Name        | Type   | Description                                                                |
/// |-------------|--------|----------------------------------------------------------------------------|
/// | customer_id | string | Unique identifier for the organization. Either UUID or starts with `cus_`. |
/// 
/// # Request Body Parameters
/// 
/// | Name | Type | Description |
/// | :--- | :--- | :--- |
/// | dropoff_address | string | (required) Dropoff address in Street Address, City, State, Zip format
/// | pickup_address | string | (required) Pickup address in Street Address, City, State, Zip format
/// | dropoff_latitude | double | Dropoff latitude coordinate
/// | dropoff_longitude | double | Dropoff longitude coordinate
/// | dropoff_phone_number | string | Phone number of recipient |
/// | pickup_latitude | double | Pickup latitude coordinate |
/// | pickup_longitude | double | Pickup longitude coordinate |
/// | pickup_phone_number  | string | Phone number of sender/store |  
/// | pickup_ready_dt | timestamp (RFC 3339) | Beginning of the window when an order must be picked up. Must be less than 30 days in the future. |
/// | pickup_deadline_dt | timestamp (RFC 3339) | End of the window when an order may be picked up. Must be at least 10 mins later than pickup_ready_dt and at least 20 minutes in the future from now. |
/// | dropoff_ready_dt | timestamp (RFC 3339) | Beginning of the window when an order must be dropped off. Must be less than or equal to pickup_deadline_dt |
/// | dropoff_deadline_dt | timestamp (RFC 3339) | End of the window when an order must be dropped off. Must be at least 20 mins later than dropoff_ready_dt and must be greater than or equal to pickup_deadline_dt. |
/// | manifest_total_value | integer | Value (in US cents) of the items in the delivery. i.e.: $10.99 => 1099. |
/// | external_store_id | string | (Optional) Unique identifier used by our Partners to reference a Store or Location |
/// 
/// # Response Body Parameters
/// 
/// | Name | Type | Description |
/// | :--- | :--- | :--- |
/// | created | timestamp (RFC 3339) | Date/Time at which the quote was created. |
/// | currency | string | [DEPRECATED] Three-letter ISO currency code, in lowercase. |
/// | currency_type | string | Three-letter ISO currency code, in uppercase. |
/// | dropoff_deadline | timestamp (RFC 3339) | When a delivery must be dropped off. This is the end of the dropoff window. |
/// | dropoff_eta | timestamp (RFC 3339) | Estimated drop-off time. |
/// 
/// # Endpoint Specific Errors
/// 
/// | Http Status | Code | Code Message |
/// | :--- | :--- | :--- |
/// | 400 | invalid_params | The parameters of your request were invalid. |
/// | 400 | unknown_location | The specified location was not understood. |
/// | 400 | address_undeliverable | The specified location is not in a deliverable area. |
/// | 400 | pickup_window_too_small | The pickup window needs to be at least 10 minutes long. |
/// | 400 | dropoff_deadline_too_early | The dropoff deadline needs to be at least 20 minutes after the dropoff ready time. |
/// | 400 | dropoff_deadline_before_pickup_deadline | The dropoff deadline needs to be after the pickup deadline. |
/// | 400 | dropoff_ready_after_pickup_deadline | The dropoff ready time needs to be at or before the pickup deadline. |
/// | 400 | pickup_ready_too_early | The pickup ready time cannot be in the past. |
/// | 400 | pickup_deadline_too_early | The pickup deadline time needs to be at least 20 minutes from now. |
/// | 400 | pickup_ready_too_late | The pickup ready time needs to be within the next 30 days. |
/// | 404 | customer_not_found | Customer does not exist.  |
/// 
pub async fn create_quote() {

}

// 3. Create Delivery POST https://api.uber.com/v1/customers/{customer_id}/deliveries
///
/// # Request Path Parameters
/// 
/// | Name        | Type   | Description                                                                |
/// |-------------|--------|----------------------------------------------------------------------------|
/// | customer_id | string | Unique identifier for the organization. Either UUID or starts with `cus_`. |
/// 
/// # Request Body Parameters
/// 
/// | Name | Type | Description |
/// | :--- | :--- | :--- |
/// | dropoff_address | string - Structured Address | (required) For single string, format is : "Street Address, City, State, Zip" |
/// | dropoff_name | string | (required) Name of the place where the courier will make the dropoff. |
/// | dropoff_phone_number | string | (required) The phone number of the dropoff location. |
/// | manifest | string | (required) [SOON TO BE DEPRECATED] 'manifest_items` should be used instead. |
/// | manifest_items | Manifestltem[] | (required) List of items being delivered. |
/// | pickup_address | string | (required) Pickup address in Street Address, City, State, Zip format. |
/// | pickup_name | string | (required) Name of the place where the courier will make the pickup. |
/// | pickup_phone_number | string | (required) Name of the place where the courier will make the pickup. |
/// | deliverable_action | DeliverableAction | Specify the "happy path" action for the courier to take on a delivery. When used, delivery action can be set to "leave at door" for a contactless delivery. Cannot leave at door when signature or ID verification requirements are applied when creating a delivery. Photo confirmation of delivery will be automatically applied as a requirement to complete dropoff. |
/// | dropoff_business_name | string | Business name of the dropoff location. |
/// | dropoff_latitude | double | Dropoff latitude coordinate. |
/// | dropoff_longitude | double | Dropoff longitude coordinate. |
/// | dropoff_notes | string | Additional instructions for the courier at the dropoff location. Max 280 characters. |
/// | dropoff_seller_notes | string | Additional instructions provided by the merchant for the dropoff. Max 280 characters. |
/// | dropoff_verification | VerificationRequirement | Verification steps (i.e. barcode scanning) that must be taken before the dropoff can be completed. |
/// | manifest_reference | string | Reference that identifies the manifest. Use this to connect a delivery to corresponding information in your system. |
/// | manifest_total_value | integer | Value (in US cents) of the items in the delivery. i.e.: $10.99=>1099. |
/// | pickup_business_name | string | Business name of the pickup location. |
/// | pickup_latitude | double | Pickup latitude coordinate. |
/// | pickup_longitude | double | Pickup longitude coordinate. |
/// | pickup_notes | string | Additional instructions for the courier at the pickup location. Max 280 characters. |
/// | pickup_verification | VerificationRequirement | Verification steps (i.e. barcode scanning) that must be taken before the pickup can be completed. |
/// | quote_id | string | The ID of a previously generated delivery quote. |
/// | undeliverable_action | UndeliverableAction | Specify the "unhappy path" action for the courier to take on a delivery once a normal delivery attempt is made and a customer is not available. |
/// | pickup_ready_dt | timestamp (RFC 3339) | Beginning of the window when an order must be picked up. Must be less than 30 days in the future. |
/// | pickup_deadline_dt | timestamp (RFC 3339) | End of the window when an order may be picked up. Must be at least 10 mins later than pickup_ready_dt and at least 20 minutes in the future from now. |
/// | dropoff_ready_dt | timestamp (RFC 3339) | Beginning of the window when an order must be dropped off. Must be less than or equal to pickup_deadline_dt . |
/// | dropoff_deadline_dt | timestamp (RFC 3339) | End of the window when an order must be dropped off. Must be at least 20 mins later than dropoff_ready_dt and must be greater than or equal to pickup_deadline_dt. |
/// | requires_dropoff_signature | boolean | [DEPRECATED] Flag to indicate this delivery requires signature capture at dropoff. |
/// | requires_id | boolean | Flag to indicate this delivery requires ID check (minimum age) at dropoff |
/// | tip | integer | Upfront tip amount. 0.01 of the national currency (cents in US or $0.01 ) |
/// | idempotency_key | string | A key which is used to avoid duplicate order creation with identical idempotency keys for the same account. The key persists for a set time frame, defaulting to 6 hours |
/// | external_store_id | string | (Optional) Unique identifier used by our Partners to reference a Store or Location |
/// | return_verification | VerificationRequirement | Verification steps (barcode scanning, picture, or signature) that must be taken before the return can be completed. |
/// 
/// # Request Body Parameters - Structured Address
/// 
/// | Name | Type | Description |
/// | :--- | :--- | :--- |
/// | street_address | array[string] | Array of street address elements. For example: ["2000 Ocean Ave", "Floor 2" ] |
/// | city | string |  |
/// | state | string |  |
/// | zip_code | string |  |
/// | country | string | (optional) |

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