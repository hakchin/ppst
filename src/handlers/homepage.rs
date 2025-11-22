use crate::error::Result;
use crate::models::academy::{AcademyInfo, Instructor, Program};
use askama::Template as AskamaTemplate;
use axum::response::IntoResponse;

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
pub async fn get_homepage() -> Result<impl IntoResponse> {
    // Load academy information (currently hard-coded, could be from config file in future)
    let academy_info = AcademyInfo::default();

    // Create template context
    // Progressive enhancement: This works without JavaScript
    let template = HomepageTemplate {
        academy_name: academy_info.name,
        tagline: academy_info.tagline,
        programs: academy_info.programs,
        instructors: academy_info.instructors,
    };

    // Render HTML - errors automatically converted to AppError via ?
    let html = template.render()?;
    Ok(axum::response::Html(html))
}
