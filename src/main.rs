use uber_api::{create_delivery, auth};

use uber_api::models::general::{ManifestItem, RoboCourierSpecification, TestSpecifications};

use uber_api::models::create_delivery::{CreateDeliveryRequest};

use clap::Parser;

#[derive(clap::StructOpt, Debug)]
#[structopt(
about = "Uber client",
name = "uber-client",
version = "0.1.0",
)]
struct CmdArgs {
    #[structopt(long)]
    client_id: String,
    #[structopt(long)]
    client_secret: String,
    #[structopt(long)]
    customer_id: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let CmdArgs {
        client_id, client_secret, customer_id,
    } = CmdArgs::parse();

    println!(" client id => {}\n client_secret => {}\n customer_id => {}", client_id, client_secret, customer_id);
    let mut access_key = auth(
        &client_id,
        &client_secret,

        Some("client_credentials".into()), Some("eats.deliveries".into())
    ).await?;

    // &impl Future<Output = Result<String, reqwest::error::Error>>`
    // {}     Display
    // {:?}   Debug
    // {:#?}  Debug formatted
    println!("Access Key: => '{:#?}'", access_key);

    let access_token = access_key.access_token;

    let dropoff_address = "123 Main St, San Francisco, CA, 94103";
    let dropoff_name = "Dropoff Location";
    let dropoff_phone_number = "+1-555-555-5555";
    let manifest = "Delivery items";

    let mut manifest_items = ManifestItem::new("Robin", 1, 5);

    let pickup_address = "456 Market St, San Francisco, CA, 94103";
    let pickup_name = "Pickup Location";
    let pickup_phone_number = "+1-555-555-5555";
    let test_specifications = TestSpecifications {
        robo_courier_specification: RoboCourierSpecification {
            mode: "auto".to_owned()
        }
    };

    // let mut request = CreateDeliveryRequest::new_with_test(dropoff_address, dropoff_name, dropoff_phone_number, manifest, manifest_items, pickup_address, pickup_name, pickup_phone_number, test_specifications);

    let response = create_delivery(
        &access_token,
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
    ).await?;

    println!(" Response => {:#?}", response);
    Ok(())
}


