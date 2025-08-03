use axum::http::StatusCode;

pub struct HttpError {
    pub message: String,
    pub status: StatusCode,
}

impl HttpError {
    pub fn new(message: String, status: StatusCode) -> Self {
        Self { message, status }
    }
    // 300
    pub fn multiple_choices(message: String) -> Self {
        Self::new(message, StatusCode::MULTIPLE_CHOICES)
    }
    // 301
    pub fn moved_permanently(message: String) -> Self {
        Self::new(message, StatusCode::MOVED_PERMANENTLY)
    }
    // 302
    pub fn found(message: String) -> Self {
        Self::new(message, StatusCode::FOUND)
    }
    // 303
    pub fn see_other(message: String) -> Self {
        Self::new(message, StatusCode::SEE_OTHER)
    }
    // 304
    pub fn not_modified(message: String) -> Self {
        Self::new(message, StatusCode::NOT_MODIFIED)
    }
    // 305
    pub fn use_proxy(message: String) -> Self {
        Self::new(message, StatusCode::USE_PROXY)
    }
    // 307
    pub fn temporary_redirect(message: String) -> Self {
        Self::new(message, StatusCode::TEMPORARY_REDIRECT)
    }
    // 308
    pub fn permanent_redirect(message: String) -> Self {
        Self::new(message, StatusCode::PERMANENT_REDIRECT)
    }
    // 400
    pub fn bad_request(message: String) -> Self {
        Self::new(message, StatusCode::BAD_REQUEST)
    }
    // 401
    pub fn unauthorized(message: String) -> Self {
        Self::new(message, StatusCode::UNAUTHORIZED)
    }
    // 402
    pub fn payment_required(message: String) -> Self {
        Self::new(message, StatusCode::PAYMENT_REQUIRED)
    }
    // 403
    pub fn forbidden(message: String) -> Self {
        Self::new(message, StatusCode::FORBIDDEN)
    }
    // 404
    pub fn not_found(message: String) -> Self {
        Self::new(message, StatusCode::NOT_FOUND)
    }
    // 405
    pub fn method_not_allowed(message: String) -> Self {
        Self::new(message, StatusCode::METHOD_NOT_ALLOWED)
    }
    // 406
    pub fn not_acceptable(message: String) -> Self {
        Self::new(message, StatusCode::NOT_ACCEPTABLE)
    }
    // 407
    pub fn proxy_authentication_required(message: String) -> Self {
        Self::new(message, StatusCode::PROXY_AUTHENTICATION_REQUIRED)
    }
    // 408
    pub fn request_timeout(message: String) -> Self {
        Self::new(message, StatusCode::REQUEST_TIMEOUT)
    }
    // 409
    pub fn conflict(message: String) -> Self {
        Self::new(message, StatusCode::CONFLICT)
    }
    // 410
    pub fn gone(message: String) -> Self {
        Self::new(message, StatusCode::GONE)
    }
    // 411
    pub fn length_required(message: String) -> Self {
        Self::new(message, StatusCode::LENGTH_REQUIRED)
    }
    // 412
    pub fn precondition_failed(message: String) -> Self {
        Self::new(message, StatusCode::PRECONDITION_FAILED)
    }
    // 413
    pub fn payload_too_large(message: String) -> Self {
        Self::new(message, StatusCode::PAYLOAD_TOO_LARGE)
    }
    // 414
    pub fn uri_too_long(message: String) -> Self {
        Self::new(message, StatusCode::URI_TOO_LONG)
    }
    // 415
    pub fn unsupported_media_type(message: String) -> Self {
        Self::new(message, StatusCode::UNSUPPORTED_MEDIA_TYPE)
    }
    // 416
    pub fn range_not_satisfiable(message: String) -> Self {
        Self::new(message, StatusCode::RANGE_NOT_SATISFIABLE)
    }
    // 417
    pub fn expectation_failed(message: String) -> Self {
        Self::new(message, StatusCode::EXPECTATION_FAILED)
    }
    // 418
    pub fn im_a_teapot(message: String) -> Self {
        Self::new(message, StatusCode::IM_A_TEAPOT)
    }
    // 421
    pub fn misdirected_request(message: String) -> Self {
        Self::new(message, StatusCode::MISDIRECTED_REQUEST)
    }
    // 422
    pub fn unprocessable_entity(message: String) -> Self {
        Self::new(message, StatusCode::UNPROCESSABLE_ENTITY)
    }
    // 423
    pub fn locked(message: String) -> Self {
        Self::new(message, StatusCode::LOCKED)
    }
    // 424
    pub fn failed_dependency(message: String) -> Self {
        Self::new(message, StatusCode::FAILED_DEPENDENCY)
    }
    // 425
    pub fn too_early(message: String) -> Self {
        Self::new(message, StatusCode::TOO_EARLY)
    }
    // 426
    pub fn upgrade_required(message: String) -> Self {
        Self::new(message, StatusCode::UPGRADE_REQUIRED)
    }
    // 428
    pub fn precondition_required(message: String) -> Self {
        Self::new(message, StatusCode::PRECONDITION_REQUIRED)
    }
    // 429
    pub fn too_many_requests(message: String) -> Self {
        Self::new(message, StatusCode::TOO_MANY_REQUESTS)
    }
    // 431
    pub fn request_header_fields_too_large(message: String) -> Self {
        Self::new(message, StatusCode::REQUEST_HEADER_FIELDS_TOO_LARGE)
    }
    // 451
    pub fn unavailable_for_legal_reasons(message: String) -> Self {
        Self::new(message, StatusCode::UNAVAILABLE_FOR_LEGAL_REASONS)
    }
    // 500
    pub fn internal_server_error(message: String) -> Self {
        Self::new(message, StatusCode::INTERNAL_SERVER_ERROR)
    }
    // 501
    pub fn not_implemented(message: String) -> Self {
        Self::new(message, StatusCode::NOT_IMPLEMENTED)
    }
    // 502
    pub fn bad_gateway(message: String) -> Self {
        Self::new(message, StatusCode::BAD_GATEWAY)
    }
    // 503
    pub fn service_unavailable(message: String) -> Self {
        Self::new(message, StatusCode::SERVICE_UNAVAILABLE)
    }
    // 504
    pub fn gateway_timeout(message: String) -> Self {
        Self::new(message, StatusCode::GATEWAY_TIMEOUT)
    }
    // 505
    pub fn http_version_not_supported(message: String) -> Self {
        Self::new(message, StatusCode::HTTP_VERSION_NOT_SUPPORTED)
    }
    // 506
    pub fn variant_also_negotiates(message: String) -> Self {
        Self::new(message, StatusCode::VARIANT_ALSO_NEGOTIATES)
    }
    // 507
    pub fn insufficient_storage(message: String) -> Self {
        Self::new(message, StatusCode::INSUFFICIENT_STORAGE)
    }
    // 508
    pub fn loop_detected(message: String) -> Self {
        Self::new(message, StatusCode::LOOP_DETECTED)
    }
    // 510
    pub fn not_extended(message: String) -> Self {
        Self::new(message, StatusCode::NOT_EXTENDED)
    }
    // 511
    pub fn network_authentication_required(message: String) -> Self {
        Self::new(message, StatusCode::NETWORK_AUTHENTICATION_REQUIRED)
    }
}
