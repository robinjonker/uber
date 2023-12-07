#[cfg(test)]
mod create_delivery_tests {
    use reqwest::StatusCode;
    use uber_api::models::create_delivery::convert_status_to_message_create;

    use super::*;

    #[test]
    fn test_convert_status_to_message_create_success() {
        let result = convert_status_to_message_create(StatusCode::OK);
        assert_eq!(result, "Success!");
    }

    #[test]
    fn test_convert_status_to_message_create_duplicate_delivery() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("An active delivery like this already exists. A pointer to the other delivery is provided."));
    }

    #[test]
    fn test_convert_status_to_message_create_invalid_params() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("The parameters of your request were invalid."));
    }

    #[test]
    fn test_convert_status_to_message_create_unknown_location() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("The specified location was not understood."));
    }

    #[test]
    fn test_convert_status_to_message_create_address_undeliverable() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("The specified location is not in a deliverable area."));
    }

    #[test]
    fn test_convert_status_to_message_create_expired_quote() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("The price quote specified has expired."));
    }

    #[test]
    fn test_convert_status_to_message_create_used_quote() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("The price quote specified has already been used."));
    }

    #[test]
    fn test_convert_status_to_message_create_mismatched_price_quote() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("The price quote specified doesn’t match the delivery."));
    }

    #[test]
    fn test_convert_status_to_message_create_missing_payment() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("Your account’s payment information has not been provided."));
    }

    #[test]
    fn test_convert_status_to_message_create_pickup_ready_time_not_specified() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("Pickup ready time must be specified when passing in pickup/dropoff windows."));
    }

    #[test]
    fn test_convert_status_to_message_create_pickup_window_too_small() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("The pickup window needs to be at least 10 minutes long."));
    }

    #[test]
    fn test_convert_status_to_message_create_dropoff_deadline_too_early() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("The dropoff deadline needs to be at least 20 minutes after the dropoff ready time."));
    }

    #[test]
    fn test_convert_status_to_message_create_dropoff_deadline_before_pickup_deadline() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("The dropoff deadline needs to be after the pickup deadline."));
    }

    #[test]
    fn test_convert_status_to_message_create_dropoff_ready_after_pickup_deadline() {
        let result = convert_status_to_message_create(StatusCode::BAD_REQUEST);
        assert!(result.contains("The dropoff ready time needs to be at or before the pickup deadline."));
    }

    #[test]
    fn test_bad_request_pickup_ready_too_early() {
        let status = StatusCode::BAD_REQUEST;
        status.set_canonical_reason(Some("pickup_ready_too_early"));
        assert_eq!(convert_status_to_message_create(status), "The pickup ready time cannot be in the past.");
    }

    #[test]
    fn test_bad_request_pickup_deadline_too_early() {
        let status = StatusCode::BAD_REQUEST;
        status.set_canonical_reason(Some("pickup_deadline_too_early"));
        assert_eq!(convert_status_to_message_create(status), "The pickup deadline time needs to be at least 20 minutes from now.");
    }

    #[test]
    fn test_bad_request_pickup_ready_too_late() {
        let status = StatusCode::BAD_REQUEST;
        status.set_canonical_reason(Some("pickup_ready_too_late"));
        assert_eq!(convert_status_to_message_create(status), "The pickup ready time needs to be within the next 30 days.");
    }

    #[test]
    fn test_payment_required_customer_suspended() {
        let status = StatusCode::PAYMENT_REQUIRED;
        status.set_canonical_reason(Some("customer_suspended"));
        assert_eq!(convert_status_to_message_create(status), "Your account is passed due. Payment is required.");
    }

    #[test]
    fn test_forbidden_customer_blocked() {
        let status = StatusCode::FORBIDDEN;
        status.set_canonical_reason(Some("customer_blocked"));
        assert_eq!(convert_status_to_message_create(status), "Your account is not allowed to create deliveries.");
    }

    #[test]
    fn test_unprocessable_entity_address_undeliverable_limited_couriers() {
        let status = StatusCode::UNPROCESSABLE_ENTITY;
        status.set_canonical_reason(Some("address_undeliverable_limited_couriers"));
        assert_eq!(convert_status_to_message_create(status), "The specified location is not in a deliverable area at this time because all couriers are currently busy.");
    }

    #[test]
    fn test_too_many_requests_customer_limited() {
        let status = StatusCode::TOO_MANY_REQUESTS;
        status.set_canonical_reason(Some("customer_limited"));
        assert_eq!(convert_status_to_message_create(status), "Your account's limits have been exceeded.");
    }

    #[test]
    fn test_internal_server_error_unknown_error() {
        let status = StatusCode::INTERNAL_SERVER_ERROR;
        status.set_canonical_reason(Some("unknown_error"));
        assert_eq!(convert_status_to_message_create(status), "An unknown error happened.");
    }

    #[test]
    fn test_unknown_status_code() {
        let status = StatusCode::NOT_FOUND;
        assert_eq!(convert_status_to_message_create(status), "Unknown status code.");
    }

}
   
