use serde::{Deserialize, Serialize};

use crate::models::general::{
    CourierInfo,
    LocalDateTime,
    WaypointInfo,
    ManifestInfo,
    ManifestItem,
    RelatedDelivery,
    VerificationRequirement
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct UpdateDeliveryResponse {
    pub complete: Option<bool>,
    pub courier: Option<CourierInfo>,
    pub courier_imminent: Option<bool>,
    pub created: Option<LocalDateTime>,
    pub currency: Option<String>,
    pub deliverable_action: Option<String>,
    pub dropoff: Option<WaypointInfo>,
    pub dropoff_deadline: Option<LocalDateTime>,
    pub dropoff_eta: Option<LocalDateTime>,
    pub dropoff_identifier: Option<String>,
    pub dropoff_ready: Option<LocalDateTime>,
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
    pub related_deliveries: Option<RelatedDelivery>,
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
#[derive(Serialize, Default)]
pub struct UpdateDeliveryRequest {
    pub dropoff_notes: Option<String>,
    pub dropoff_seller_notes: Option<String>,
    pub dropoff_verification: Option<VerificationRequirement>,
    pub manifest_reference: Option<String>,
    pub pickup_notes: Option<String>,
    pub pickup_verification: Option<VerificationRequirement>,
    pub requires_dropoff_signature: Option<bool>,
    pub requires_id: Option<bool>,
    pub tip_by_customer: Option<u32>,
    pub dropoff_latitude: Option<f64>,
    pub dropoff_longitude: Option<f64>,
}