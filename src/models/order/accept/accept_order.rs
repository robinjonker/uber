use serde::{Serialize, Deserialize};

/// # Request Parameters
///
/// up-to-date documentation can be found here -> https://developer.uber.com/docs/eats/references/api/v1/post-eats-order-orderid-acceptposorder
/// endpoint -> https://api.uber.com/v1/eats/orders/{order_id}/accept_pos_order
///
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct AcceptOrderRequest {
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pickup_time: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields_relayed: Option<Vec<FieldsRelayed>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FieldsRelayed {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_special_instructions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_special_instructions: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_special_requests: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<bool>,
}