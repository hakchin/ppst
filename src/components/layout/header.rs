use leptos::prelude::*;
use leptos_router::components::A;

use crate::components::icons::{CloseIcon, MenuIcon};

/// Navigation menu items - anchor links for home page sections
/// Uses /#section format to work from any page (navigates to home then scrolls)
const ANCHOR_NAV_ITEMS: &[(&str, &str)] = &[
    ("/#mission", "미션"),
    ("/#programs", "프로그램"),
    ("/#admissions", "입학안내"),
    ("/#policies", "학원규칙"),
    ("/#contact", "문의/입회"),
];

/// Site header with navigation - uses anchor links for smooth scrolling to sections
/// Mobile menu uses CSS-only <details>/<summary> pattern for reliability
#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="bg-white border-b border-gray-200 sticky top-0 z-50">
            <nav class="container-section">
                <div class="flex items-center justify-between h-16">
                    // Logo - navigates to home page
                    <a href="/" class="text-xl font-bold text-brand-600 tracking-tight whitespace-nowrap">
                        <span>"☆"</span><span class="text-black">"별을"</span><span>"셀"</span>
                    </a>

                    // Desktop navigation
                    <div class="hidden md:flex items-center space-x-8">
                        {ANCHOR_NAV_ITEMS.iter().map(|(href, label)| view! {
                            <a
                                href=*href
                                class="text-gray-600 hover:text-brand-600 font-medium transition-colors"
                            >
                                {*label}
                            </a>
                        }).collect_view()}
                        <A
                            href="/about"
                            attr:class="text-gray-600 hover:text-brand-600 font-medium transition-colors"
                        >
                            "about"
                        </A>
                    </div>

                    // Mobile menu - CSS-only using details/summary
                    <details class="md:hidden mobile-menu-details">
                        <summary class="p-2 text-gray-600 hover:text-gray-900 cursor-pointer list-none">
                            <span class="sr-only">"메뉴"</span>
                            <MenuIcon class="w-6 h-6 hamburger-icon"/>
                            <CloseIcon class="w-6 h-6 close-icon"/>
                        </summary>
                        <div class="mobile-menu-content absolute left-0 right-0 top-16 bg-white border-b border-gray-200 shadow-lg">
                            <div class="container-section py-2">
                                {ANCHOR_NAV_ITEMS.iter().map(|(href, label)| view! {
                                    <a
                                        href=*href
                                        class="block px-3 py-3 text-base font-medium text-gray-600 hover:text-brand-600 hover:bg-gray-50 rounded-md"
                                        onclick="this.closest('details').open=false"
                                    >
                                        {*label}
                                    </a>
                                }).collect_view()}
                                <A
                                    href="/about"
                                    attr:class="block px-3 py-3 text-base font-medium text-gray-600 hover:text-brand-600 hover:bg-gray-50 rounded-md"
                                    attr:onclick="this.closest('details').open=false"
                                >
                                    "about"
                                </A>
                            </div>
                        </div>
                    </details>
                </div>
            </nav>
        </header>
    }
}
