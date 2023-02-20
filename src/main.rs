use std::default;

use uber_api::{
    create_delivery, 
    auth,
    create_quote, 
    get_delivery,
    update_delivery, 
    cancel_delivery,
    list_deliveries,
    AuthRequest, 
    CreateDeliveryRequest,
    UpdateDeliveryRequest,
    models::{general::{
        ManifestItem, 
        RoboCourierSpecification, 
        TestSpecifications
    }, create_delivery::CreateDeliveryRequestTypes, create_quote}, CreateQuoteRequest
};

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

    let auth_request = AuthRequest::new(&client_id, &client_secret);
    let auth_response = auth(auth_request).await?;
    let access_token = &auth_response.access_token;

    println!("Access Key: => '{:#?}'", &auth_response);

    let dropoff_address = "123 Main St, San Francisco, CA, 94103";
    let dropoff_name = "Dropoff Location";
    let dropoff_phone_number = "+1-555-555-5555";
    let manifest = "Delivery items";
    let manifest_items = vec![ManifestItem::new("Robin", 1, "small")];
    let pickup_address = "456 Market St, San Francisco, CA, 94103";
    let pickup_name = "Pickup Location";
    let pickup_phone_number = "+1-555-555-5555";
    let test_specifications = TestSpecifications {
        robo_courier_specification: RoboCourierSpecification {
            mode: "auto".to_owned()
        }
    };
    let create_delivery_request = CreateDeliveryRequest::new_with_test(dropoff_address, dropoff_name, dropoff_phone_number, manifest, manifest_items, pickup_address, pickup_name, pickup_phone_number, test_specifications);
    
    let create_delivery_response = create_delivery(&access_token, &customer_id, create_delivery_request).await?;

    println!("Create Delivery Response => {:#?}", &create_delivery_response);

    let create_quote_request = CreateQuoteRequest::new(pickup_address, dropoff_address);

    let create_quote_response = create_quote(&access_token, &customer_id, create_quote_request).await?;

    println!("Create Quote Response => {:#?}", &create_quote_response);

    let delivery_id = create_delivery_response.id.unwrap();

    let get_delivery_response = get_delivery(&access_token, &customer_id, &delivery_id).await?;

    println!("Get Delivery Response => {:#?}", &get_delivery_response);

    let update_delivery_request = UpdateDeliveryRequest{
        dropoff_notes: Some("Introduce yourself as John John".to_owned()),
        ..Default::default()
    };

    let update_delivery_response = update_delivery(&access_token, &customer_id, &delivery_id, update_delivery_request).await?;

    println!("Update Delivery Response => {:#?}", &update_delivery_response);

    // let cancel_delivery_response = cancel_delivery(&access_token, &customer_id, &delivery_id).await?;

    // println!("Cancel Delivery Response => {:#?}", &cancel_delivery_response);

    // let list_deliveries_response = list_deliveries(&access_token, &customer_id, None, None, None).await?;

    // println!("List Deliveries Response => {:#?}", &list_deliveries_response);

    Ok(())
}


