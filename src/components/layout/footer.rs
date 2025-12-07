use leptos::prelude::*;
use leptos_router::components::A;

use crate::constants::contact;

/// Site footer
#[component]
pub fn Footer() -> impl IntoView {
    let current_year = time::OffsetDateTime::now_utc().year();

    view! {
        <footer class="bg-gray-900 text-gray-300">
            <div class="container-section py-12">
                <div class="grid grid-cols-1 md:grid-cols-4 gap-8">
                    // Brand column
                    <div class="col-span-1 md:col-span-2">
                        <h3 class="text-xl font-bold text-white mb-4">"★별을셀수학"</h3>
                        <p class="text-gray-400 max-w-md">
                            "수학을 가르칩니다® · 수학이 재미있는 곳 ★별을셀입니다"
                        </p>
                        <p class="text-gray-400 max-w-md">
                            "★별을셀, 학생들을 사랑합니다."
                        </p>
                    </div>

                    // Quick links
                    <div>
                        <h4 class="text-sm font-semibold text-white uppercase tracking-wider mb-4">
                            "바로가기"
                        </h4>
                        <ul class="space-y-2">
                            <FooterAnchorLink href="/#mission">"미션"</FooterAnchorLink>
                            <FooterAnchorLink href="/#programs">"프로그램"</FooterAnchorLink>
                            <FooterAnchorLink href="/#admissions">"입학안내"</FooterAnchorLink>
                            <FooterAnchorLink href="/#policies">"학원규칙"</FooterAnchorLink>
                            <FooterAnchorLink href="/#contact">"문의/입회"</FooterAnchorLink>
                            <FooterLink href="/about">"about"</FooterLink>
                        </ul>
                    </div>

                    // Contact info
                    <div>
                        <h4 class="text-sm font-semibold text-white uppercase tracking-wider mb-4">
                            "연락처"
                        </h4>
                        <ul class="space-y-2 text-gray-400">
                            <li>{contact::ADDRESS_FULL}</li>
                            <li>"(지번) 경기도 군포시 산본동 1142-7"</li>
                            <li>"웹사이트 " {contact::WEBSITE}</li>
                            <li>"전화 " {contact::PHONE}</li>
                        </ul>
                    </div>
                </div>

                // Copyright
                <div class="mt-12 pt-8 border-t border-gray-800 text-center text-gray-400 text-sm">
                    <p>"© " {current_year} " ★별을셀수학. All rights reserved."</p>
                </div>
            </div>
        </footer>
    }
}

/// Footer link for internal routes (uses leptos_router A component)
#[component]
fn FooterLink(href: &'static str, children: Children) -> impl IntoView {
    view! {
        <li>
            <A
                href=href
                attr:class="text-gray-400 hover:text-white transition-colors"
            >
                {children()}
            </A>
        </li>
    }
}

/// Footer link for anchor links (uses regular <a> tag)
#[component]
fn FooterAnchorLink(href: &'static str, children: Children) -> impl IntoView {
    view! {
        <li>
            <a
                href=href
                class="text-gray-400 hover:text-white transition-colors"
            >
                {children()}
            </a>
        </li>
    }
}
