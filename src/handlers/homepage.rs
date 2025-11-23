use crate::error::Result;
use crate::models::academy::AcademyInfo;
use askama::Template as AskamaTemplate;
use axum::response::IntoResponse;

/// Template for the homepage
#[derive(AskamaTemplate)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    academy_name: String,
    tagline: String,
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
    };

    // Render HTML - errors automatically converted to AppError via ?
    let html = template.render()?;
    Ok(axum::response::Html(html))
}
