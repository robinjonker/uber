use serde::{Serialize, Deserialize, Serializer, Deserializer};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};
use std::fmt;

#[derive(Clone)]
pub struct LocalDateTime(DateTime<Local>);

impl Serialize for LocalDateTime {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.collect_str(&format!("{}", self.0.format("%Y-%m-%dT%H:%M:%SZ"))) // 2019-10-12T07:20:50.52Z
    }
}// 2023-02-09T09:29:27Z

impl ToString for LocalDateTime {
    fn to_string(&self) -> String {
        format!("{}", self.0.format("%Y-%m-%dT%H:%M:%SZ"))
    }
}

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

impl Default for LocalDateTime {
    fn default() -> Self {
        let default_datetime = NaiveDateTime::from_timestamp_opt(0, 0).unwrap_or_default();
        let local_datetime = Local.from_local_datetime(&default_datetime).unwrap();
        LocalDateTime(local_datetime)
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
    pub detailed_address: StructuredAddressResponse,
    pub notes: Option<String>,
    pub seller_notes: Option<String>,
    pub courier_notes: Option<String>,
    pub location: LatLng,	
    pub verification: Option<VerificationProof>,
    pub verification_requirements: Option<VerificationRequirement>,
    pub external_store_id: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct StructuredAddress { 
    pub street_address: Vec<String>, // vec containing street_address_1, street_address_2 (optional), sublocality_level_1 (optional)
    pub city: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub state: String,
    pub zip_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct StructuredAddressResponse { 
    pub street_address_1: Option<String>,
    pub street_address_2: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub zip_code: Option<String>,
    pub country: Option<String>,
    pub sublocality_level_1: Option<String>,
}


#[derive(Deserialize, Debug)]
pub struct VerificationProof {
    pub signature: Option<SignatureProof>,
    pub barcodes: Option<Vec<BarcodeRequirement>>,
    pub picture: Option<PictureProof>,
    pub identification: Option<IdentificationProof>,
    pub pin_code: Option<PincodeProof>,
}

#[derive(Deserialize, Debug)]
pub struct SignatureProof {
    pub image_url: Option<String>,
    pub signer_name: Option<String>,
    pub signer_relationship: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct PictureProof {
    pub image_url: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct IdentificationProof {
    pub min_age_verified: Option<bool>,
}

#[derive(Deserialize, Debug)]
pub struct PincodeProof {
    pub entered: Option<String>,
}

#[derive(Serialize, Default, Debug, Deserialize, Clone)]
pub struct ManifestItem {
    pub name: String,
    pub quantity: u32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub size: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<u32>,
    #[serde(skip_serializing_if = "should_skip_serialization_bool")]
    pub must_be_upright: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perishability: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparation_time: Option<u32>,
}

fn should_skip_serialization_bool (b: &bool) -> bool {
    !*b
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

#[derive(Serialize, Debug, Deserialize, Default)]
pub struct VerificationRequirement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_requirement: Option<SignatureRequirement>,
    #[serde(skip_serializing_if = "Vec::is_empty", default = "Vec::default", deserialize_with = "null_to_default")]
    pub barcodes: Vec<BarcodeRequirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pincode: Option<PincodeRequirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package: Option<PackageRequirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification: Option<IdentificationRequirement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<bool>,
}

#[derive(Serialize, Debug, Deserialize, Clone)]
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

#[derive(Serialize, Debug, Deserialize, Clone)]
pub struct BarcodeRequirement {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
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

#[derive(Serialize, Debug, Clone, Default)]
pub struct RoboCourierSpecification {
    pub mode: String,
}

fn null_to_default<'de, D, T>(d: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Default + Deserialize<'de>,
{
    let opt = Option::deserialize(d)?;
    let val = opt.unwrap_or_else(T::default);
    Ok(val)
}