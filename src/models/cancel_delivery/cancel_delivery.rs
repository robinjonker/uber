use serde::{Deserialize};

use crate::models::general::{
    CourierInfo,
    LocalDateTime,
    WaypointInfo,
    ManifestInfo,
    ManifestItem,
    RelatedDelivery
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CancelDeliveryResponse {
    pub complete: bool,
    pub courier: CourierInfo,
    pub courier_imminent: bool,
    pub created: LocalDateTime,
    pub currency: String,
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
    pub manifest_items:	Vec<ManifestItem>,
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