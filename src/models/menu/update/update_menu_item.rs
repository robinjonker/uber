use serde::{Serialize, Deserialize};

use crate::models::menu::{BeverageInfo, Classifications, MedicationInfo, NutritionalInfo, PhysicalPropertiesInfo, PriceRules, ProductInfo, SellingInfo, SuspensionRules};

/// # Request Parameters
///
/// up-to-date documentation can be found here -> https://developer.uber.com/docs/eats/references/api/v2/post-eats-stores-storeid-menus-items-itemid
/// endpoint -> https://api.uber.com/v2/eats/stores/{store_id}/menus/items/{item_id}
///
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct UpdateItemConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_info: Option<PriceRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspension_info: Option<SuspensionRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_info: Option<ProductInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifications: Option<Classifications>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beverage_info: Option<BeverageInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_properties_info: Option<PhysicalPropertiesInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_info: Option<MedicationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nutritional_info: Option<NutritionalInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selling_info: Option<SellingInfo>,
}
