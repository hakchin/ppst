use crate::models::academy::{AcademyInfo, Instructor, Program};
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

/// Handler for GET /
/// Renders the academy homepage with all information sections
pub async fn get_homepage() -> impl IntoResponse {
    // Load academy information (currently hard-coded, could be from config file in future)
    let academy_info = AcademyInfo::default();

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
            Html("<h1>Internal Server Error</h1><p>Failed to render page</p>".to_string())
        }
    }
}
