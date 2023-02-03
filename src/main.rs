use dotenv::dotenv;
use uber::{ManifestItem, TestSpecifications, RoboCourierSpecification, create_delivery, auth};
use std::env;

fn main() {
    dotenv().ok();
    let secret_code = env::var("customer_id").expect("SECRET_CODE must be set");
    println!("The secret code is: {}", secret_code);

    let mut access_key = auth(
        env::var("client_id"),
        env::var("client_secret"),
        None, 
        None,
    );
    
    // &impl Future<Output = Result<String, reqwest::error::Error>>`

    println!("{:#?}", access_key);

    let customer_id = env::var("customer_id")?;

    let dropoff_address = "123 Main St, San Francisco, CA, 94103";
    let dropoff_name = "Dropoff Location";
    let dropoff_phone_number = "+1-555-555-5555";
    let manifest = "Delivery items";

    let mut manifest_items = ManifestItem {
        name: "Robin".to_owned(),
        quantity: 1,
        preparation_time: 10,
        size: None,
        dimensions: None,
        price: None,
        must_be_upright: None,
        weight: None,
        perishability: None,
    };

    let pickup_address = "456 Market St, San Francisco, CA, 94103";
    let pickup_name = "Pickup Location";
    let pickup_phone_number = "+1-555-555-5555";
    let test_specifications = TestSpecifications {
        robo_courier_specification: RoboCourierSpecification {
            mode: "auto".to_owned()
        }
    };

    let response = create_delivery(
        &access_key,
        &customer_id,
        &dropoff_address,
        &dropoff_name,
        &dropoff_phone_number,
        &manifest,
        manifest_items,
        &pickup_address,
        &pickup_name,
        &pickup_phone_number,
        None, 
        None, 
        None, 
        None,
        None, 
        None, 
        None, 
        None, 
        None, 
        None, 
        None, 
        None, 
        None, 
        None, 
        None,
        None, 
        None, 
        None, 
        None, 
        None,
        None, 
        None, 
        None, 
        None, 
        None, 
        None, 
        Some(test_specifications),
    );
}
