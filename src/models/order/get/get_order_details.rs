use serde::{Serialize, Deserialize};

/// # Request Parameters
///
/// up-to-date documentation can be found here -> https://developer.uber.com/docs/eats/references/api/v2/get-eats-order-orderid
/// endpoint -> https://api.uber.com/v2/eats/order/{order_id}
///
#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct GetOrderDetailsResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub order_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store: Option<Store>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eater: Option<Eater>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eaters: Option<Vec<Eater>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cart: Option<Cart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<Payment>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub packaging: Option<Packaging>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub placed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_ready_for_pickup_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deliveries: Option<Vec<EatsDelivery>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_manager_client_id: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Store {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrator_store_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integrator_brand_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_store_id: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Eater {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery: Option<Delivery>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Delivery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub delivery_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Location {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub location_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_place_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct EatsDelivery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vehicle: Option<Vehicle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub estimated_pickup_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_state: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_code: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Vehicle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_plate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_autonomous: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub handoff_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passcode: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Payment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges: Option<Charges>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting: Option<Accounting>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<Promotions>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Charges {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_total: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_fee: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_fee_tax: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bag_fee: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_promo_applied: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_total_promo_applied: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_promo_applied: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pick_and_pack_fee: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_fee: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_fee_tax: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_order_fee: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_order_fee_tax: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_amount_due: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marketplace_fee_due_to_uber: Option<Money>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Money {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted_amount: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Accounting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_remittance: Option<TaxRemittance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_reporting: Option<TaxReporting>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TaxRemittance {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<RemittanceInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_fee_tax: Option<RemittanceInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_fee_tax: Option<RemittanceInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_order_fee_tax: Option<RemittanceInfo>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct RemittanceInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uber: Option<Vec<PayeeDetail>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restaurant: Option<Vec<PayeeDetail>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub courier: Option<Vec<PayeeDetail>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eater: Option<Vec<PayeeDetail>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct PayeeDetail {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Money>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TaxReporting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<TaxBreakdown>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<TaxLocation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<TaxLocation>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TaxBreakdown {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<TaxInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<Vec<TaxInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<Vec<TaxInfo>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TaxInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub tax_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_tax: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<Tax>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Tax {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_inclusive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jurisdiction: Option<Jurisdiction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imposition: Option<Imposition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_remittance: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Jurisdiction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Imposition {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TaxLocation {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_iso2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Promotions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotions: Option<Vec<Promotion>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Promotion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_promotion_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_discount_value: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_discount_percentage: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_delivery_fee_value: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_items: Option<Vec<DiscountItem>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DiscountItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounted_quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_amount_applied: Option<u32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Cart {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<Item>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_issues: Option<FulfillmentIssue>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Item {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<ItemPrice>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_modifier_groups: Option<Vec<ModifierGroup>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_requests: Option<Vec<SpecialRequests>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_quantity: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub special_instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_action: Option<FulfillmentAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eater_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_info: Option<TaxI>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct TaxI {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub labels: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ItemPrice {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_price: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_price: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_unit_price: Option<Money>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_total_price: Option<Money>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ModifierGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_items: Option<Vec<Item>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed_items: Option<Vec<Item>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct SpecialRequests {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allergy: Option<Allergy>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Allergy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allergens_to_exclude: Option<Vec<Allergen>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allergy_instructions: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Allergen {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "type")]
    pub allergen_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freeform_text: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct FulfillmentAction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_action_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_substitutes: Option<Vec<Item>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct FulfillmentIssue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_issue_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fulfillment_action_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_item: Option<Item>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_availability_info: Option<ItemAvailabilityInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_substitute: Option<Item>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct ItemAvailabilityInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_requested: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items_available: Option<u32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct Packaging {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposable_items: Option<DisposableItems>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "snake_case")]
pub struct DisposableItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub should_include: Option<bool>,
}
