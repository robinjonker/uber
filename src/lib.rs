#[macro_use]
extern crate serde_derive;

use reqwest::{Client, Error};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::header::AUTHORIZATION;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};


// Auth

////////////////////////////////////////////////////////////////////////////////////////////////
// 1. Auth: POST https://login.uber.com/oauth/v2/token
////////////////////////////////////////////////////////////////////////////////////////////////

// Structs:

#[derive(Serialize)]
pub struct AuthRequest {
    client_id: String,
    client_secret: String,
    grant_type: Option<String>,
    scope: Option<String>,
}
/// Retrieve access token for authenticated user
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
pub async fn auth(
    client_id: &str,
    client_secret: &str,
    grant_type: Option<&str>, 
    scope: Option<&str>
) -> Result<String, Error> {
    let request = AuthRequest {
        client_id: client_id.to_string(),
        client_secret: client_secret.to_string(),
        grant_type: grant_type.map(|s| s.to_string()),
        scope: scope.map(|s| s.to_string()),
    };

    let client = Client::new();
    let url = "https://login.uber.com/oauth/v2/token";
    let content_type = HeaderValue::from_str("application/x-www-form-urlencoded").unwrap();

    let res = client.post(&url)
        .header(CONTENT_TYPE, content_type)
        .json(&request)
        .send()
        .await?;

    let text = res.text().await?;
    Ok(text)
}

// DaaS

////////////////////////////////////////////////////////////////////////////////////////////////
// 2. Create Quote: # POST https://api.uber.com/v1/customers/{customer_id}/delivery_quotes
////////////////////////////////////////////////////////////////////////////////////////////////

// Structs:

#[derive(Serialize)]
pub struct CreateQuoteRequest {
    pickup_address: String,
    dropoff_address: String,
    dropoff_latitude: Option<f64>,
    dropoff_longitude: Option<f64>,
    dropoff_phone_number: Option<String>,
    pickup_latitude: Option<f64>,
    pickup_longitude: Option<f64>,
    pickup_phone_number: Option<String>,
    pickup_ready_dt: Option<String>,
    pickup_deadline_dt: Option<String>,
    dropoff_ready_dt: Option<String>,
    dropoff_deadline_dt: Option<String>,
    manifest_total_value: Option<u32>,
    external_store_id: Option<String>,
}
/// Create a quote to check deliverability, validity and cost for delivery between two addresses.
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
pub async fn create_quote(
    access_token: &str,
    customer_id: &str,
    pickup_address: &str,
    dropoff_address: &str,
    dropoff_latitude: Option<f64>,
    dropoff_longitude: Option<f64>,
    dropoff_phone_number: Option<&str>,
    pickup_latitude: Option<f64>,
    pickup_longitude: Option<f64>,
    pickup_phone_number: Option<&str>,
    pickup_ready_dt: Option<&str>,
    pickup_deadline_dt: Option<&str>,
    dropoff_ready_dt: Option<&str>,
    dropoff_deadline_dt: Option<&str>,
    manifest_total_value: Option<u32>,
    external_store_id: Option<&str>,
) -> Result<String, Error> {
    let request = CreateQuoteRequest {
        pickup_address: pickup_address.to_string(),
        dropoff_address: dropoff_address.to_string(),
        dropoff_latitude,
        dropoff_longitude,
        dropoff_phone_number: dropoff_phone_number.map(|s| s.to_string()),
        pickup_latitude,
        pickup_longitude,
        pickup_phone_number: pickup_phone_number.map(|s| s.to_string()),
        pickup_ready_dt: pickup_ready_dt.map(|s| s.to_string()),
        pickup_deadline_dt: pickup_deadline_dt.map(|s| s.to_string()),
        dropoff_ready_dt: dropoff_ready_dt.map(|s| s.to_string()),
        dropoff_deadline_dt: dropoff_deadline_dt.map(|s| s.to_string()),
        manifest_total_value,
        external_store_id: external_store_id.map(|s| s.to_string()),
    };

    let client = Client::new();
    let url = format!(
        "https://api.uber.com/v1/customers/{}/deliveryquotes",
        customer_id
    );
    let content_type = HeaderValue::from_str("application/json").unwrap();
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header).unwrap();

    let res = client.post(&url)
    .header(CONTENT_TYPE, content_type)
    .header(AUTHORIZATION, authorization)
    .body(&request)
    .send()
    .await?;

    let text = res.text().await?;
    Ok(text)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 3. Create Delivery POST https://api.uber.com/v1/customers/{customer_id}/deliveries
