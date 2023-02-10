//! # Uber API
//! 
//! ### uber_api is a crate that has the relevant DaaS endpoints to make Uber Direct API calls. This allows you to call the relevant API you want and not have to worry about how Uber wants to receive and handle the data.
//! 
//! - The functions state which API can be called, the full documentation of that endpoint along with all the parameters needed for it, the complete Uber Direct docs - last updated 9/2/2023 - can be found within each function. The general flow is that each endpoint takes a request struct as its parameter. The docs for the direct parameters are also shown at each request struct. Create the request struct with the relevant information and simply pass that into the function to call the Uber Direct API.
//! 
//! - At the time of creation, access to these APIs may require written approval from Uber. Once successful, they will provide you with the relevant customer_id, client_id, and client_secret needed for Authentication.
//! 
//! If you want to test that your authentication codes work, an example has been setup for you to run. Clone the repo: [Uber API](https://www.github.com/robinjonker/uber) and run the main file, ensuring you pass in the relevant auth fields as parameters
//! 
//! Example: 
//! ``` cargo run -- --customer-id="1234" --client-id="xyz" --client-secret="xyz" ```

#[macro_use]
extern crate serde_derive;

use reqwest::{Client};
use reqwest::header::{HeaderValue, CONTENT_TYPE};
use reqwest::header::AUTHORIZATION;

mod errors;
use errors::UberError;

pub mod models;
pub use models::{
    auth::{
        AuthRequest,
        AuthResponse
    },
    create_quote::{
        CreateQuoteRequest,
        CreateQuoteResponse
    },
    create_delivery::{
        CreateDeliveryRequest,
        CreateDeliveryResponse
    },
    get_delivery::{
        GetDeliveryResponse
    },
    update_delivery::{
        UpdateDeliveryRequest,
        UpdateDeliveryResponse
    },
    cancel_delivery::{
        CancelDeliveryResponse
    },
    list_deliveries::{
        ListDeliveriesResponse
    },
    pod_retrieval::{
        PODRetrievalRequest,
        PODRetrievalResponse
    },
};

