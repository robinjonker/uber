use std::os::macos::raw::stat;

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

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CancelDeliveryStatusResponse {
    pub status_code: String,
    pub message: String,
}

pub fn convert_status_to_message(status_code: String) -> &'static str {
    let message = 
        if status_code.contains("200") {
            "Delivery successfully cancelled"
        }
        else if status_code.contains("400") {
            "Delivery cannot be cancelled."
        }
        else if status_code.contains("404") {
            "The requested delivery does not exist."
        }
        else if status_code.contains("408") {
            "The request timed out."
        }
        else if status_code.contains("500") {
            "An unknown error happened."
        }
        else if status_code.contains("503") {
            "Service is currently unavailable."
        }
        else {
            "Unknown Status Code."
        };
        message
}
