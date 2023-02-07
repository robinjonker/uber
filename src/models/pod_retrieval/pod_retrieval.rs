use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PODRetrievalResponse {
    pub document: String,
}

#[derive(Serialize)]
pub struct PODRetrievalRequest {
    pub waypoint: String,
    pub type_of_waypoint: String,
}