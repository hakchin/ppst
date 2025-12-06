use leptos::prelude::*;

/// Navigation menu items - single source of truth for both Desktop and Mobile
const NAV_ITEMS: &[(&str, &str)] = &[
    ("#mission", "미션"),
    ("#programs", "프로그램"),
    ("#admissions", "입학안내"),
    ("#policies", "학원규칙"),
    ("#contact", "문의/입회"),
    ("#about", "about"),
];

/// Site header with navigation - uses anchor links for smooth scrolling to sections
#[component]
pub fn Header() -> impl IntoView {
    let (menu_open, set_menu_open) = signal(false);

    // Close menu when clicking an anchor link (for mobile UX)
    let close_menu = move |_| set_menu_open.set(false);

    view! {
        <header class="bg-white border-b border-gray-200 sticky top-0 z-50">
            <nav class="container-section">
                <div class="flex items-center justify-between h-16">
                    // Logo - scrolls to top
                    <a href="#page-top" class="text-xl font-bold text-brand-600 tracking-tight whitespace-nowrap">
                        <span>"☆"</span><span class="text-black">"별을"</span><span>"셀"</span>
                    </a>

                    // Desktop navigation
                    <div class="hidden md:flex items-center space-x-8">
                        {NAV_ITEMS.iter().map(|(href, label)| view! {
                            <a
                                href=*href
                                class="text-gray-600 hover:text-brand-600 font-medium transition-colors"
                            >
                                {*label}
                            </a>
                        }).collect_view()}
                    </div>

                    // Mobile menu button
                    <button
                        type="button"
                        class="md:hidden p-2 text-gray-600 hover:text-gray-900"
                        on:click=move |_| set_menu_open.update(|v| *v = !*v)
                    >
                        <span class="sr-only">"메뉴 열기"</span>
                        <svg class="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d=move || if menu_open.get() {
                                    "M6 18L18 6M6 6l12 12"
                                } else {
                                    "M4 6h16M4 12h16M4 18h16"
                                }
                            />
                        </svg>
                    </button>
                </div>

                // Mobile navigation
                <div
                    class="md:hidden"
                    class:hidden=move || !menu_open.get()
                >
                    <div class="px-2 pt-2 pb-3 space-y-1">
                        {NAV_ITEMS.iter().map(|(href, label)| view! {
                            <a
                                href=*href
                                class="block px-3 py-2 text-base font-medium text-gray-600 hover:text-brand-600 hover:bg-gray-50 rounded-md"
                                on:click=close_menu
                            >
                                {*label}
                            </a>
                        }).collect_view()}
                    </div>
                </div>
            </nav>
        </header>
    }
}