////////////////////////////////////////////////////////////////////////////////////////////////

// Structs:

#[derive(Serialize)]
pub struct CreateDeliveryRequest {
    dropoff_address: String,
    dropoff_name: String,
    dropoff_phone_number: String,
    manifest: String,
    manifest_items: ManifestItem,
    pickup_address: String,
    pickup_name: String,
    pickup_phone_number: String,
    deliverable_action: Option<DeliverableAction>,
    dropoff_business_name: Option<String>,
    dropoff_latitude: Option<f64>, 
    dropoff_longitude: Option<f64>, 
    dropoff_notes: Option<String>,
    dropoff_seller_notes: Option<String>,
    dropoff_verification: Option<VerificationRequirement>,
    manifest_reference: Option<String>,
    manifest_total_value: Option<u32>,
    pickup_business_name: Option<String>,
    pickup_latitude: Option<f64>,
    pickup_longitude: Option<f64>,
    pickup_notes: Option<String>,
    pickup_verification: Option<VerificationRequirement>,
    quote_id: Option<String>,
    undeliverable_action: Option<UndeliverableAction>,
    pickup_ready_dt: Option<DateTime<Local>>,
    pickup_deadline_dt: Option<DateTime<Local>>,
    dropoff_ready_dt: Option<DateTime<Local>>,
    dropoff_deadline_dt: Option<DateTime<Local>>,
    requires_dropoff_signature: Option<bool>,
    requires_id: Option<bool>,
    tip: Option<u32>,
    idempotency_key: Option<String>, 
    external_store_id: Option<String>,
    return_verification: Option<VerificationRequirement>,
    test_specifications: Option<TestSpecifications>,
}

#[derive(Serialize)]
pub struct ManifestItem {
    name: String,
    quantity: u32,
    size: Option<Size>,
    dimensions: Option<Dimensions>,
    price: Option<u32>,
    must_be_upright: Option<bool>,
    weight: Option<u32>,
    perishability: Option<u32>,
    preparation_time: u32,
}

#[derive(Serialize)]
pub struct DeliverableAction {
    deliverable_action_meet_at_door: String,
    deliverable_action_leave_at_door: String,
}

#[derive(Serialize)]
pub struct VerificationRequirement {
    signature: Option<bool>,
    signature_requirement: Option<SignatureRequirement>,
    barcodes: Option<BarcodeRequirement>,
    pincode: Option<PincodeRequirement>,
    package: Option<PackageRequirement>,
    identification: Option<IdentificationRequirement>,
    picture: Option<bool>,
}

#[derive(Serialize)]
pub struct UndeliverableAction {
    leave_at_door: String,
    return_order: String,
}

#[derive(Serialize)]
pub struct Size {
    small: String,
    medium: String,
    large: String,
    xlarge: String,
}

#[derive(Serialize)]
pub struct Dimensions {
    length: Option<u32>,
    height: Option<u32>,
    depth: Option<u32>,
}

#[derive(Serialize)]
pub struct SignatureRequirement {
    enabled: bool,
    collect_signer_name: bool,
    collect_signer_relationship: bool,
}

#[derive(Serialize)]
pub struct BarcodeRequirement {
    value: String,
    type_of_barcode: String,
}

#[derive(Serialize)]
pub struct PincodeRequirement {
    enabled: bool,
    value: String,
}

#[derive(Serialize)]
pub struct PackageRequirement {
    bag_count: u32,
    drink_count: u32,
}

#[derive(Serialize)]
pub struct IdentificationRequirement {
    min_age: u32,
}

#[derive(Serialize)]
pub struct TestSpecifications {
    robo_courier_specification: RoboCourierSpecification,
}

