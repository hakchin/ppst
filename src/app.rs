use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::{Footer, Header};
use crate::pages::{AboutPage, AdmissionsPage, ContactPage, HomePage, MissionPage, NotFoundPage, PoliciesPage, ProgramsPage};

/// Root application component
#[component]
pub fn App() -> impl IntoView {
    // Provides context for metadata management
    provide_meta_context();

    view! {
        <Title text="☆별을셀 — Excellence in Mathematics Education"/>
        <Meta name="description" content="수학을 가르칩니다® · 수학이 재미있는 곳 ★별을셀입니다"/>

        <Router>
            <div class="min-h-screen flex flex-col">
                <Header/>
                <main class="flex-1">
                    <Routes fallback=|| view! { <NotFoundPage/> }.into_any()>
                        <Route path=path!("/") view=|| view! { <HomePage/> }.into_any()/>
                        <Route path=path!("/about") view=|| view! { <AboutPage/> }.into_any()/>
                        <Route path=path!("/mission") view=|| view! { <MissionPage/> }.into_any()/>
                        <Route path=path!("/programs") view=|| view! { <ProgramsPage/> }.into_any()/>
                        <Route path=path!("/admissions") view=|| view! { <AdmissionsPage/> }.into_any()/>
                        <Route path=path!("/policies") view=|| view! { <PoliciesPage/> }.into_any()/>
                        <Route path=path!("/contact") view=|| view! { <ContactPage/> }.into_any()/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}
