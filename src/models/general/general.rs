use serde::{Serialize, Deserialize, Serializer, Deserializer};
use chrono::{DateTime, Local, TimeZone, NaiveDateTime};
use std::fmt;
use serde::de::Error as OtherError;

pub struct LocalDateTime(DateTime<Local>);

impl Serialize for LocalDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.collect_str(&format!("{}", self.0.format("%YYYY-%mm-%ddT%H:%M:%SZ"))) // 2019-10-12T07:20:50.52Z
    }
}// 2023-02-09T09:29:27Z

impl<'de> Deserialize<'de> for LocalDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = DateTime::parse_from_rfc3339(&s).unwrap();
        // let dt = Local.datetime_from_str(&s, "%YYYY-%mm-%ddT%H:%M:%SZ").map_err(D::Error::custom)?;
        Ok(LocalDateTime(dt.into()))
    }
}

impl fmt::Debug for LocalDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LocalDateTime({:?})", self.0)
    }
}


impl From<DateTime<Local>> for LocalDateTime {
    fn from(value: DateTime<Local>) -> Self {
        Self(value)
    }
}

impl From<LocalDateTime> for DateTime<Local> {
    fn from(value: LocalDateTime) -> Self {
        value.0
    }
}

#[derive(Deserialize, Debug)]
pub struct CourierInfo {
    pub name: Option<String>,
    pub rating:	Option<f64>,	
    pub vehicle_type: Option<String>,
    pub phone_number: Option<String>,
    pub location: Option<LatLng>,
    pub img_href: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct LatLng {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Deserialize, Debug)]
pub struct RelatedDelivery {
    pub id: String,
    pub relationship: String,
}

#[derive(Deserialize, Debug)]
pub struct ManifestInfo {
    pub reference: Option<String>,
    pub description: String,
    pub total_value: u32,
}

#[derive(Deserialize, Debug)]
pub struct WaypointInfo {
    pub name: String,
    pub phone_number: String,
    pub address: String,
    pub detailed_address: StructuredAddress,
    pub notes: Option<String>,
    pub seller_notes: Option<String>,
    pub courier_notes: Option<String>,
    pub location: LatLng,	
    pub verification: Option<VerificationProof>,
    pub verification_requirements: Option<VerificationRequirement>,
    pub external_store_id: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default)]
pub struct StructuredAddress { 
    pub street_address_1: String,
    pub street_address_2: Option<String>,
    pub city: String,
    pub state: String,
    pub zip_code: String,
    pub country: Option<String>,
    pub sublocality_level_1: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct VerificationProof {
    pub signature: SignatureProof,
    pub barcodes: Vec<BarcodeRequirement>,
    pub picture: PictureProof,
    pub identification: IdentificationProof,
    pub pin_code: PincodeProof,
}

#[derive(Deserialize, Debug)]
pub struct SignatureProof {
    pub image_url: String,
    pub signer_name: String,
    pub signer_relationship: String,
}

#[derive(Deserialize, Debug)]
pub struct PictureProof {
    pub image_url: String,
}

#[derive(Deserialize, Debug)]
pub struct IdentificationProof {
    pub min_age_verified: bool,
}

#[derive(Deserialize, Debug)]
pub struct PincodeProof {
    pub entered: String,
}

#[derive(Serialize, Default, Debug, Deserialize)]
pub struct ManifestItem {
    pub name: String,
    pub quantity: u32,
    pub size: String,
    pub dimensions: Option<Dimensions>,
    pub price: Option<u32>,
    pub must_be_upright: Option<bool>,
    pub weight: Option<u32>,
    pub perishability: Option<u32>,
    pub preparation_time: Option<u32>,
}
impl ManifestItem {
    pub fn new<T: Into<String>>(name: T, quantity: u32, size: T) -> Self {
        ManifestItem {
            name: name.into(),
            quantity,
            size: size.into(),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DeliverableAction {
    pub deliverable_action_meet_at_door: String,
    pub deliverable_action_leave_at_door: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct VerificationRequirement {
    pub signature: Option<bool>,
    pub signature_requirement: Vec<SignatureRequirement>,
    pub barcodes: Vec<BarcodeRequirement>,
    pub pincode: Option<PincodeRequirement>,
    pub package: Option<PackageRequirement>,
    pub identification: Option<IdentificationRequirement>,
    pub picture: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct UndeliverableAction {
    pub leave_at_door: String,
    pub return_order: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Dimensions {
    pub length: Option<u32>,
    pub height: Option<u32>,
    pub depth: Option<u32>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct SignatureRequirement {
    pub enabled: bool,
    pub collect_signer_name: bool,
    pub collect_signer_relationship: bool,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct BarcodeRequirement {
    pub value: String,
    pub type_of_barcode: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct PincodeRequirement {
    pub enabled: bool,
    pub value: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct PackageRequirement {
    pub bag_count: u32,
    pub drink_count: u32,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct IdentificationRequirement {
    pub min_age: u32,
}

#[derive(Serialize, Debug, Clone)]
pub struct TestSpecifications {
    pub robo_courier_specification: RoboCourierSpecification,
}

#[derive(Serialize, Debug, Clone)]
pub struct RoboCourierSpecification {
    pub mode: String,
}