#[derive(Serialize)]
pub struct RoboCourierSpecification {
    mode: String,
}
/// Create a delivery between two addresses.
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
/// 
/// # Request Body Parameters - ManifestItem
/// 
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |name|	string|	Description of item.|
/// |quantity|	integer|	Quantity of items.|
/// |size|	Size|	Approximate size of item. Specifying `xlarge` will cause dispatch to only couriers using a car or larger (no walkers/bikes/scooters/etc…).|
/// |dimensions|	Dimensions	|[optional] Struct that contains dimensions|
/// |price|	integer|	[optional] The price of the item. The value passed in should be based on 0.01 unit of the local currency. For example, in the US, a value of 1 would mean a cent ($0.01), and a value of 100 would mean a dollar ($1.00).|
/// |must_be_upright|	boolean|	[optional] Whether the item should be in upright position (box of pizza, bottle of milk etc)|
/// |weight|	integer|	[optional] Weight in grams|
/// |perishability|	integer	|[optional] Perishability represents the number of minutes before an item expires. For example, an ice cream might have a perishability of 15 minutes from pickup|
/// |preparation_time|	integer|	[optional] How long a given item takes to prepare (in minutes)|
/// 
/// ### ManifestItem - Size
/// 
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |small	|string	|You can carry it with one hand e.g. bottle of water.|
/// |medium	|string	|You need a tote bag to carry it e.g. retail bag.|
/// |large	|string	|You need two hands to carry it e.g. computer monitor.|
/// |xlarge	|string	|You will need to make multiple trips to/from a vehicle to transport e.g. grocery order. Specifying `xlarge` will cause dispatch to only couriers using a car or larger (no walkers/bikes/scooters/etc…).|
/// |big	|string	|[DEPRECATED] Same as large.|
/// 
/// ### ManifestItem - Dimensions
/// 
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |length	|integer	|[optional] Length in centimeters|
/// |height	|integer	|[optional] Height in centimeters|
/// |depth	|integer	|[optional] Depth in centimeters|
/// 
/// # Request Body Parameters - DeliverableAction
/// 
/// |Name|	Type|	Description|
/// | :--- | :--- | :--- |
/// |deliverable_action_meet_at_door|	string|	Meet at door delivery. This is the default if DeliverableAction is not set.|
/// |deliverable_action_leave_at_door|	string	|The “happy path” action for the courier to take on a delivery. When used, delivery action can be set to “leave at door” for a contactless delivery. Cannot leave at door when signature or ID verification requirements are applied when creating a delivery. Photo confirmation of delivery will be automatically applied as a requirement to complete drop-off.|
/// 
/// # Request Body Parameters - VerificationRequirement
/// 
/// |Name	|Type|	Description|
/// | :--- | :--- | :--- |
/// |signature|	boolean|	[DEPRECATED] Flag for if a signature is required at this waypoint. signature_requirement should be used instead.|
/// |signature_requirement|	SignatureRequirement[]	|Signature requirement spec to indicate that a signature must be collected at this waypoint.|
/// |barcodes	|BarcodeRequirement[]|	Barcode values/types that must be scanned at the waypoint. Number of elements in the array is equal to the number of barcodes that must be scanned.|
/// |pincode|	PincodeRequirement|	Pincode requirement spec to indicate a delivery requires pincode confirmation upon delivery|
/// |package	|PackageRequirement	|Package verifications required for this waypoint.|
/// |identification	|IdentificationRequirement	|Identification scanning/verification requirements for this waypoint…|
/// |picture	|boolean| |
/// 
/// ### VerificationRequirement - SignatureRequirement
/// 
/// |Name|	Type|	Description|
/// | :--- | :--- | :--- |
/// |enabled	|boolean	|Flag for if a signature is required at this waypoint.|
/// |collect_signer_name|	boolean|	Flag for if the signer’s name is required at this waypoint.|
/// |collect_signer_relationship|	boolean|	Flag for if the signer’s relationship to the intended recipient is required at this waypoint.|
/// 
/// ### VerificationRequirement - BarcodeRequirement
/// 
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |value	|string	|String value encoded in the barcode.|
/// |type	|string	|Type of barcode. Valid values: “CODE39”, “CODE39_FULL_ASCII”, “CODE128”, “QR”.|
/// 
/// ### VerificationRequirement - PincodeRequirement
/// 
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |enabled	|bool	|When set to true in POST requests, the delivery will require pincode entry at handoff.|
/// |value	|string	|The pincode that the customer must present at dropoff. This field will be ignored in the CreateDelivery requests, and the pin code is internally generated when this requirement is present.|
/// 
/// ### VerificationRequirement - PackageRequirement
/// 
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |bag_count	|integer	|Number of bags to be picked up.|
/// |drink_count	|integer	|Number of drinks to be picked up.|
/// 
/// ### VerificationRequirement - IdentificationRequirement
///
/// |Name	|Type|	Description|
/// | :--- | :--- | :--- |
/// |min_age	|integer|	Minimum age that must be verified for this delivery.|
/// 
/// # Request Body Parameters - UndeliverableAction
/// 
/// |Name	|Type|	Description|
/// | :--- | :--- | :--- |
/// |leave_at_door|	string	|Specify the “unhappy path” action for the courier to take on a delivery once a normal delivery attempt is made and a customer is not available. Cannot leave at door when signature or ID verification requirements are applied when creating a delivery. Photo confirmation of delivery will be automatically applied as a requirement to complete drop-off.|
/// |return|	string	|Specify the “unhappy path” action for the courier to take on a delivery once a normal delivery attempt is made and a customer is not available. This action requests the courier to return the delivery to the pickup waypoint.|
/// 
pub async fn create_delivery(
    access_token: &str,
    customer_id: &str,
    dropoff_address: &str,
    dropoff_name: &str,
    dropoff_phone_number: &str,
    manifest: &str,
    manifest_items: ManifestItem,
    pickup_address: &str,
    pickup_name: &str,
    pickup_phone_number: &str,
    deliverable_action: Option<DeliverableAction>,
    dropoff_business_name: Option<&str>,
    dropoff_latitude: Option<f64>,
    dropoff_longitude: Option<f64>,
    dropoff_notes: Option<&str>,
    dropoff_seller_notes: Option<&str>,
    dropoff_verification: Option<VerificationRequirement>,
    manifest_reference: Option<&str>,
    manifest_total_value: Option<u32>,
    pickup_business_name: Option<&str>,
    pickup_latitude: Option<f64>,
    pickup_longitude: Option<f64>,
    pickup_notes: Option<&str>,
    pickup_verification: Option<VerificationRequirement>,
    quote_id: Option<&str>,
    undeliverable_action: Option<UndeliverableAction>,
    pickup_ready_dt: Option<DateTime<Local>>,
    pickup_deadline_dt: Option<DateTime<Local>>,
    dropoff_ready_dt: Option<DateTime<Local>>,
    dropoff_deadline_dt: Option<DateTime<Local>>,
    requires_dropoff_signature: Option<bool>,
    requires_id: Option<bool>,
    tip: Option<u32>,
    idempotency_key: Option<&str>, 
    external_store_id: Option<&str>,
    return_verification: Option<VerificationRequirement>,
    test_specifications: Option<TestSpecifications>,
    ) -> Result<String, Error> {
    let request = CreateDeliveryRequest {
        dropoff_address: dropoff_address.to_string(),
        dropoff_name: dropoff_name.to_string(),
        dropoff_phone_number: dropoff_phone_number.to_string(),
        manifest: manifest.to_string(),
        manifest_items,
        pickup_address: pickup_address.to_string(),
        pickup_name: pickup_name.to_string(),
        pickup_phone_number: pickup_phone_number.to_string(),
        deliverable_action,
        dropoff_business_name: dropoff_business_name.map(|s| s.to_string()),
        dropoff_latitude,
        dropoff_longitude,
        dropoff_notes: dropoff_notes.map(|s| s.to_string()),
        dropoff_seller_notes: dropoff_seller_notes.map(|s| s.to_string()),
        dropoff_verification,
        manifest_reference: manifest_reference.map(|s| s.to_string()),
        manifest_total_value,
        pickup_business_name: pickup_business_name.map(|s| s.to_string()),
        pickup_latitude,
        pickup_longitude,
        pickup_notes: pickup_notes.map(|s| s.to_string()),
        pickup_verification,
        quote_id: quote_id.map(|s| s.to_string()),
        undeliverable_action,
        pickup_ready_dt,
        pickup_deadline_dt,
        dropoff_ready_dt,
        dropoff_deadline_dt,
        requires_dropoff_signature,
        requires_id,
        tip,
        idempotency_key: idempotency_key.map(|s| s.to_string()),
        external_store_id: external_store_id.map(|s| s.to_string()),
        return_verification,
        test_specifications,
    };

    let client = Client::new();
    let url = format!(
        "https://api.uber.com/v1/customers/{}/deliveries",
        customer_id
    );
    let content_type = HeaderValue::from_str("application/json").unwrap();
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header).unwrap();

    let res = client.post(&url)
    .header(CONTENT_TYPE, content_type)
    .header(AUTHORIZATION, authorization)
    .body(&request)
    .send()
    .await?;

    let text = res.text().await?;
    Ok(text)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 4. Get Delivery GET https://api.uber.com/v1/customers/{customer_id}/deliveries/{delivery_id}
