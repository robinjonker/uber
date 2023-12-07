use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// # Request Parameters
///
/// up-to-date documentation can be found here -> https://developer.uber.com/docs/eats/references/api/v2/put-eats-stores-storeid-menu
///
#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MenuConfiguration {
    menus: Vec<Menu>,
    categories: Vec<Category>,
    items: Vec<Item>,
    modifier_groups: Vec<ModifierGroup>,
    #[serde(skip_serializing_if = "Option::is_none")]
    menu_type: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Menu {
    id: String,
    title: MultiLanguageText,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<MultiLanguageText>,
    service_availability: Vec<ServiceAvailability>,
    category_ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MultiLanguageText {
    translations: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ServiceAvailability {
    day_of_week: String,
    time_periods: Vec<TimePeriod>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct TimePeriod {
    start_time: String,
    end_time: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Category {
    id: String,
    title: MultiLanguageText,
    #[serde(skip_serializing_if = "Option::is_none")]
    subtitle: Option<MultiLanguageText>,
    entities: Vec<MenuEntity>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MenuEntity {
    id: String,
    #[serde(rename = "type")]
    entity_type: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Item {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_data: Option<String>,
    title: MultiLanguageText,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<MultiLanguageText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    image_url: Option<String>,
    price_info: PriceRules,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity_info: Option<QuantityConstraintRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suspension_info: Option<SuspensionRules>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modifier_group_ids: Option<ModifierGroupsRules>,
    tax_info: TaxInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    nutritional_info: Option<NutritionalInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dish_info: Option<DishInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visibility_info: Option<VisibilityInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_label_info: Option<TaxLabelsRuleSet>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_info: Option<ProductInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    bundled_items: Option<Vec<BundledItems>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    beverage_info: Option<BeverageInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    physical_properities_info: Option<PhysicalPropertiesInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    medication_info: Option<PhysicalPropertiesInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    selling_info: Option<SellingInfo>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ModifierGroup {
    id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    external_data: Option<String>,
    title: MultiLanguageText,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity_info: Option<QuantityConstraintRules>, // Assuming a String type for quantity_info
    modifier_options: Vec<MenuEntity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    display_type: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PriceRules {
    price: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_price: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    container_deposit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<Vec<PriceOverride>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priced_by_unit: Option<MeasurementUnit>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PriceOverride {
    context_type: String,
    context_value: String,
    price: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    core_price: Option<u32>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MeasurementUnit {
    measurement_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    length_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    weight_unit: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    volume_unit: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct DiscountRule {
    amount: f32,
    #[serde(rename = "type")]
    discount_type: String, // e.g., "flat" or "percentage"
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct QuantityConstraintRules {
    quantity: QuantityConstraint,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<Vec<QuantityConstraintOverride>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct QuantityConstraint {
    #[serde(skip_serializing_if = "Option::is_none")]
    min_permitted: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_permitted: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_min_permitted_optional: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    charge_above: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    refund_under: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    min_permitted_unique: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_permitted_unique: Option<u32>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct QuantityConstraintOverride {
    context_type: String,
    context_value: String,
    quantity: QuantityConstraint,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SuspensionRules {
    #[serde(skip_serializing_if = "Option::is_none")]
    suspension: Option<Suspension>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<Vec<SuspensionOverride>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Suspension {
    #[serde(skip_serializing_if = "Option::is_none")]
    suspend_until: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SuspensionOverride {
    context_type: String,
    context_value: String,
    suspension: Suspension,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ModifierGroupsRules {
    ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    overrides: Option<Vec<ModifierGroupsOverride>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ModifierGroupsOverride {
    context_type: String,
    context_value: String,
    ids: Vec<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct TaxInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    tax_rate: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    vat_rate_percentage: Option<f32>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct NutritionalInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    calories: Option<EnergyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kilojoules: Option<EnergyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    serving_size: Option<MeasurementInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_servings: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    number_of_servings_interval: Option<Interval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    net_quantity: Option<MeasurementInterval>,
    #[serde(skip_serializing_if = "Option::is_none")]
    calories_per_serving: Option<EnergyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    kilojoules_per_serving: Option<EnergyInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fat: Option<NutrientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    saturated_fatty_acids: Option<NutrientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    carbohydrates: Option<NutrientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sugar: Option<NutrientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    protein: Option<NutrientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    salt: Option<NutrientInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allergens: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct EnergyInfo {
    energy_interval: Interval,
    display_type: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct MeasurementInterval {
    measurement_type: String,
    weight_interval: WeightInterval,
    volume_interval: VolumeInterval,
    count_interval: CountInterval,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct WeightInterval {
    interval: Interval,
    weight: Weight,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Interval {
    lower: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    upper: Option<u32>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Weight {
    unit_type: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct VolumeInterval {
    interval: Interval,
    volume: Volume,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Volume {
    unit_type: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CountInterval {
    interval: Interval,
    count: Count,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Count {
    unit_type: String,
    custom_unit: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct NutrientInfo {
    amount: WeightInterval,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct DishInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    classifications: Option<Classifications>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct Classifications {
    #[serde(skip_serializing_if = "Option::is_none")]
    can_serve_alone: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alcoholic_items: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    dietary_label_info: Option<DietaryLabelInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instructions_for_use: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ingredients: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    additives: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    preparation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    food_business_operator: Option<FoodBusinessOperator>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_high_fat_salt_sugar: Option<bool>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct DietaryLabelInfo {
    labels: Vec<String> // VEGAN, VEGETARIAN, GLUTEN_FREE
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct FoodBusinessOperator {
    name: String,
    address: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct VisibilityInfo {
    hours: VisibilityHours,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct VisibilityHours {
    #[serde(skip_serializing_if = "Option::is_none")]
    start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end_date: Option<String>,
    hours_of_week: HoursOfWeek,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct HoursOfWeek {
    day_of_week: String,
    time_periods: Vec<TimePeriod>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct TaxLabelsRuleSet {
    default_value: TaxLabelsInfo,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct TaxLabelsInfo {
    labels: Vec<String>,
    source: String,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct ProductInfo {
    target_market: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    gtin: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    plu: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    merchant_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    product_traits: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    countries_of_origin: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BundledItems {
    item_id: String,
    core_price: u32,
    included_quantity: u32,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct BeverageInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    caffeine_amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    alcohol_by_volume: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coffee_info: Option<CoffeeInfo>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct CoffeeInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    coffee_bean_origin: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PhysicalPropertiesInfo {
    reusable_packaging: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    storage_instructions: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SellingInfo {
    selling_options: Vec<SellingOption>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SellingOption {
    #[serde(skip_serializing_if = "Option::is_none")]
    sold_by_unit: Option<MeasurementUnit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    quantity_constraints: Option<SellingQuantityConstraint>,
    #[serde(skip_serializing_if = "Option::is_none")]
    priced_by_to_sold_by_unit_conversion_info: Option<PricedByToSoldByUnitConversionInfo>
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct SellingQuantityConstraint {
    #[serde(skip_serializing_if = "Option::is_none")]
    min_permitted: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_permitted: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    increment: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    default_quantity: Option<f32>,
}

#[derive(Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub struct PricedByToSoldByUnitConversionInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    conversion_rate: Option<f32>,
}

