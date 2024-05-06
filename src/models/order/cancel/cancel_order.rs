use serde::{Serialize, Deserialize};

/// # Request Parameters
///
/// up-to-date documentation can be found here -> https://developer.uber.com/docs/eats/references/api/v1/post-eats-order-orderid-cancel
/// endpoint -> https://api.uber.com/v1/eats/orders/{order_id}/cancel
///
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CancelOrderRequest {
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    pub cancelling_party: String,
}