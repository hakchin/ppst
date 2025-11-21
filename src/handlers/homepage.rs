use crate::config::academy_data;
use crate::models::academy::{Instructor, Program};
use askama::Template as AskamaTemplate;
use axum::response::{Html, IntoResponse};

/// Template for the homepage
#[derive(AskamaTemplate)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    academy_name: String,
    tagline: String,
    // These fields are prepared for future use when templates become dynamic
    #[allow(dead_code)]
    programs: Vec<Program>,
    #[allow(dead_code)]
    instructors: Vec<Instructor>,
}

/// Template for server errors
#[derive(AskamaTemplate)]
#[template(path = "partials/server_error.html")]
struct ServerErrorTemplate {}

/// Handler for GET /
/// Renders the academy homepage with all information sections
pub async fn get_homepage() -> impl IntoResponse {
    // Load academy information from config module
    let academy_info = academy_data::get_academy_info();

    // Create template context
    // Progressive enhancement: This works without JavaScript
    // askama_axum::Template implements IntoResponse automatically
    let template = HomepageTemplate {
        academy_name: academy_info.name,
        tagline: academy_info.tagline,
        programs: academy_info.programs,
        instructors: academy_info.instructors,
    };

    // Render HTML with proper error handling
    match template.render() {
        Ok(html) => Html(html),
        Err(e) => {
            tracing::error!("Failed to render homepage template: {}", e);
            // Fallback to error template
            let error_template = ServerErrorTemplate {};
            match error_template.render() {
                Ok(html) => Html(html),
                Err(e) => {
                    tracing::error!("Failed to render error template: {}", e);
                    Html("<h1>Internal Server Error</h1><p>Failed to render page</p>".to_string())
                }
            }
        }
    }
}
