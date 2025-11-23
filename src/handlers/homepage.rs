use crate::error::Result;
use crate::models::academy::AcademyInfo;
use askama::Template as AskamaTemplate;
use axum::response::IntoResponse;

/// Template for the homepage
#[derive(AskamaTemplate)]
#[template(path = "homepage.html")]
struct HomepageTemplate {
    academy_name: String,
    academy_name_star: String,
    academy_name_base: String,
    academy_name_accent: String,
    tagline: String,
}

/// Handler for GET /
/// Renders the academy homepage with all information sections
pub async fn get_homepage() -> Result<impl IntoResponse> {
    // Load academy information (currently hard-coded, could be from config file in future)
    let academy_info = AcademyInfo::default();

    // Create template context
    // Progressive enhancement: This works without JavaScript
    let name = academy_info.name;
    let mut chars: Vec<char> = name.chars().collect();
    let star = if !chars.is_empty() && chars[0] == '☆' {
        chars.remove(0).to_string()
    } else {
        String::new()
    };
    let accent = if !chars.is_empty() {
        chars.pop().map(|c| c.to_string()).unwrap_or_default()
    } else {
        String::new()
    };
    let base: String = chars.into_iter().collect();

    let template = HomepageTemplate {
        academy_name: name,
        academy_name_star: star,
        academy_name_base: base,
        academy_name_accent: accent,
        tagline: academy_info.tagline,
    };

    // Render HTML - errors automatically converted to AppError via ?
    let html = template.render()?;
    Ok(axum::response::Html(html))
}
