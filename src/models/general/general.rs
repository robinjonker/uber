use serde::{Serialize, Deserialize, Serializer, Deserializer};
use chrono::{DateTime, Local, TimeZone};
use std::fmt;
use serde::de::Error as OtherError;

pub struct LocalDateTime(DateTime<Local>);

impl Serialize for LocalDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.collect_str(&format!("{}", self.0.format("%YYYY-%mm-%ddT%H:%M:%S"))) // 2019-10-12T07:20:50.52Z
    }
}

impl<'de> Deserialize<'de> for LocalDateTime {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let dt = Local.datetime_from_str(&s, "%YYYY-%mm-%ddT%H:%M:%S").map_err(D::Error::custom)?;
        Ok(LocalDateTime(dt))
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
    pub name: String,
    pub rating:	f64,	
    pub vehicle_type: String,
    pub phone_number: String,
    pub location: LatLng,
    pub img_href: String,
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
    pub detailed_address: Address,
    pub notes: Option<String>,
    pub seller_notes: Option<String>,
    pub courier_notes: Option<String>,
    pub location: LatLng,	
    pub verification: Option<VerificationProof>,
    pub verification_requirements: Option<VerificationRequirement>,
    pub external_store_id: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Address {
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
    deliverable_action_meet_at_door: String,
    deliverable_action_leave_at_door: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct VerificationRequirement {
    signature: Option<bool>,
    signature_requirement: Option<Vec<SignatureRequirement>>,
    barcodes: Option<Vec<BarcodeRequirement>>,
    pincode: Option<PincodeRequirement>,
    package: Option<PackageRequirement>,
    identification: Option<IdentificationRequirement>,
    picture: Option<bool>,
}

#[derive(Serialize, Debug)]
pub struct UndeliverableAction {
    leave_at_door: String,
    return_order: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct Dimensions {
    length: Option<u32>,
    height: Option<u32>,
    depth: Option<u32>,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct SignatureRequirement {
    enabled: bool,
    collect_signer_name: bool,
    collect_signer_relationship: bool,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct BarcodeRequirement {
    value: String,
    type_of_barcode: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct PincodeRequirement {
    enabled: bool,
    value: String,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct PackageRequirement {
    bag_count: u32,
    drink_count: u32,
}

#[derive(Serialize, Debug, Deserialize)]
pub struct IdentificationRequirement {
    min_age: u32,
}

#[derive(Serialize, Debug, Clone)]
pub struct TestSpecifications {
    pub robo_courier_specification: RoboCourierSpecification,
}

#[derive(Serialize, Debug, Clone)]
pub struct RoboCourierSpecification {
    pub mode: String,
}