////////////////////////////////////////////////////////////////////////////////////////////////

// Structs:

/// Retrieve the current status of an existing delivery
/// 
/// # Request Path Parameters
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |customer_id	|string|	Unique identifier for the organization. Either UUID or starts with `cus_`.|
/// |delivery_id	|string	|Unique identifier for the delivery. Always starts with `del_`.|

pub async fn get_delivery(
    access_token: &str,
    customer_id: &str,
    delivery_id: &str,
    ) -> Result<String, Error> {

    let client = Client::new();
    let url = format!(
        "https://api.uber.com/v1/customers/{}/deliveries/{}",
        customer_id,
        delivery_id
    );
    let content_type = HeaderValue::from_str("application/json").unwrap();
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header).unwrap();

    let res = client.get(&url)
    .header(CONTENT_TYPE, content_type)
    .header(AUTHORIZATION, authorization)
    .send()
    .await?;

    let text = res.text().await?;
    Ok(text)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 5. Update Delivery POST https://api.uber.com/v1/customers/{customer_id}/deliveries/{delivery_id}
////////////////////////////////////////////////////////////////////////////////////////////////

// Structs:

#[derive(Serialize)]
pub struct UpdateDeliveryRequest {
    dropoff_notes: Option<String>,
    dropoff_seller_notes: Option<String>,
    dropoff_verification: Option<VerificationRequirement>,
    manifest_reference: Option<String>,
    pickup_notes: Option<String>,
    pickup_verification: Option<VerificationRequirement>,
    requires_dropoff_signature: Option<bool>,
    requires_id: Option<bool>,
    tip_by_customer: Option<u32>,
    dropoff_latitude: Option<f64>,
    dropoff_longitude: Option<f64>,
}
/// Modify an ongoing delivery.
/// 
/// # Request Body Parameters
/// 
/// |Name|	Type	|Description|
/// | :--- | :--- | :--- |
/// |dropoff_notes|	string	|Additional instructions for the courier at the dropoff location. Max 280 characters.|
/// |dropoff_seller_notes|	string|	Additional instructions provided by the merchant for the dropoff. Max 280 characters.|
/// |dropoff_verification|	VerificationRequirement|	Verification steps (i.e. barcode scanning) that must be taken before the dropoff can be completed.|
/// |manifest_reference	|string	|Reference that identifies the manifest. Use this to connect a delivery to corresponding information in your system.|
/// |pickup_notes	|string|	Additional instructions for the courier at the pickup location. Max 280 characters.|
/// |pickup_verification	|VerificationRequirement	|Verification steps (i.e. barcode scanning) that must be taken before the pickup can be completed.|
/// |requires_dropoff_signature	|boolean|	Flag to indicate this delivery requires signature capture at dropoff.|
/// |requires_id	|boolean	|Flag to indicate this delivery requires ID verification.|
/// |tip_by_customer	|integer|	Amount in cents that will be paid to the courier as a tip.|
/// |dropoff_latitude	|double|	Dropoff latitude coordinate.|
/// |dropoff_longitude	|double|	Dropoff longitude coordinate.|
/// 
/// ### VerificationRequirement - docs at create_delivery
/// 
/// # Request Validity
/// ### Business rules for when request will be valid:
/// 
/// |Parameter|	Delivery created|	Pickup started|	Pickup imminent|	Pickup complete|	Dropff started|	Dropoff imminent|	Dropoff complete|
/// | :--- | :--- | :--- |:--- |:--- |:--- |:--- |:--- |
/// |manifest reference|	edit|	edit|	-|	-|	-	|-	|-|
/// |manifest items|	edit|	edit|	-	|-|	-|	-|	-|
/// |dropoff_latitude|	edit|	edit|	-|	-|	-|	-|	-|
/// |dropoff_longitude|	edit|	edit|	-|	-|	-|	-|	-|
/// |pickup_notes|	edit|	edit|	-|	-	|-	|-	|-|
/// |pickup_verification.barcodes|	edit	|edit|	edit|	-|	-	|-|	-|
/// |dropoff_notes|	edit|	edit|	edit|	edit|	edit|	-|	-|
/// |dropoff_seller_notes|	edit|	edit|	edit|	edit|	edit|	-|	-|
/// |dropoff_verification.barcodes|	edit|	edit|	edit|	edit|	edit|	-|	-|
/// |dropoff_verification.signature_requirement|	edit|	edit|	edit|	edit|	edit|	-|	-|
/// |dropoff_verification.identification	remove|	remove|	remove|	remove|	remove|	-|	-|
/// |dropoff_verification.pincodes|	edit	|edit|	edit|	edit	|edit|	-|	-|
/// |tip_by_customer|	-|	-|	-|	-	|edit|	edit|	edit|
/// 
pub async fn update_delivery(
    access_token: &str,
    customer_id: &str,
    delivery_id: &str,
    dropoff_notes: Option<&str>,
    dropoff_seller_notes: Option<&str>,
    dropoff_verification: Option<VerificationRequirement>,
    manifest_reference: Option<&str>,
    pickup_notes: Option<&str>,
    pickup_verification: Option<VerificationRequirement>,
    requires_dropoff_signature: Option<bool>,
    requires_id: Option<bool>,
    tip_by_customer: Option<u32>,
    dropoff_latitude: Option<f64>,
    dropoff_longitude: Option<f64>,
    ) -> Result<String, Error> {
    let request = UpdateDeliveryRequest {
        dropoff_notes: dropoff_notes.map(|s| s.to_string()),
        dropoff_seller_notes: dropoff_seller_notes.map(|s| s.to_string()),
        dropoff_verification,
        manifest_reference: manifest_reference.map(|s| s.to_string()),
        pickup_notes: pickup_notes.map(|s| s.to_string()),
        pickup_verification,
        requires_dropoff_signature,
        requires_id,
        tip_by_customer,
        dropoff_latitude,
        dropoff_longitude,
    };

    let client = Client::new();
    let url = format!(
        "https://api.uber.com/v1/customers/{}/deliveries/{}",
        customer_id,
        delivery_id
    );
    let content_type = HeaderValue::from_str("application/json").unwrap();
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header).unwrap();

    let res = client.post(&url)
    .header(CONTENT_TYPE, content_type)
    .header(AUTHORIZATION, authorization)
    .body(&request)
    .send()
    .await?;

    let text = res.text().await?;
    Ok(text)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 6. Cancel Delivery POST https://api.uber.com/v1/customers/{customer_id}/deliveries/{delivery_id}/cancel
