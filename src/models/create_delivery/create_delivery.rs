use serde::{Deserialize, Serialize};

use crate::models::general::{
    LocalDateTime,
    ManifestItem,
    VerificationRequirement,
    TestSpecifications,
    CourierInfo,
    WaypointInfo,
    ManifestInfo,
    RelatedDelivery,
    StructuredAddress
};
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
#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct CreateDeliveryRequest {
    pub dropoff_address: String, // StructuredAddress then serde_json::to_string(&dropoff_address).unwrap()
    pub dropoff_name: String,
    pub dropoff_phone_number: String,
    pub manifest_items: Vec<ManifestItem>,
    pub pickup_address: String,
    pub pickup_name: String,
    pub pickup_phone_number: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliverable_action: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropoff_business_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropoff_latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropoff_longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropoff_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropoff_seller_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropoff_verification: Option<VerificationRequirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manifest_total_value: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_business_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_verification: Option<VerificationRequirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quote_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub undeliverable_action: Option<i8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_ready_dt: Option<LocalDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_deadline_dt: Option<LocalDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropoff_ready_dt: Option<LocalDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropoff_deadline_dt: Option<LocalDateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_store_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_verification: Option<VerificationRequirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_specifications: Option<TestSpecifications>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CreateDeliveryResponse {
    pub complete: Option<bool>,
    pub courier: Option<CourierInfo>,
    pub courier_imminent: Option<bool>,
    pub created: Option<LocalDateTime>,
    pub currency: Option<String>,
    pub dropoff: Option<WaypointInfo>,
    pub dropoff_deadline: Option<LocalDateTime>,
    pub dropoff_eta: Option<LocalDateTime>,
    pub dropoff_identifier: Option<String>,
    pub dropoff_ready: Option<LocalDateTime>,
    pub external_id: Option<String>,
    pub fee: Option<u32>,
    pub id: Option<String>,
    pub kind: Option<String>,
    pub live_mode: Option<bool>,
    pub manifest: Option<ManifestInfo>,
    pub manifest_items:	Option<Vec<ManifestItem>>,
    pub pickup:	Option<WaypointInfo>,
    pub pickup_deadline: Option<LocalDateTime>,
    pub pickup_eta: Option<LocalDateTime>,
    pub pickup_ready: Option<LocalDateTime>,
    pub quote_id: Option<String>,
    pub related_deliveries: Option<Vec<RelatedDelivery>>,
    pub status: Option<String>,
    pub tip: Option<u32>,
    pub tracking_url: Option<String>,
    pub undeliverable_action: Option<String>,
    pub undeliverable_reason: Option<String>,
    pub updated: Option<LocalDateTime>,
    pub uuid: Option<String>,
    #[serde(rename = "return")]
    pub return_waypoint: Option<WaypointInfo>,
}