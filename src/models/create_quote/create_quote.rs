use serde::{Deserialize, Serialize};

use crate::models::general::{
    LocalDateTime
};

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
#[derive(Serialize, Default)]
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
impl CreateQuoteRequest {
    pub fn new<T: Into<String>>(
        pickup_address: T, 
        dropoff_address: T) -> Self {
        CreateQuoteRequest {
            pickup_address: pickup_address.into(),
            dropoff_address: dropoff_address.into(),
            ..Default::default()
        }
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CreateQuoteResponse {
    pub created: Option<LocalDateTime>,
    pub currency: Option<String>,
    pub currency_type: Option<String>,
    pub dropoff_deadline: Option<LocalDateTime>,
    pub dropoff_eta: Option<LocalDateTime>,
    pub duration: Option<i64>,
    pub expires: Option<LocalDateTime>,
    pub fee: Option<i64>,
    pub id: Option<String>,
    pub kind: Option<String>,
    pub pickup_duration: Option<i64>,
    pub external_store_id: Option<String>,
}