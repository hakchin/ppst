//! Home page module
//!
//! Contains the main HomePage component and all section components.

mod achievements;
mod admissions;
mod contact;
mod hero;
mod mission;
mod policies;
mod programs;
mod teaching;

use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::components::DirectionsSection;

use achievements::AchievementsSection;
use admissions::AdmissionsSection;
use contact::ContactSection;
use hero::HeroSection;
use mission::MissionSection;
use policies::PoliciesSection;
use programs::ProgramsSection;
use teaching::TeachingPhilosophySection;

/// Home page component - Single page layout with all sections (like legacy site)
#[component]
pub fn HomePage() -> impl IntoView {
    // Get reactive location - updates when URL changes
    let location = use_location();

    // Scroll to hash anchor when hash changes (for SPA navigation from other pages)
    Effect::new(move |_| {
        let hash = location.hash.get();
        if !hash.is_empty() {
            // Use request_animation_frame to ensure DOM is ready
            request_animation_frame(move || {
                if let Some(window) = leptos::web_sys::window()
                    && let Some(document) = window.document()
                {
                    // Remove the leading '#' from hash
                    let id = hash.trim_start_matches('#');
                    if let Some(element) = document.get_element_by_id(id) {
                        element.scroll_into_view();
                    }
                }
            });
        }
    });

    view! {
        <div id="page-top">
            <HeroSection/>
            <MissionSection/>
            <AchievementsSection/>
            <TeachingPhilosophySection/>
            <ProgramsSection/>
            <AdmissionsSection/>
            <PoliciesSection/>
            <ContactSection/>
            <DirectionsSection/>
        </div>
    }
}
