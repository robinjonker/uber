use serde::{Deserialize, Serialize};

use crate::models::general::{
    CourierInfo,
    LocalDateTime,
    DeliverableAction,
    WaypointInfo,
    ManifestInfo,
    ManifestItem,
    RelatedDelivery,
    VerificationRequirement
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct UpdateDeliveryResponse {
    pub complete: bool,
    pub courier: CourierInfo,
    pub courier_imminent: bool,
    pub created: LocalDateTime,
    pub currency: String,
    pub deliverable_action: DeliverableAction,
    pub dropoff: WaypointInfo,
    pub dropoff_deadline: LocalDateTime,
    pub dropoff_eta: LocalDateTime,
    pub dropoff_identifier: String,
    pub dropoff_ready: LocalDateTime,
    pub fee: u32,
    pub id: String,
    pub kind: String,
    pub live_mode: bool,
    pub manifest: ManifestInfo,
    pub manifest_items:	ManifestItem,
    pub pickup:	WaypointInfo,
    pub pickup_deadline: LocalDateTime,
    pub pickup_eta: LocalDateTime,
    pub pickup_ready: LocalDateTime,
    pub quote_id: String,
    pub related_deliveries: RelatedDelivery,
    pub status: String,
    pub tip: u32,
    pub tracking_url: String,
    pub undeliverable_action: String,
    pub undeliverable_reason: String,
    pub updated: LocalDateTime,
    pub uuid: String,
    pub return_waypoint: WaypointInfo,
}

#[derive(Serialize)]
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