use serde::{Deserialize};
use reqwest::StatusCode;

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

pub fn convert_status_to_message_cancel(status: StatusCode) -> String {
    match status {
        StatusCode::OK => String::from("Success!"),
        StatusCode::BAD_REQUEST => String::from("Delivery cannot be cancelled."),
        StatusCode::NOT_FOUND if status.canonical_reason().unwrap_or("").contains("customer") => String::from("Customer does not exist."),
        StatusCode::NOT_FOUND => String::from("The requested delivery does not exist."),
        StatusCode::REQUEST_TIMEOUT => String::from("The request timed out."),
        StatusCode::INTERNAL_SERVER_ERROR => String::from("An unknown error happened."),
        StatusCode::SERVICE_UNAVAILABLE => String::from("Service is currently unavailable."),
        _ => String::from("Unknown status code."),
    }
}
