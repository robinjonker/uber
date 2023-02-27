use crate::models::general::{
    LocalDateTime,
    ManifestItem,
    CourierInfo,
    ManifestInfo,
    RelatedDelivery,
    LatLng,
    StructuredAddress,
    VerificationProof
};

/// # Delivery Status Webhook Event Definitions
/// 
/// | Name|	Type|	Description|
/// | :--- | :--- | :--- |
/// | status|	String|	Status of the delivery the event refers to.|
/// | kind|	String|	The kind of the event in more detail (event.delivery_status, event.delivery_return).|
/// | created|	LocalDateTime|	LocalDateTime indicating when the event was generated.|
/// | live_mode|	bool|	A flag indicating if the event applies to a live vs a test delivery.|
/// | delivery_id|	String|	The id of the delivery the event applies to.|
/// | id|	String|	A unique id for this event instance.|
/// | data|	DeliveryData|	Information about the delivery|
/// | customer_id|	String|	Unique identifier for the customer this delivery belongs to.|
/// | developer_id|	String|	Unique identifier for the developer the above customer_id maps to.|
/// | account_id|	String|	Unique identifier for the account of the above developer that this delivery belongs to.|
/// | route_id|	String|	Unique identifier of the route. This value can be used to identify when multiple deliveries are being picked up by a single courier.|
///
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DeliveryStatus {
    pub status: String,
    pub kind: String, 
    pub created: LocalDateTime, 
    pub live_mode: bool,  
    pub delivery_id: String, 
    pub id: String,
    pub data: DeliveryData, 
    pub customer_id: String, 
    pub developer_id: String, 
    pub account_id: String, 
    pub route_id: String,
}

/// Courier Update Webhook Event Definitions
/// |Name|	Type|	Description|
/// | :--- | :--- | :--- |
/// |location|	LatLng|	A latitude and longitude indicating the courier’s location.|
/// |kind|	string|	The kind of the event in more detail (event.courier_update).|
/// |live_mode|	boolean|	A flag indicating if the event applies to a live vs a test delivery.|
/// |delivery_id|	string|	The id of the delivery the event applies to.|
/// |job_id|	string|	|
/// |data|	DeliveryData|	Information about the delivery|
/// 
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CourierUpdate {
    pub location: LatLng,
    pub kind: String,
    pub live_mode: bool,
    pub delivery_id: String,
    pub job_id: String,
    pub data: DeliveryData,
}

