//! Reusable UI components organized by category
//!
//! ## Categories
//! - `layout` - Page structure components (header, footer)
//! - `ui` - Visual primitives (icons, buttons)
//! - `maps` - Location and map components

pub mod layout;
pub mod maps;
pub mod ui;

// Re-exports for convenience (backward compatible)
pub use layout::{Footer, Header};
pub use maps::DirectionsSection;

// Preserve `components::icons::` path for icon imports
pub mod icons {
    pub use super::ui::*;
}
