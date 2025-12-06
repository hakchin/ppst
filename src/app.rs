use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{
    components::{Route, Router, Routes},
    path,
};

use crate::components::{Footer, Header};
use crate::pages::{AboutPage, HomePage, NotFoundPage};

/// Root application component
#[component]
pub fn App() -> impl IntoView {
    // Provides context for metadata management
    provide_meta_context();

    view! {
        <Title text="별을셀 — Excellence in Mathematics Education"/>
        <Meta name="description" content="수학을 가르칩니다® · 수학이 재미있는 곳 ★별을셀입니다"/>
        <Meta name="keywords" content="Mathematics education, coding academy, School Mathematics, Euclidean Geometry, Mathematical Logic with RegEx"/>
      
        <Router>
            <div class="min-h-screen flex flex-col">
                <Header/>
                <main class="flex-1">
                    <Routes fallback=|| view! { <NotFoundPage/> }.into_any()>
                        <Route path=path!("/") view=|| view! { <HomePage/> }.into_any()/>
                        <Route path=path!("/about") view=|| view! { <AboutPage/> }.into_any()/>
                    </Routes>
                </main>
                <Footer/>
            </div>
        </Router>
    }
}
