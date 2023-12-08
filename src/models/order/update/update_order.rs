use serde::{Serialize, Deserialize};
use crate::models::order::FulfillmentIssue;

/// # Request Parameters
///
/// up-to-date documentation can be found here -> https://developer.uber.com/docs/eats/references/api/v2/patch-eats-orders-orderid-cart
/// endpoint -> https://api.uber.com/v2/eats/orders/{order_id}/cart
///
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct UpdateOrderRequest {
    pub fulfillment_issues: Vec<FulfillmentIssue>,
}