////////////////////////////////////////////////////////////////////////////////////////////////

// Structs:

/// Cancel an ongoing or previously scheduled delivery.
/// 
/// # Endpoint Specific Errors
/// 
/// |Http Status Code|	Code	|Message|
/// | :--- | :--- | :--- |
/// |400	|noncancelable_delivery|	Delivery cannot be cancelled.|
/// |404	|customer_not_cound|	Customer does not exist.|
/// |404	|delivery_not_found|	The requested delivery does not exist.|
/// |408	|request_timeout|	The request timed out.|
/// |500	|unknown_error|	An unknown error happened.|
/// |503	|service_unavailable	|Service is currently unavailable.|
/// 
pub async fn cancel_delivery(
    access_token: &str,
    customer_id: &str,
    delivery_id: &str,
    ) -> Result<String, Error> {

    let client = Client::new();
    let url = format!(
        "https://api.uber.com/v1/customers/{}/deliveries/{}/cancel",
        customer_id,
        delivery_id
    );
    let content_type = HeaderValue::from_str("application/json").unwrap();
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header).unwrap();

    let res = client.post(&url)
    .header(CONTENT_TYPE, content_type)
    .header(AUTHORIZATION, authorization)
    .send()
    .await?;

    let text = res.text().await?;
    Ok(text)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 7. List Deliveries GET https://api.uber.com/v1/customers/{customer_id}/deliveries