////////////////////////////////////////////////////////////////////////////////////////////////
// 1. Auth: POST https://login.uber.com/oauth/v2/token
////////////////////////////////////////////////////////////////////////////////////////////////

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
    auth_request: AuthRequest
) -> Result<AuthResponse, UberError> {

    let client = Client::new();
    let url = "https://login.uber.com/oauth/v2/token";
    let content_type = HeaderValue::from_str("application/x-www-form-urlencoded")?;
    let body = serde_urlencoded::to_string(&auth_request)?;

    let res = client.post(&*url)
        .header(CONTENT_TYPE, content_type)
        .body(body)
        .send()
        .await?;

    let response_body = res.text().await?;
    let response_data: AuthResponse = serde_json::from_str(&response_body)?;
    
    Ok(response_data)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 2. Create Quote: # POST https://api.uber.com/v1/customers/{customer_id}/delivery_quotes
////////////////////////////////////////////////////////////////////////////////////////////////

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
/// | duration|	integer|	Estimated minutes for this delivery to reach dropoff|
/// | expires	|timestamp (RFC 3339)	|Date/Time after which the quote will no longer be accepted.|
/// | fee|	integer|	Amount in cents that will be charged if this delivery is created.|
/// | id	|string|	Unique identifier for the quote (always starts with `dqt_`)|
/// | kind	|string|	The type of object being described. Always “delivery_quote”|
/// | pickup_duration	|integer	|Estimated minutes until a courier will arrive at the pickup.|
/// | external_store_id	|string	|Unique identifier used by our Partners to reference a Store or Location|
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
/// | 408	|request_timeout|	The request timed out…|
/// | 402	|customer_suspended	|Your account is passed due. Payment is required.|
/// | 422	|address_undeliverable_limited_couriers|	The specified location is not in a deliverable area at this time because all couriers are currently busy.|
/// | 403	|customer_blocked|	Your account is not allowed to create deliveries.|
/// | 429	|customer_limited|	Your account’s limits have been exceeded.|
/// | 500	|unknown_error|	An unknown error happened.|
///
pub async fn create_quote(
    access_token: &str,
    customer_id: &str,
    create_quote_request: CreateQuoteRequest,
) -> Result<CreateQuoteResponse, UberError> {

    let client = Client::new();
    let url = format!(
        "https://api.uber.com/v1/customers/{}/deliveryquotes",
        customer_id
    );
    let content_type = HeaderValue::from_str("application/json")?;
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header)?;
    let body = serde_json::to_string(&create_quote_request)?;

    let res = client.post(&url)
        .header(CONTENT_TYPE, content_type)
        .header(AUTHORIZATION, authorization)
        .body(body)
        .send()
        .await?;

    let response_body = res.text().await?;
    let response_data: CreateQuoteResponse = serde_json::from_str(&response_body)?;

    Ok(response_data)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 3. Create Delivery POST https://api.uber.com/v1/customers/{customer_id}/deliveries
////////////////////////////////////////////////////////////////////////////////////////////////

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
/// # Response Body Parameters
/// 
/// |Name|	Type|	Description|
/// | :--- | :--- | :--- |
/// |complete|	boolean	|Flag indicating if the delivery is ongoing.|
/// |courier|	CourierInfo|	Information about the courier. Only present when a delivery is in progress.|
/// |courier_imminent|	boolean	|Flag indicating if the courier is close to the pickup or dropoff location.|
/// |created|	timestamp (RFC 3339)|	Date/Time at which the delivery was created.|
/// |currency|	string|	Three-letter ISO currency code, in lowercase.|
/// |dropoff	|WaypointInfo|	Dropoff details.|
/// |dropoff_deadline	|timestamp (RFC 3339)|	When a delivery must be dropped off. This is the end of the dropoff window.|
/// |dropoff_eta	|timestamp (RFC 3339)	|Estimated drop-off time.|
/// |dropoff_identifier	|string	|This field identifies who received delivery at the dropoff location.|
/// |dropoff_ready	|timestamp (RFC 3339)|	When a delivery is ready to be dropped off. This is the start of the dropoff window.|
/// |external_id	|string|	An ID for an account as stored in an external system.|
/// |fee	|integer|	Amount in cents that will be charged if this delivery is created.|
/// |id|	string|	Unique identifier for the delivery ( `del_` + tokenize(uuid)).|
/// |kind|	string	|The type of object being described. Always “delivery”.|
/// |live_mode	|boolean|	Flag that indicates if this is live mode or test mode.|
/// |manifest	|ManifestInfo|	A detailed description of what the courier will be delivering.|
/// |manifest_items	|ManifestItem[]|	List of items being delivered.|
/// |pickup	|WaypointInfo|	The pickup details for the delivery.|
/// |pickup_deadline|	timestamp (RFC 3339)|	When a delivery must be picked up by. This is the end of the pickup window.|
/// |pickup_eta	|timestamp (RFC 3339)|	Estimated time the courier will arrive at the pickup location.|
/// |pickup_ready	|timestamp (RFC 3339)|	When a delivery is ready to be picked up. This is the start of the pickup window.|
/// |quote_id	|string|	ID for the Delivery Quote if one was provided when creating this Delivery.|
/// |related_deliveries	|RelatedDelivery[]|	A collection describing other jobs that share an association. i.e.: a return delivery.|
/// |status|	string|	The current status of the delivery. ALLOWED VALUES: pending,pickup,pickup_complete,dropoff,delivered,canceled,returned,ongoing|
/// |tip	|integer	|Amount in cents that will be paid to the courier as a tip.|
/// |tracking_url|	string	|This url can be used to track the courier during the delivery (unauthenticated page).|
/// |undeliverable_action	|string|	If a delivery was undeliverable, this field will contain the resulting action taken by the courier.|
/// |undeliverable_reason	|string|	If a delivery was undeliverable, this field will contain the reason why it was undeliverable.|
/// |updated	|timestamp (RFC 3339)|	Date/Time at which the delivery was last updated.|
/// |uuid	|string|	Alternative delivery identifier. “Id” field should be used for any identification purposes. “uuid” field is equally unique but loses contextual information (i.e. nothing in this identifier points out that it relates to a delivery). “uuid” is case-sensitive. Value for the “uuid” field is UUID v4 with ‘-’ characters removed.|
/// |return|	WaypointInfo|	The return details for the delivery.|
/// 
/// ### Summary of order identifiers above
/// | :--- | :--- | :--- |
/// |ID	|Description|	Use case|
/// |External Order ID (optional)|	Generated by: Merchant	Use your own ID structure|
/// |UUID	|Generated by: Uber. Unique order identifier created for every delivery.|	When you need a single unique identifer|
/// |Abbreviated UUID|	Generated by: Uber. Last 5 digits of the UUID.|	Shown as “Order ID” in Courier app|
/// |Tokens	|Generated by: Uber. We use three order tokens that track an order through 3 potential stages: Quote, delivery, and return. A token prefixed with dqt_ indicates the order is in the quote stage (A quote has been generated, but a delivery has not been created). del_ indicates the delivery stage (The delivery is in-progress), and ret_ indicates the order is in the return stage (The items are being returned to the merchant). You can track an order through its stages by referencing the same token, but prepending with the token identifier (e.g. dqt_XXXXXXXX becomes del_XXXXXXXX). These tokens are unique.	| |
/// 
/// Response Body Parameters - CourierInfo
/// 
/// |Name|	Type	|Description|
/// | :--- | :--- | :--- |
/// |name|	string	|Courier’s first name and last initial.|
/// |rating|	float|	[DEPRECATED] Courier’s rating on a scale of 1.0 to 5.0.|
/// |vehicle_type|	string	|The type of vehicle the courier is using. Currently support bicycle, car, van, truck, scooter, motorcycle, and walker.|
/// |phone_number|	string	|The courier’s phone number. This is a masked phone number that can only receive calls or SMS from the dropoff phone number.|
/// |location|	LatLng|	A latitude and longitude indicating courier’s location.|
/// |img_href|	string|	A URL to courier’s profile image.|
/// 
/// ### CourierInfo - LatLng
/// 
/// |Name|	Type	|Description|
/// | :--- | :--- | :--- |
/// |lat	|double|	Latitude.|
/// |lng	|double|	Longitude.|
/// 
/// # Response Body Parameters - WaypointInfo
/// 
/// |Name|	Type	|Description|
/// | :--- | :--- | :--- |
/// |name|	string	|Display name of the person/merchant at the waypoint.|
/// |phone_number|	string|	The masked phone number of the waypoint.|
/// |address|	string|	The address of the waypoint.|
/// |detailed_address|	Address	|Structured address of the waypoint.|
/// |notes	|string|	Additional instructions at the waypoint location.|
/// |seller_notes	|string|	Delivery instructions provided by the seller for the courier at the waypoint location.|
/// |courier_notes	|string|	When a picture is requested as proof-of-delivery, this field contains the notes provided by the courier (e.g. where the items were left).|
/// |location|	LatLng|	Geographic location (Latitude, Longitude) associated with the waypoint.|
/// |verification	|VerificationProof|	Details about different verifications that have/will occur at this waypoint and any associated proof.|
/// |verification_requirements|	VerificationRequirement	|Details about the verification steps that have/must be taken at this waypoint.|
/// |external_store_id	|string|	Unique identifier used by our Partners to reference a Store or Location|
/// 
/// ### WaypointInfo - Address
/// 
/// |Name	|Type|	Description|
/// | :--- | :--- | :--- |
/// |street_address_1|	string|	|
/// |street_address_2|	string|	|
/// |city	|string|	|
/// |state	|string|	|
/// |zip_code|	string|	|
/// |country|	string|	|
/// |sublocality_level_1	|string|	|
/// 
/// ### WaypointInfo - LatLng
/// 
/// |Name|Type	|Description|
/// | :--- | :--- | :--- |
/// |lat|	double	|Latitude.|
/// |lng	|double	|Longitude. |
/// 
/// ### WaypointInfo - VerificationProof
/// 
/// |Name|	Type	|Description|
/// | :--- | :--- | :--- |
/// |signature|	SignatureProof|	Signature information captured.|
/// |barcodes|	BarcodeRequirement[]|	Barcode values/types that were scanned.|
/// |picture|	PictureProof	|Picture captured at the waypoint.|
/// |identification|	IdentificationProof|	Identification information or scanning information captured.|
/// |pin_code	|PincodeProof|	Pin entry data available after delivery completes.///  |
/// 
/// ### VerificationProof - SignatureProof
/// 
/// |Name	|Type|	Description|
/// | :--- | :--- | :--- |
/// |image_url	|string	|The url of the signature image.|
/// |signer_name|	string|	The name of the person who signed for the package.|
/// |signer_relationship	|string|	The relationship of the person who signed for the package to the intended recipient.|
/// 
/// ### VerificationProof - BarcodeRequirement
/// 
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |value	|string	|String value encoded in the barcode.|
/// |type	|string	|Type of barcode. Valid values: “CODE39”, “CODE39_FULL_ASCII”, “CODE128”, “QR”.|
/// 
/// ### VerificationProof - PictureProof
/// 
/// |Name|	Type|	Description|
/// | :--- | :--- | :--- |
/// |image_url|	string|	The url of the image taken at the waypoint.|
/// 
/// ### VerificationProof - IdentificationProof
/// 
/// |Name|	Type	|Description|
/// | :--- | :--- | :--- |
/// |min_age_verified|	boolean|	Flag if ID was successfully verified/scanned.|
/// 
/// ### VerificationProof - PincodeProof
/// 
/// |Name|	Type	|Description|
/// | :--- | :--- | :--- |
/// |entered	|string	|Value entered during pin verification.|
/// 
/// # WaypointInfo - VerificationRequirement
/// 
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |signature|	boolean|	[DEPRECATED] Flag for if a signature is required at this waypoint. signature_requirement should be used instead.|
/// |signature_requirement	|SignatureRequirement[]	|Signature requirement spec to indicate that a signature must be collected at this waypoint.|
/// |barcodes	|BarcodeRequirement[]|	Barcode values/types that must be scanned at the waypoint. Number of elements in the array is equal to the number of barcodes that must be scanned.|
/// |pincode	|PincodeRequirement|	Pincode requirement spec to indicate a delivery requires pincode confirmation upon delivery.|
/// |package|	PackageRequirement	|Package verifications required for this waypoint.|
/// |identification	|IdentificationRequirement	|Identification scanning/verification requirements for this waypoint…|
/// |picture|	boolean|	|
/// 
/// ### VerificationRequirement - SignatureRequirement
/// 
/// |Name	Type|	Description|
/// | :--- | :--- | :--- |
/// |enabled	|boolean|	Flag for if a signature is required at this waypoint.|
/// |collect_signer_name	|boolean|	Flag for if the signer’s name is required at this waypoint.|
/// |collect_signer_relationship	|boolean|	Flag for if the signer’s relationship to the intended recipient is required at this waypoint.|
/// 
/// ### VerificationRequirement - BarcodeRequirement
/// 
/// |Name|	Type|	Description|
/// | :--- | :--- | :--- |
/// |value	|string	|String value encoded in the barcode.|
/// |type	|string|	Type of barcode. Valid values: “CODE39”, “CODE39_FULL_ASCII”, “CODE128”, “QR”.|
/// 
/// ### VerificationRequirement - PincodeRequirement
/// 
/// |Name	|Type|	Description|
/// | :--- | :--- | :--- |
/// |enabled|bool|	When set to true in POST requests, the delivery will require pincode entry at handoff.|
/// |value	|string	|The pincode that the customer must present at dropoff. This is a read-only field available in GET requests that will contain the internally generated pincode.|
/// 
/// ### VerificationRequirement - PackageRequirement
/// 
/// |Name|	Type	|Description|
/// | :--- | :--- | :--- |
/// |bag_count|	integer	|Number of bags to be picked up.|
/// |drink_count|	integer|	Number of drinks to be picked up.|
/// 
/// ### VerificationRequirement - IdentificationRequirement
/// 
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |min_age	|integer	|Minimum age that must be verified for this delivery.|
/// 
/// # Response Body Parameters - ManifestInfo
/// 
/// |Name|	Type|	Description|
/// | :--- | :--- | :--- |
/// |reference	|string|	Reference that identifies the manifest.|
/// |description|	string	|[DEPRECATED] A detailed description of what the courier will be delivering. It is better to consume the description of each item in `ManifestItem.name`.|
/// |total_value	|integer|	Value of the items in the delivery, in local currency (as defined by the pickup location). e.g. $10.99 => 1099 for items in the US.|
/// 
/// # Response Body Parameters - ManifestItem
/// 
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |small	|string|	You can carry it with one hand e.g. bottle of water.|
/// |medium	|string	|You need a tote bag to carry it e.g. retail bag.|
/// |large	|string|	You need two hands to carry it e.g. computer monitor.|
/// |xlarge	|string	|You will need to make multiple trips to/from a vehicle to transport e.g. grocery order. Specifying `xlarge` will cause dispatch to only couriers using a car or larger (no walkers/bikes/scooters/etc…).|
/// |big|	string|	[DEPRECATED] Same as large.|
/// 
/// # Response Body Parameters - RelatedDelivery
/// 
/// |Name	|Type	|Description|
/// | :--- | :--- | :--- |
/// |id	|string	|Unique identifier for the delivery.|
/// |relationship	|string|	Indicating the nature of the delivery identified in related_deliveries. “original” for the forward leg of the trip, and “returned” for the return leg of the trip.|
/// 
///  # Endpoint Specific Errors
/// |Http Status Code|	Code|	Message|
/// | :--- | :--- | :--- |
/// |409|	duplicate_delivery|	An active delivery like this already exists. A pointer to the other delivery is provided.|
/// |400|	invalid_params	|The parameters of your request were invalid.|
/// |400|	unknown_location	|The specified location was not understood.|
/// |400|	address_undeliverable	|The specified location is not in a deliverable area.|
/// |400|	expired_quote	|The price quote specified has expired.|
/// |400|	used_quote|	TThe price quote specified has expired.|
/// |400|	mismatched_price_quote	|The price quote specified doesn’t match the delivery.|
/// |400|	missing_payment|	Your account’s payment information has not been provided.|
/// |400|	pickup_ready_time_not_specified	|Pickup ready time must be specified when passing in pickup/dropoff windows.|
/// |400|	pickup_window_too_small|	The pickup window needs to be at least 10 minutes long.|
/// |400|	dropoff_deadline_too_early	|The dropoff deadline needs to be at least 20 minutes after the dropoff ready time.|
/// |400|	dropoff_deadline_before_pickup_deadline|	The dropoff deadline needs to be after the pickup deadline.|
/// |400|	dropoff_ready_after_pickup_deadline	|The dropoff ready time needs to be at or before the pickup deadline.|
/// |400|	pickup_ready_too_early|	The pickup ready time cannot be in the past.|
/// |400|   pickup_deadline_too_early	|The pickup deadline time needs to be at least 20 minutes from now.|
/// |400|	pickup_ready_too_late|	The pickup ready time needs to be within the next 30 days.|
/// |402|	customer_suspended	|Your account is passed due. Payment is required.|
/// |403|	customer_blocked	|Your account is not allowed to create deliveries.|
/// |422|	address_undeliverable_limited_couriers	|The specified location is not in a deliverable area at this time because all couriers are currently busy.|
/// |429|	customer_limited	|Your account’s limits have been exceeded.|
/// |500|	unknown_error|	An unknown error happened.|
/// 
pub async fn create_delivery <T: Into<CreateDeliveryRequest>>(
    access_token: &str,
    customer_id: &str,
    create_delivery_request: T,
) -> Result<CreateDeliveryResponse, UberError> {

    let client = Client::new();
    let url = format!(
        "https://api.uber.com/v1/customers/{}/deliveries",
        customer_id
    );
    let content_type = HeaderValue::from_str("application/json")?;
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header)?;
    let create_delivery_request = create_delivery_request.into();
    let body = serde_json::to_string(&create_delivery_request)?;
    
    let res = client.post(&url)
        .header(CONTENT_TYPE, content_type)
        .header(AUTHORIZATION, authorization)
        .body(body)
        .send()
        .await?;

    let response_body = res.text().await?;
    let response_data: CreateDeliveryResponse = serde_json::from_str(&response_body)?;
    
    Ok(response_data)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 4. Get Delivery GET https://api.uber.com/v1/customers/{customer_id}/deliveries/{delivery_id}
////////////////////////////////////////////////////////////////////////////////////////////////

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
) -> Result<GetDeliveryResponse, UberError> {

    let client = Client::new();
    let url = format!(
        "https://api.uber.com/v1/customers/{}/deliveries/{}",
        customer_id,
        delivery_id
    );
    let content_type = HeaderValue::from_str("application/json")?;
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header)?;

    let res = client.get(&url)
        .header(CONTENT_TYPE, content_type)
        .header(AUTHORIZATION, authorization)
        .send()
        .await?;

    let response_body = res.text().await?;
    let response_data: GetDeliveryResponse = serde_json::from_str(&response_body)?;

    Ok(response_data)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 5. Update Delivery POST https://api.uber.com/v1/customers/{customer_id}/deliveries/{delivery_id}
