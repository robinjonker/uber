#[cfg(test)]
mod cancel_delivery_tests {
    use reqwest::StatusCode;
    use uber_api::models::cancel_delivery::convert_status_to_message_cancel;

    use super::*;

    #[test]
    fn test_convert_status_to_message_cancel_ok() {
        assert_eq!(convert_status_to_message_cancel(StatusCode::OK), "Success!");
    }

    #[test]
    fn test_convert_status_to_message_cancel_bad_request() {
        assert_eq!(convert_status_to_message_cancel(StatusCode::BAD_REQUEST), "Delivery cannot be cancelled.");
    }

    #[test]
    fn test_convert_status_to_message_cancel_not_found_customer() {
        let status = StatusCode::NOT_FOUND;
        let reason = "customer not found";
        let mut builder = http::response::Builder::new();
        builder.status(status);
        builder.header(http::header::CONTENT_TYPE, "text/plain");
        builder.body(reason.into()).unwrap();
        let response = builder.build().unwrap();

        assert_eq!(convert_status_to_message_cancel(response.status()), "Customer does not exist.");
    }

    #[test]
    fn test_convert_status_to_message_cancel_not_found_other() {
        assert_eq!(convert_status_to_message_cancel(StatusCode::NOT_FOUND), "The requested delivery does not exist.");
    }

    #[test]
    fn test_convert_status_to_message_cancel_request_timeout() {
        assert_eq!(convert_status_to_message_cancel(StatusCode::REQUEST_TIMEOUT), "The request timed out.");
    }

    #[test]
    fn test_convert_status_to_message_cancel_internal_server_error() {
        assert_eq!(convert_status_to_message_cancel(StatusCode::INTERNAL_SERVER_ERROR), "An unknown error happened.");
    }

    #[test]
    fn test_convert_status_to_message_cancel_service_unavailable() {
        assert_eq!(convert_status_to_message_cancel(StatusCode::SERVICE_UNAVAILABLE), "Service is currently unavailable.");
    }

    #[test]
    fn test_convert_status_to_message_cancel_unknown() {
        assert_eq!(convert_status_to_message_cancel(StatusCode::IM_A_TEAPOT), "Unknown status code.");
    }
}
