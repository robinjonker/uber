use serde::{Serialize, Deserialize, Serializer, Deserializer};
use chrono::{DateTime, Local};
use std::fmt;

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

#[derive(Deserialize, Debug, Serialize, Default, Clone)]
pub struct StructuredAddress { 
    pub street_address_1: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address_2: Option<String>,
    pub city: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub state: String,
    pub zip_code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sublocality_level_1: Option<String>,
}

impl fmt::Display for StructuredAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut address = self.street_address_1.clone();
        if let Some(street_address_2) = &self.street_address_2 {
            address.push_str(&format!(", {}", street_address_2));
        }
        write!(
            f,
            "{}, {}, {}, {}",
            address,
            self.city,
            self.state,
            self.zip_code
        )?;
        if let Some(sublocality_level_1) = &self.sublocality_level_1 {
            write!(f, ", {}", sublocality_level_1)?;
        }
        if let Some(country) = &self.country {
            write!(f, ", {}", country)?;
        }
        Ok(())
    }
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

#[derive(Serialize, Default, Debug, Deserialize, Clone)]
pub struct ManifestItem {
    pub name: String,
    pub quantity: u32,
    pub size: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dimensions: Option<Dimensions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub must_be_upright: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perishability: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
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

#[derive(Serialize, Debug, Deserialize)]
pub struct VerificationRequirement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<bool>,
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

#[derive(Serialize, Debug, Clone)]
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