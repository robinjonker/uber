use serde::{Deserialize};

use crate::models::general::{
    CourierInfo,
    LocalDateTime,
    DeliverableAction,
    WaypointInfo,
    ManifestInfo,
    ManifestItem,
    RelatedDelivery
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ListDeliveriesResponse {
    pub data: Option<Vec<Delivery>>,
    pub next_href: Option<String>,
    pub object: Option<String>,
    pub total_count: Option<u32>,
    pub url: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Delivery {
    pub complete: Option<bool>,
    pub courier: Option<CourierInfo>,
    pub courier_imminent: Option<bool>,
    pub created: Option<LocalDateTime>,
    pub currency: Option<String>,
    pub deliverable_action: Option<DeliverableAction>,
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
    pub return_waypoint: Option<WaypointInfo>,
}
