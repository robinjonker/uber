# Uber API
 
## Uber has a vast array of integrations available, from using them as your logistics provider or integrating your restaurant onto their platform.

### - Uber Direct (Uber Logistics - delivery vehicles)

### - Uber Menu (Uber Groceries - host your restaurant on their platform) *in-progress

 ### This crate has the relevant DaaS endpoints to make Uber Direct API calls along with all the models. This allows you to call the relevant API you want and not have to worry about how Uber wants to receive and handle the data.
 
 - The functions state which API can be called, the full documentation of that endpoint along with all the parameters needed for it, the complete Uber Direct docs - last updated 9/2/2023 - can be found within each function. The general flow is that each endpoint takes a request struct as its parameter. The docs for the direct parameters are also shown at each request struct. Create the request struct with the relevant information and simply pass that into the function to call the Uber Direct API.

- It contains functions for the following:

| Functions |
| :--- |
| Get authorization codes |
| Get a quote for a delivery |
| Create a new delivery |
| Cancel a delivery |
| Update a delivery |
| Get the information about the delivery |
| Get a list of deliveries |
| Get the returned proof of delivery document from driver |
 
 - At the time of creation, access to these APIs may require written approval from Uber. Once successful, they will provide you with the relevant customer_id, client_id, and client_secret needed for Authentication.
 
 - Since then the models required for the Uber Menu integration have been added.

 If you want to test that your authentication codes work, an example has been setup for you to run. Clone the repo: [Uber API](https://www.github.com/robinjonker/uber) and run the main file, ensuring you pass in the relevant auth fields as parameters
 
 Example: 
 ``` cargo run -- --customer-id="1234" --client-id="xyz" --client-secret="xyz" ```