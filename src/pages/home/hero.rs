use leptos::prelude::*;

/// Hero section with welcome message
#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <section class="bg-gray-900 text-white">
            <div class="container-section py-24 md:py-32">
                <div class="max-w-3xl mx-auto text-center">
                    <h1 class="text-4xl md:text-5xl lg:text-6xl font-bold mb-6 text-balance">
                        "Welcome to "
                        <span class="inline-block">
                            <span class="text-brand-400">"☆"</span>
                            "별을"
                            <span class="text-brand-400">"셀"</span>
                        </span>
                    </h1>
                    <p class="text-xl md:text-2xl text-gray-300 mb-8">
                        "Excellence in Mathematics Education"
                    </p>
                </div>
            </div>
        </section>
    }
}