////////////////////////////////////////////////////////////////////////////////////////////////

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
    update_delivery_request: UpdateDeliveryRequest,
) -> Result<UpdateDeliveryResponse, UberError> {

    let client = Client::new();
    let url = format!(
        "https://api.uber.com/v1/customers/{}/deliveries/{}",
        customer_id,
        delivery_id
    );
    let content_type = HeaderValue::from_str("application/json")?;
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header)?;
    let body = serde_json::to_string(&update_delivery_request)?;

    let res = client.post(&url)
        .header(CONTENT_TYPE, content_type)
        .header(AUTHORIZATION, authorization)
        .body(body)
        .send()
        .await?;

    let response_body = res.text().await?;
    let response_data: UpdateDeliveryResponse = serde_json::from_str(&response_body)?;

    Ok(response_data)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 6. Cancel Delivery POST https://api.uber.com/v1/customers/{customer_id}/deliveries/{delivery_id}/cancel
////////////////////////////////////////////////////////////////////////////////////////////////

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
) -> Result<CancelDeliveryResponse, UberError> {

    let client = Client::new();
    let url = format!(
        "https://api.uber.com/v1/customers/{}/deliveries/{}/cancel",
        customer_id,
        delivery_id
    );
    let content_type = HeaderValue::from_str("application/json")?;
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header)?;

    let res = client.post(&url)
        .header(CONTENT_TYPE, content_type)
        .header(AUTHORIZATION, authorization)
        .send()
        .await?;

    let response_body = res.text().await?;
    let response_data: CancelDeliveryResponse = serde_json::from_str(&response_body)?;

    Ok(response_data)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 7. List Deliveries GET https://api.uber.com/v1/customers/{customer_id}/deliveries
