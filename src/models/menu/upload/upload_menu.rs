use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// # Request Parameters
///
/// up-to-date documentation can be found here -> https://developer.uber.com/docs/eats/references/api/v2/put-eats-stores-storeid-menu
/// endpoint -> https://api.uber.com/v2/eats/stores/{store_id}/menus
///
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MenuConfiguration {
    pub menus: Vec<Menu>,
    pub categories: Vec<Category>,
    pub items: Vec<Item>,
    pub modifier_groups: Vec<ModifierGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub menu_type: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Menu {
    pub id: String,
    pub title: MultiLanguageText,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<MultiLanguageText>,
    pub service_availability: Vec<ServiceAvailability>,
    pub category_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MultiLanguageText {
    pub translations: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ServiceAvailability {
    pub day_of_week: String,
    pub time_periods: Vec<TimePeriod>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct TimePeriod {
    pub start_time: String,
    pub end_time: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Category {
    pub id: String,
    pub title: MultiLanguageText,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<MultiLanguageText>,
    pub entities: Vec<MenuEntity>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MenuEntity {
    pub id: String,
    #[serde(rename = "type")]
    pub entity_type: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Item {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_data: Option<String>,
    pub title: MultiLanguageText,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<MultiLanguageText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    pub price_info: PriceRules,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_info: Option<QuantityConstraintRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspension_info: Option<SuspensionRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub modifier_group_ids: Option<ModifierGroupsRules>,
    pub tax_info: TaxInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nutritional_info: Option<NutritionalInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dish_info: Option<DishInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility_info: Option<VisibilityInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_label_info: Option<TaxLabelsRuleSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_info: Option<ProductInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundled_items: Option<Vec<BundledItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub beverage_info: Option<BeverageInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub physical_properities_info: Option<PhysicalPropertiesInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medication_info: Option<MedicationInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selling_info: Option<SellingInfo>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ModifierGroup {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_data: Option<String>,
    pub title: MultiLanguageText,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_info: Option<QuantityConstraintRules>, // Assuming a String type for quantity_info
    pub modifier_options: Vec<MenuEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_type: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PriceRules {
    pub price: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_price: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_deposit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<PriceOverride>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priced_by_unit: Option<MeasurementUnit>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PriceOverride {
    pub context_type: String,
    pub context_value: String,
    pub price: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub core_price: Option<u32>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MeasurementUnit {
    pub measurement_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub length_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub volume_unit: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct DiscountRule {
    pub amount: f32,
    #[serde(rename = "type")]
    pub discount_type: String, // e.g., "flat" or "percentage"
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct QuantityConstraintRules {
    pub quantity: QuantityConstraint,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<QuantityConstraintOverride>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct QuantityConstraint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_permitted: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_permitted: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_min_permitted_optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge_above: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_under: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_permitted_unique: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_permitted_unique: Option<u32>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct QuantityConstraintOverride {
    pub context_type: String,
    pub context_value: String,
    pub quantity: QuantityConstraint,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SuspensionRules {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspension: Option<Suspension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<SuspensionOverride>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Suspension {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suspend_until: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SuspensionOverride {
    pub context_type: String,
    pub context_value: String,
    pub suspension: Suspension,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ModifierGroupsRules {
    pub ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overrides: Option<Vec<ModifierGroupsOverride>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ModifierGroupsOverride {
    pub context_type: String,
    pub context_value: String,
    pub ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct TaxInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vat_rate_percentage: Option<f32>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct NutritionalInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calories: Option<EnergyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kilojoules: Option<EnergyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serving_size: Option<MeasurementInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_servings: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_servings_interval: Option<Interval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_quantity: Option<MeasurementInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calories_per_serving: Option<EnergyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kilojoules_per_serving: Option<EnergyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fat: Option<NutrientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub saturated_fatty_acids: Option<NutrientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carbohydrates: Option<NutrientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sugar: Option<NutrientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protein: Option<NutrientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salt: Option<NutrientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allergens: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct EnergyInfo {
    pub energy_interval: Interval,
    pub display_type: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MeasurementInterval {
    pub measurement_type: String,
    pub weight_interval: WeightInterval,
    pub volume_interval: VolumeInterval,
    pub count_interval: CountInterval,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WeightInterval {
    pub interval: Interval,
    pub weight: Weight,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Interval {
    pub lower: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upper: Option<u32>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Weight {
    pub unit_type: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct VolumeInterval {
    pub interval: Interval,
    pub volume: Volume,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Volume {
    pub unit_type: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CountInterval {
    pub interval: Interval,
    pub count: Count,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Count {
    pub unit_type: String,
    pub custom_unit: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct NutrientInfo {
    pub amount: WeightInterval,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct DishInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifications: Option<Classifications>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Classifications {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_serve_alone: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alcoholic_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dietary_label_info: Option<DietaryLabelInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions_for_use: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ingredients: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additives: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preparation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub food_business_operator: Option<FoodBusinessOperator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_high_fat_salt_sugar: Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct DietaryLabelInfo {
    pub labels: Vec<String> // VEGAN, VEGETARIAN, GLUTEN_FREE
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FoodBusinessOperator {
    pub name: String,
    pub address: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct VisibilityInfo {
    pub hours: VisibilityHours,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct VisibilityHours {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    pub hours_of_week: HoursOfWeek,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct HoursOfWeek {
    pub day_of_week: String,
    pub time_periods: Vec<TimePeriod>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct TaxLabelsRuleSet {
    pub default_value: TaxLabelsInfo,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct TaxLabelsInfo {
    pub labels: Vec<String>,
    pub source: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProductInfo {
    pub target_market: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gtin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_traits: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub countries_of_origin: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BundledItems {
    pub item_id: String,
    pub core_price: u32,
    pub included_quantity: u32,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BeverageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub caffeine_amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alcohol_by_volume: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coffee_info: Option<CoffeeInfo>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CoffeeInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coffee_bean_origin: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PhysicalPropertiesInfo {
    pub reusable_packaging: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub storage_instructions: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MedicationInfo {
    pub medical_prescription_required: Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SellingInfo {
    pub selling_options: Vec<SellingOption>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SellingOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sold_by_unit: Option<MeasurementUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity_constraints: Option<SellingQuantityConstraint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priced_by_to_sold_by_unit_conversion_info: Option<PricedByToSoldByUnitConversionInfo>
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SellingQuantityConstraint {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_permitted: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_permitted: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub increment: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_quantity: Option<f32>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PricedByToSoldByUnitConversionInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_rate: Option<f32>,
}