////////////////////////////////////////////////////////////////////////////////////////////////

// Structs:

/// Receive update of information on a delivery
/// 
/// # Query Parameters
/// 
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |filter	|string	|Filter deliveries by delivery state. Valid values are: “pending”, “pickup”, “pickup_complete”, “dropoff”, “delivered”, “canceled”, “returned”, and “ongoing”.|
/// |limit	|integer|	Maximum number of responses to return.|
/// |Offset	|integer|	Offset of response objects for pagination.|
/// 
pub async fn list_deliveries(
    access_token: &str,
    customer_id: &str,
    filter: Option<&str>,
    limit:	Option<u32>,
    offset:	Option<u32>,
) -> Result<String, Error> {

    let client = Client::new();
    let mut url = format!(
        "https://api.uber.com/v1/customers/{}/deliveries",
        customer_id
    );
    if let Some(filter) = filter {
        url = format!("{}?filter={}", url, filter.map(|s| s.to_string()));
    }
    if let Some(limit) = limit {
        url = format!("{}&limit={}", url, limit);
    }
    if let Some(offset) = offset {
        url = format!("{}&offset={}", url, offset);
    }

    let content_type = HeaderValue::from_str("application/json").unwrap();
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header).unwrap();

    let res = client.get(&url)
    .header(CONTENT_TYPE, content_type)
    .header(AUTHORIZATION, authorization)
    .send()
    .await?;

    let text = res.text().await?;
    Ok(text)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 8. POD Retrieval POST https://api.uber.com/v1/customers/{customer_id}/deliveries/{delivery_id}/proof-of-delivery
