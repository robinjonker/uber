use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PODRetrievalResponse {
    pub document: Option<String>,
}
/// # Request Body Parameters - 
///
/// |Name	|Type|	Description|
/// | :--- | :--- | :--- |
/// |waypoint|	string|	Waypoint can be “pickup” or “dropoff” or “return”.|
/// |type|string|	Type can be “picture” or “signature” or “pincode”.|
///
#[derive(Serialize, Debug)]
pub struct PODRetrievalRequest {
    pub waypoint: String,
    pub type_of_waypoint: String,
}