use crate::handlers;
use axum::{
    Router,
    routing::{get, post},
};

pub fn create_router() -> Router {
    Router::new()
        // Homepage route (User Story 1)
        .route("/", get(handlers::homepage::get_homepage))
        // Contact form submission route (User Story 2)
        .route("/contact", post(handlers::contact::post_contact))
}
