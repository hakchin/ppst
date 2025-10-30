use axum::{routing::{get, post}, Router};
use crate::handlers;

pub fn create_router() -> Router {
    Router::new()
        // Homepage route (User Story 1)
        .route("/", get(handlers::homepage::get_homepage))
        // Contact form submission route (User Story 2)
        .route("/contact", post(handlers::contact::post_contact))
}