////////////////////////////////////////////////////////////////////////////////////////////////

/// # Response Body Parameters
/// 
/// |Name|	Type|	Description|
/// | :--- | :--- | :--- |
/// |data|	Delivery[]	|Array of deliveries matching filters (if any) provided.|
/// |next_href	|string	|Url to fetch next set of deliveries.|
/// |object	|string	|Response type. Will always be “list”.|
/// |total_count	|integer	|[DEPRECATED] Response is always -1.|
/// |url	|string|	Url for request.|
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
    limit: Option<u32>,
    offset: Option<u32>,
) -> Result<ListDeliveriesResponse, UberError> {
    let client = Client::new();
    let mut url = format!(
        "https://api.uber.com/v1/customers/{}/deliveries",
        customer_id
    );
    if let Some(filter) = filter {
        url = format!("{}?filter={}", url, filter);
    }
    if let Some(limit) = limit {
        url = format!("{}&limit={}", url, limit);
    }
    if let Some(offset) = offset {
        url = format!("{}&offset={}", url, offset);
    }

    let content_type = HeaderValue::from_str("application/json")?;
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header)?;

    let res = client.get(&url)
        .header(CONTENT_TYPE, content_type)
        .header(AUTHORIZATION, authorization)
        .send()
        .await?;

    let response_body = res.text().await?;
    let response_data: ListDeliveriesResponse = serde_json::from_str(&response_body)?;

    Ok(response_data)
}

////////////////////////////////////////////////////////////////////////////////////////////////
// 8. POD Retrieval POST https://api.uber.com/v1/customers/{customer_id}/deliveries/{delivery_id}/proof-of-delivery
////////////////////////////////////////////////////////////////////////////////////////////////

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
    pod_retrieval_request: PODRetrievalRequest,
) -> Result<PODRetrievalResponse, UberError> {

    let client = Client::new();
    let url = format!(
        "https://api.uber.com/v1/customers/{}/deliveries/{}/proof-of-delivery",
        customer_id,
        delivery_id
    );
    let content_type = HeaderValue::from_str("application/json")?;
    let auth_header = format!("Bearer {}", access_token);
    let authorization = HeaderValue::from_str(&auth_header)?;
    let body = serde_json::to_string(&pod_retrieval_request)?;

    let res = client.post(&url)
        .header(CONTENT_TYPE, content_type)
        .header(AUTHORIZATION, authorization)
        .body(body)
        .send()
        .await?;

    let response_body = res.text().await?;
    let response_data: PODRetrievalResponse = serde_json::from_str(&response_body)?;

    Ok(response_data)
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