/// # Webhook Event Definitions - DeliveryData
/// 
///| Name|	Type	|Description|
/// | :--- | :--- | :--- |
/// |id|	String|	The id of the delivery the event applies to.|
/// |status|	String|	Status of the delivery the event refers to.|
/// |created|	LocalDateTime (RFC 3339)|	Date/Time at which the delivery was created.|
/// |updated|	LocalDateTime (RFC 3339)|	Date/Time at which the delivery was last updated.|
/// |pickup_eta|	LocalDateTime (RFC 3339)|	Estimated time the courier will arrive at the pickup location.|
/// |pickup_ready|	LocalDateTime (RFC 3339)|	When a delivery is ready to be picked up. This is the start of the pickup window.|
/// |pickup_deadline|	LocalDateTime (RFC 3339)|	When a delivery must be picked up by. This is the end of the pickup window.|
/// |dropoff_eta|	LocalDateTime (RFC 3339)|	Estimated drop-off time.|
/// |dropoff_ready|	LocalDateTime (RFC 3339)|	When a delivery is ready to be dropped off. This is the start of the dropoff window.|
/// |dropoff_deadline|	LocalDateTime (RFC 3339)|	When a delivery must be dropped off. This is the end of the dropoff window.|
/// |quote_id|	String|	ID for the Delivery Quote if one was provided when creating this Delivery.|
/// |fee|	i64|	Amount in cents that is charged for this delivery.|
/// |currency|	String|	Three-letter ISO currency code, in lowercase.|
/// |tip|	i64|	Amount in cents that will be paid to the courier as a tip.|
/// |manifest|	ManifestInfo|	A detailed description of what the courier will be delivering.|
/// |manifest_items|	ManifestItem|[]	List of items being delivered.|
/// |pickup|	DeliveryInfo|	The pickup details for the delivery.|
/// |dropoff|	DeliveryInfo|	The dropoff details for the delivery|
/// |courier|	CourierInfo|	The courier associated with this delivery.|
/// |live_mode|	bool|	Indicates if the delivery is live mode or test mode.|
/// |related_deliveries|	RelatedDelivery|[]	A collection describing other jobs that share an association.|
/// |tracking_url|	String|	This url can be used to track the courier during the delivery.|
/// |dropoff_identifier|	String|	This field identifies who received delivery at the dropoff location.|
/// |courier_imminent|	bool|	Flag indicating if the courier is close to the pickup or dropoff location|
/// |undeliverable_reason|	String|	If a delivery was undeliverable, this field will contain the reason why it was undeliverable.|
/// |undeliverable_action|	String|	If a delivery was undeliverable, this field will contain the resulting action taken by the courier.|
/// |complete|	bool|	Flag indicating if the delivery is ongoing.|
/// |kind|	String|	The type of object being described.|
/// |uuid|	String|	Alternative delivery identifier. “Id” field should be used for any identification purposes. “uuid” field is equally unique but loses contextual information (i.e. nothing in this identifier points out that it relates to a delivery). Unlike “Id” field, “uuid” is case-insensitive. Value for “uuid” field is UUID v4 with ‘-’ characters removed.|
/// |route_id|	String|	Defines an ID for a contiguous series of waypoints. Batched deliveries will have the same route.|
/// |order|	OrderInfo|	Order details for a delivery|
/// |cancelation_reason|	CancellationReason|	Info on why the delivery was cancelled|
/// |return|	DeliveryInfo|	The return details for the delivery.|
/// 
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DeliveryData {
    pub id: String,
    pub status: String,
    pub created: Option<LocalDateTime>,
    pub updated: Option<LocalDateTime>,
    pub pickup_eta: Option<LocalDateTime>,
    pub pickup_ready: Option<LocalDateTime>,
    pub pickup_deadline: Option<LocalDateTime>,
    pub dropoff_eta: Option<LocalDateTime>,
    pub dropoff_ready: Option<LocalDateTime>,
    pub dropoff_deadline: Option<LocalDateTime>,
    pub quote_id: Option<String>,
    pub fee: Option<i64>,
    pub currency: Option<String>,
    pub tip: Option<i64>,
    pub manifest: Option<ManifestInfo>,
    pub manifest_items: Option<Vec<ManifestItem>>,
    pub pickup: Option<DeliveryInfo>,
    pub dropoff: Option<DeliveryInfo>,
    pub courier: Option<CourierInfo>,
    pub live_mode: Option<bool>,
    pub related_deliveries: Option<Vec<RelatedDelivery>>,
    pub tracking_url: Option<String>,
    pub dropoff_identifier: Option<String>,
    pub courier_imminent: Option<bool>,
    pub undeliverable_reason: Option<String>,
    pub undeliverable_action: Option<String>,
    pub complete: Option<bool>,
    pub kind: Option<String>,
    pub uuid: Option<String>,
    pub route_id: Option<String>,
    pub order: Option<OrderInfo>,
    pub cancelation_reason: Option<CancellationReason>,
    #[serde(rename = "return")]
    pub return_delivery: Option<DeliveryInfo>,
}

/// # DeliveryData - DeliveryInfo|
/// 
/// |Name|	Type|	Description|
/// | :--- | :--- | :--- |
/// |name|	string|	The name of the person at the waypoint|
/// |phone_number|	string|	The phone number of the waypoint|
/// |address|	string|	The address of the waypoint|
/// |detailed_address|	StructuredAddress|	The full address of the waypoint|
/// |location|	LatLng|	Geographic location (Latitude, Longitude) associated with the waypoint|
/// |notes|	string|	Additional instructions at the waypoint location.|
/// |verification|	VerificationProof|	The details about different verification types|
/// |courier_notes|	string|	When a picture is requested as proof-of-delivery, this field contains the notes provided by the courier (e.g. where the items were left).|
/// 
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DeliveryInfo {
    pub name: Option<String>,
    pub phone_number: Option<String>,
    pub address: Option<String>,
    pub detailed_address: Option<StructuredAddress>,
    pub location: Option<LatLng>,
    pub notes: Option<String>,
    pub verification: Option<VerificationProof>,
    pub courier_notes: Option<String>,
}

/// # DeliveryData - OrderInfo
/// 
/// |Name|	Type|	Description|
/// | :--- | :--- | :--- |
/// |id|	string|	Unique identifier for order|
/// |number|	string|	A 3-digit customer and courier facing order id|
/// |display_name|	string|	Customer name includes customer’s first name and last initial|
/// 
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct OrderInfo {
    pub id: String,
    pub number: String,
    pub display_name: String,
}

/// # DeliveryData - CancellationReason
/// 
/// |Name|	Type|	Description|
/// | :--- | :--- | :--- |
/// |primary_reason|	string|	|
/// |secondary_reason|	string| |
/// 
#[derive(Deserialize, Debug)]
#[serde(rename_all = "snake_case")]
pub struct CancellationReason {
    pub primary_reason: String,
    pub secondary_reason: String
}