////////////////////////////////////////////////////////////////////////////////////////////////

// Structs:

#[derive(Serialize)]
pub struct PODRetrievalRequest {
    waypoint: String,
    type_of_waypoint: String,
}
/// Return a Proof-of-Delivery (P.O.D.) File if verification requirement given in create delivery request
/// 
/// If a delivery is created with any verification requirements (e.g.: picture, signature or pincode), the resulting image file is made available to you through our proof-of-delivery endpoint. The endpoint will return a Proof-of-Delivery (P.O.D.) File – A long Base64 string that can be converted to a PNG image (Web Converter (External)). The image will include the following information:
/// 
/// - Delivery Status
/// 
/// - Delivery timestamp
/// 
/// - Uber Order ID
/// 
/// - External Order ID
/// 
/// - Proof Type
/// 
/// - Image (Signature image, picture image, or pincode value)
/// 
/// For the “signature” verification type, signer name or signer relationship will also be included if it is enabled for a delivery.
/// 
/// # Request Body Parameters - 
/// 
/// |Name	|Type|	Description|
/// | :--- | :--- | :--- |
/// |waypoint|	string|	Waypoint can be “pickup” or “dropoff” or “return”.|
/// |type|string|	Type can be “picture” or “signature” or “pincode”.|
/// 
/// # Response Body Parameters
/// 
/// |Name	|Type|	Description|
/// | :--- | :--- |
/// |document|	string|	A long Base64 string representing the image.|
/// 
/// # Endpoint Specific Errors
/// 
/// |Http Status Code	|Code|	Message|
/// | :--- | :--- | :--- |
/// |404	|delivery_not_found	|Cannot find requested proof of delivery.|
/// |400	|invalid_params|	Waypoint, type is invalid.|
/// |404	|customer_not_cound	|Customer does not exist.|
/// |500|	unknown_error|	An unknown error happened.|
/// 
pub async fn pod_retrieval(
    access_token: &str,
    customer_id: &str,
    delivery_id: &str,
    waypoint: &str,
    type_of_waypoint: &str,
    ) -> Result<String, Error> {
    let request = PODRetrievalRequest {
        waypoint: waypoint.to_string(),
        type_of_waypoint: type_of_waypoint.to_string(),
    };

    let client = Client::new();
    let url = format!(
        "https://api.uber.com/v1/customers/{}/deliveries/{}/proof-of-delivery",
        customer_id,
        delivery_id
    );
    let content_type = HeaderValue::from_str("application/json").unwrap();
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header).unwrap();

    let res = client.post(&url)
    .header(CONTENT_TYPE, content_type)
    .header(AUTHORIZATION, authorization)
    .body(&request)
    .send()
    .await?;

    let text = res.text().await?;
    Ok(text)
}

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