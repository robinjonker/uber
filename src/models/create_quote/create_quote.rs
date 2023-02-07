use serde::{Deserialize, Serialize};

use crate::models::general::{
    LocalDateTime
};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub struct CreateQuoteRequest {
    pub pickup_address: String,
    pub dropoff_address: String,
    pub dropoff_latitude: Option<f64>,
    pub dropoff_longitude: Option<f64>,
    pub dropoff_phone_number: Option<String>,
    pub pickup_latitude: Option<f64>,
    pub pickup_longitude: Option<f64>,
    pub pickup_phone_number: Option<String>,
    pub pickup_ready_dt: Option<LocalDateTime>,
    pub pickup_deadline_dt: Option<LocalDateTime>,
    pub dropoff_ready_dt: Option<LocalDateTime>,
    pub dropoff_deadline_dt: Option<LocalDateTime>,
    pub manifest_total_value: Option<u32>,
    pub external_store_id: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CreateQuoteResponse {
    pub created: LocalDateTime,
    pub currency: String,
    pub currency_type: String,
    pub dropoff_deadline: LocalDateTime,
    pub dropoff_eta: LocalDateTime,
    pub duration: i64,
    pub expires: LocalDateTime,
    pub fee: i64,
    pub id: String,
    pub kind: String,
    pub pickup_duration: i64,
    pub external_store_id: String,
}