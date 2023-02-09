use serde::{Deserialize, Serialize};

use crate::models::general::{
    LocalDateTime,
    ManifestItem,
    DeliverableAction,
    VerificationRequirement,
    UndeliverableAction,
    TestSpecifications,
    CourierInfo,
    WaypointInfo,
    ManifestInfo,
    RelatedDelivery,
};

#[derive(Serialize, Debug, Default)]
#[serde(rename_all = "snake_case")]
pub struct CreateDeliveryRequest {
    pub dropoff_address: String,
    pub dropoff_name: String,
    pub dropoff_phone_number: String,
    pub manifest: String,
    pub manifest_items: Vec<ManifestItem>,
    pub pickup_address: String,
    pub pickup_name: String,
    pub pickup_phone_number: String,
    pub deliverable_action: Option<DeliverableAction>,
    pub dropoff_business_name: Option<String>,
    pub dropoff_latitude: Option<f64>,
    pub dropoff_longitude: Option<f64>,
    pub dropoff_notes: Option<String>,
    pub dropoff_seller_notes: Option<String>,
    pub dropoff_verification: Option<VerificationRequirement>,
    pub manifest_reference: Option<String>,
    pub manifest_total_value: Option<u32>,
    pub pickup_business_name: Option<String>,
    pub pickup_latitude: Option<f64>,
    pub pickup_longitude: Option<f64>,
    pub pickup_notes: Option<String>,
    pub pickup_verification: Option<VerificationRequirement>,
    pub quote_id: Option<String>,
    pub undeliverable_action: Option<UndeliverableAction>,
    pub pickup_ready_dt: Option<LocalDateTime>,
    pub pickup_deadline_dt: Option<LocalDateTime>,
    pub dropoff_ready_dt: Option<LocalDateTime>,
    pub dropoff_deadline_dt: Option<LocalDateTime>,
    pub requires_dropoff_signature: Option<bool>,
    pub requires_id: Option<bool>,
    pub tip: Option<u32>,
    pub idempotency_key: Option<String>,
    pub external_store_id: Option<String>,
    pub return_verification: Option<VerificationRequirement>,
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

impl CreateDeliveryRequest {
    pub fn new<T: Into<String>>(
        dropoff_address: T, 
        dropoff_name: T, 
        dropoff_phone_number: T, 
        manifest: T, 
        manifest_items: Vec<ManifestItem>, 
        pickup_address: T, 
        pickup_name: T, 
        pickup_phone_number: T) -> Self {
        CreateDeliveryRequest {
            dropoff_address: dropoff_address.into(),
            dropoff_name: dropoff_name.into(),
            dropoff_phone_number: dropoff_phone_number.into(),
            manifest: manifest.into(),
            manifest_items,
            pickup_address: pickup_address.into(),
            pickup_name: pickup_name.into(),
            pickup_phone_number: pickup_phone_number.into(),
            ..Default::default()
        }
    }
    pub fn new_with_test<T: Into<String>>(
        dropoff_address: T, 
        dropoff_name: T, 
        dropoff_phone_number: T, 
        manifest: T, 
        manifest_items: Vec<ManifestItem>, 
        pickup_address: T, 
        pickup_name: T, 
        pickup_phone_number: T,
        test_specifications: TestSpecifications) -> Self {
        CreateDeliveryRequest {
            dropoff_address: dropoff_address.into(),
            dropoff_name: dropoff_name.into(),
            dropoff_phone_number: dropoff_phone_number.into(),
            manifest: manifest.into(),
            manifest_items,
            pickup_address: pickup_address.into(),
            pickup_name: pickup_name.into(),
            pickup_phone_number: pickup_phone_number.into(),
            test_specifications: Some(test_specifications),
            ..Default::default()
        }
    }
    // pub fn for_text(prescription: &Prescription) -> Self {
    //     Self {
    //         prescription_id: prescription.id,
    //         prescription_status: prescription.status_code.clone(),
    //         content_type: CONTENT_TYPE_TEXT_PLAIN.to_string(),
    //     }
    // }
}


// #[derive(Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// pub struct PrescriptionJsonParams {
//     prescription_id: i64,
//     prescription_status: String,
//     content_type: String,
// }

// impl PrescriptionJsonParams {
//     pub fn for_text(prescription: &Prescription) -> Self {
//         Self {
//             prescription_id: prescription.id,
//             prescription_status: prescription.status_code.clone(),
//             content_type: CONTENT_TYPE_TEXT_PLAIN.to_string(),
//         }
//     }

//     pub fn for_push_notification(prescription: &Prescription) -> Self {
//         Self {
//             prescription_id: prescription.id,
//             prescription_status: prescription.status_code.clone(),
//             content_type: CONTENT_TYPE_NOTIFICATION_PUSH.to_string(),
//         }
//     }
// }