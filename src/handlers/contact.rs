use askama_axum::Template;
use axum::{
    extract::ConnectInfo,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Redirect, Response},
    Form,
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use crate::models::contact::{ContactFormInput, ValidationError};
use crate::storage::file_store;

/// Template for validation errors (HTMX fragment)
#[derive(Template)]
#[template(path = "partials/contact_error.html")]
struct ContactErrorTemplate {
    errors: Vec<String>,
}

/// Template for success message (HTMX fragment)
#[derive(Template)]
#[template(path = "partials/contact_success.html")]
struct ContactSuccessTemplate {}

/// Simple in-memory rate limiter using HashMap
/// Maps IP address to last submission timestamp
type RateLimiter = Arc<Mutex<HashMap<String, Instant>>>;

lazy_static::lazy_static! {
    static ref RATE_LIMITER: RateLimiter = Arc::new(Mutex::new(HashMap::new()));
}

const RATE_LIMIT_DURATION: Duration = Duration::from_secs(30);

/// Check if IP is rate limited (submitted within last 30 seconds)
fn is_rate_limited(ip: &str) -> bool {
    let mut limiter = RATE_LIMITER.lock().unwrap();

    if let Some(&last_submit) = limiter.get(ip) {
        if last_submit.elapsed() < RATE_LIMIT_DURATION {
            return true;
        }
    }

    // Update timestamp for this IP
    limiter.insert(ip.to_string(), Instant::now());
    false
}

/// Handler for POST /contact
/// Supports both standard POST (redirect) and HTMX (HTML fragment)
pub async fn post_contact(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    Form(form): Form<ContactFormInput>,
) -> Response {
    // Extract IP address for rate limiting
    let ip = addr.ip().to_string();

    // Check rate limiting
    if is_rate_limited(&ip) {
        return rate_limit_response(&headers);
    }

    // Extract user agent
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    // Validate and convert to ContactInquiry
    let inquiry = match form.into_inquiry(Some(ip), user_agent) {
        Ok(inquiry) => inquiry,
        Err(validation_error) => {
            return validation_error_response(&headers, validation_error);
        }
    };

    // Save to file
    if let Err(e) = file_store::save_contact_inquiry(&inquiry) {
        eprintln!("Failed to save contact inquiry: {}", e);
        return storage_error_response(&headers);
    }

    // Return success response (dual-mode: HTMX or redirect)
    success_response(&headers)
}

/// Return rate limit error response
fn rate_limit_response(headers: &HeaderMap) -> Response {
    let is_htmx = headers.get("hx-request").is_some();

    if is_htmx {
        // HTMX: Return HTML fragment
        let template = ContactErrorTemplate {
            errors: vec!["Please wait 30 seconds before submitting another message.".to_string()],
        };
        (StatusCode::TOO_MANY_REQUESTS, template).into_response()
    } else {
        // Standard: Return 429 with plain text
        (
            StatusCode::TOO_MANY_REQUESTS,
            "Rate limit exceeded. Please wait 30 seconds before submitting again.",
        )
            .into_response()
    }
}

/// Return validation error response
fn validation_error_response(headers: &HeaderMap, error: ValidationError) -> Response {
    let is_htmx = headers.get("hx-request").is_some();

    if is_htmx {
        // HTMX: Return HTML fragment with errors
        let template = ContactErrorTemplate {
            errors: error.errors,
        };
        (StatusCode::BAD_REQUEST, template).into_response()
    } else {
        // Standard: Return 400 with error list
        let error_text = error.errors.join("\n");
        (StatusCode::BAD_REQUEST, error_text).into_response()
    }
}

/// Return storage error response
fn storage_error_response(headers: &HeaderMap) -> Response {
    let is_htmx = headers.get("hx-request").is_some();

    if is_htmx {
        let template = ContactErrorTemplate {
            errors: vec!["Failed to save your message. Please try again later.".to_string()],
        };
        (StatusCode::INTERNAL_SERVER_ERROR, template).into_response()
    } else {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to save your message. Please try again later.",
        )
            .into_response()
    }
}

/// Return success response
fn success_response(headers: &HeaderMap) -> Response {
    let is_htmx = headers.get("hx-request").is_some();

    if is_htmx {
        // HTMX: Return success HTML fragment
        ContactSuccessTemplate {}.into_response()
    } else {
        // Standard: Redirect to homepage with success parameter
        Redirect::to("/?success=true").into_response()
    }
}
