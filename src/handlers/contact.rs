use askama::Template;
use axum::http::{HeaderValue, header};
use axum::{
    Form,
    extract::ConnectInfo,
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse, Redirect, Response},
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, LazyLock, Mutex};
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

static RATE_LIMITER: LazyLock<RateLimiter> = LazyLock::new(|| Arc::new(Mutex::new(HashMap::new())));

static RATE_LIMIT_DURATION: LazyLock<Duration> = LazyLock::new(|| {
    use std::env;
    let seconds = env::var("RATE_LIMIT_SECONDS")
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(30);
    Duration::from_secs(seconds)
});

/// Check if IP is rate limited (submitted within last 30 seconds)
fn is_rate_limited(ip: &str) -> bool {
    let mut limiter = RATE_LIMITER.lock().unwrap();

    if let Some(&last_submit) = limiter.get(ip)
        && last_submit.elapsed() < *RATE_LIMIT_DURATION
    {
        return true;
    }

    // Update timestamp for this IP
    limiter.insert(ip.to_string(), Instant::now());
    false
}

/// Helper: Check if request is from HTMX
fn is_htmx_request(headers: &HeaderMap) -> bool {
    headers.get("hx-request").is_some()
}

/// Helper: Render template with error handling
#[allow(clippy::result_large_err)]
fn render_template_or_fallback<T: Template>(
    template: T,
    fallback: String,
) -> Result<String, Response> {
    template.render().map_err(|e| {
        tracing::error!("Template render error: {}", e);
        (StatusCode::INTERNAL_SERVER_ERROR, fallback).into_response()
    })
}

/// Helper: Create dual-mode response (HTMX fragment or standard)
fn create_error_response(
    headers: &HeaderMap,
    status: StatusCode,
    errors: Vec<String>,
    plain_text: String,
) -> Response {
    if is_htmx_request(headers) {
        let template = ContactErrorTemplate { errors };
        match render_template_or_fallback(template, "An error occurred".to_string()) {
            Ok(html) => (status, Html(html)).into_response(),
            Err(response) => response,
        }
    } else {
        (status, plain_text).into_response()
    }
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
        tracing::error!("Failed to save contact inquiry: {}", e);
        return storage_error_response(&headers);
    }

    // Return success response (dual-mode: HTMX or redirect)
    success_response(&headers)
}

pub async fn export_contacts_ndjson() -> Response {
    match file_store::export_contacts_ndjson() {
        Ok(body) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/x-ndjson"),
            );
            (StatusCode::OK, headers, body).into_response()
        }
        Err(e) => {
            tracing::error!("{}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Export error").into_response()
        }
    }
}

pub async fn export_contacts_json() -> Response {
    match file_store::export_contacts_json_array_pretty() {
        Ok(body) => {
            let mut headers = HeaderMap::new();
            headers.insert(
                header::CONTENT_TYPE,
                HeaderValue::from_static("application/json"),
            );
            (StatusCode::OK, headers, body).into_response()
        }
        Err(e) => {
            tracing::error!("{}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, "Export error").into_response()
        }
    }
}

/// Return rate limit error response
fn rate_limit_response(headers: &HeaderMap) -> Response {
    create_error_response(
        headers,
        StatusCode::TOO_MANY_REQUESTS,
        vec!["Please wait 30 seconds before submitting another message.".to_string()],
        "Rate limit exceeded. Please wait 30 seconds before submitting again.".to_string(),
    )
}

/// Return validation error response
fn validation_error_response(headers: &HeaderMap, error: ValidationError) -> Response {
    let plain_text = error.errors.join("\n");
    create_error_response(headers, StatusCode::BAD_REQUEST, error.errors, plain_text)
}

/// Return storage error response
fn storage_error_response(headers: &HeaderMap) -> Response {
    create_error_response(
        headers,
        StatusCode::INTERNAL_SERVER_ERROR,
        vec!["Failed to save your message. Please try again later.".to_string()],
        "Failed to save your message. Please try again later.".to_string(),
    )
}

/// Return success response
fn success_response(headers: &HeaderMap) -> Response {
    if is_htmx_request(headers) {
        // HTMX: Return success HTML fragment
        let template = ContactSuccessTemplate {};
        match render_template_or_fallback(
            template,
            "Success! Your message has been sent.".to_string(),
        ) {
            Ok(html) => Html(html).into_response(),
            Err(response) => response,
        }
    } else {
        // Standard: Redirect to homepage with success parameter
        Redirect::to("/?success=true").into_response()
    }
}
