use leptos::prelude::*;
use leptos_router::hooks::use_location;

use crate::components::DirectionsSection;
use crate::components::icons::{
    ChatIcon, CheckIcon, ClockIcon, EmailIcon, LocationIcon, PhoneIcon,
};
use crate::server_fns::submit_contact;

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

/// Hero section with welcome message
#[component]
fn HeroSection() -> impl IntoView {
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

/// Mission section from legacy mission.html
#[component]
fn MissionSection() -> impl IntoView {
    view! {
        <section id="mission" class="py-16 scroll-mt-16">
            <div class="container-section">
                <h2 class="text-3xl md:text-4xl font-bold mb-8">"Mission"</h2>
                <h3 class="text-2xl font-bold mb-6">
                    "수학을 가르칩니다"
                    <sup class="text-sm align-super">"®"</sup>
                </h3>

                <div class="mb-8 text-sm text-gray-500">
                    <ul class="space-y-1">
                        <li><strong>"author"</strong>": 별을셀, 010-5102-0841"</li>
                        <li><strong>"date"</strong>": 2016-07-13"</li>
                        <li><strong>"site status"</strong>": experimental"</li>
                        <li><strong>"site etc."</strong>": updated once in a while, not formal"</li>
                    </ul>
                </div>

                <div class="prose prose-lg max-w-none space-y-4 text-gray-700">
                    <p>"수학이 재미있는 곳 "<strong>"★별을셀"</strong>"입니다. 반짝이는 밤하늘의 별처럼 여러 분도 반짝이는 별입니다."</p>
                    <p>"개인적 수학 아이디어의 많은 부분에 대하여 Georg Cantor와 Kurt Gödel에 대하여 감사합니다."</p>
                    <p>"세속의 수학(입시수학, 공업수학, 미적분, 일반 문제풀이,...)은 이들의 것과 많이 비슷하지는 않습니다. 세속의 수학에도 많은 감사를 드립니다."</p>
                    <p>"여러 분과 함께 세속의 수학을 합니다. 동시에 개인으로서는 상상과 논리의 동산에서 자신만의 수학적 사고를 하고자 합니다."</p>
                    <p>"AI가 급속히 발전하면서 AI에 의하여 대체될 수 있는 분야에서는 많은 인력들이 밀려나고 있습니다."</p>
                    <p>"과거부터 지금까지 언제나 그랬듯이 확실한 미래는 알 수가 없습니다. 미래의 흐름에 벌써 너무 걱정하기보다는 눈 앞의 단순한 입시나 조금 더 나아가서는 순순하게 사유하는 것이 좋고, 논리적 해석을 해 보고 싶은 학생들이 있다면 함께 그 길을 같이 가고 싶습니다."</p>
                    <p>"가르침과 관련해서는 "<strong>"소수의 작은 학습장"</strong>"을 고집합니다."</p>
                    <p>"미사여구와 그럴싸한 자료와 언어로 학생과 학부모님들을 현혹하지 않습니다."</p>
                    <p>"쏟아지는 광고처럼 누가 어떤 책을 썼고, 어떤 특별한 문제집을 풀어야 되고, 저명한 누구에 의하면 수학 학습은 이렇게 해야 된다는 등 그런 말씀은 드리지 않습니다."</p>
                    <p>"정성과 진심을 가지고 함께 공부합니다."</p>
                    <p>"수학을 공부함에 있어, 결과보다는 과정에 최선을 다하고, 어렵고 힘듦이 있음을 주저하지 않으며 나아갈, "<strong>"의지의 학생님"</strong>"과 함께 합니다."</p>
                    <p>"여러 분은 과정에 최선을 다하고 별을셀은 결과를 드리고자 노력합니다."</p>
                    <p>"수학을 잘 가르치는 "<strong>"★별을셀수학"</strong>"입니다."</p>
                </div>
            </div>
        </section>
    }
}

/// Achievements section showing exam results
#[component]
fn AchievementsSection() -> impl IntoView {
    view! {
        <section class="py-16 bg-gray-50">
            <div class="container-section">
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    // 2022 Achievement
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h3 class="text-lg font-bold text-brand-600 mb-4">"2022년 11월 수능수학포함 2년 연속 전원 1등급"</h3>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 재원생 전원 수능수학 1등급 성취"</li>
                            <li>"• 진솔한 소수 학생과 함께 하고자 합니다."</li>
                        </ul>
                    </div>

                    // 2021 Achievement
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h3 class="text-lg font-bold text-brand-600 mb-4">"2021년 11월 수능수학 전원 1등급"</h3>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 충분하고 넉넉한 점수로 재원생 전원 1등급 성취 (졸업수강인원 4명)"</li>
                            <li>"• 수리논술 지원자 전원 합격"</li>
                            <li>"• 2022년 11월 수능수학 또한 1등급 도전해 봅니다"</li>
                        </ul>
                    </div>
                </div>

                // Stars message
                <div class="mt-12 card-highlight p-8">
                    <h3 class="text-xl font-bold text-brand-800 mb-4">"수학이 재미있는 곳, ★별을셀입니다"</h3>
                    <ul class="space-y-2 text-brand-700">
                        <li>"• 반짝이는 밤하늘의 별처럼 여러 분은 모두 빛납니다"</li>
                        <li>"• 반짝이는 밤하늘의 별처럼 여러 분은 각기 다르지만 모두 아름답습니다"</li>
                        <li>"• 그런 별들을 세며 꿈을 꾸고 잠이 듭니다"</li>
                        <li>"• 가르치기 전에 먼저 사랑합니다"</li>
                    </ul>
                </div>
            </div>
        </section>
    }
}

/// Teaching philosophy section
#[component]
fn TeachingPhilosophySection() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    // 입시수학 책임
                    <div>
                        <h3 class="text-2xl font-bold mb-6">"입시수학을 책임집니다"</h3>
                        <p class="text-sm text-gray-500 mb-4">"아래에서 원장의 수학철학보다는 입시관점에서 안내드림을 양해부탁드립니다."</p>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• "<strong>"원생의 반 이상이 수학 1등급"</strong>" 유지하는 유일한 곳 (간혹 안 되기도 합니다만 성취도는 월등합니다.)"</li>
                            <li>"• 별을셀이 자랑하는 "<strong>"1등급 정시반"</strong>" 운영"</li>
                            <li>"• 미사여구는 없습니다. "<strong>"올바른 과정과 결과"</strong>"입니다. 언제든 문의 주세요."</li>
                            <li>"• 타학원과 달리 ★별을셀은 "<strong>"단순히 몇 명이다"</strong>"가 아닌 "<strong>"정확히 몇 명중의 몇 명이 1등급"</strong>"인지를 말합니다."</li>
                        </ul>
                    </div>

                    // 수업방향
                    <div>
                        <h3 class="text-2xl font-bold mb-6">"수업방향"</h3>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 수학이 왜 자유로운 상상의 학문인지 보여 드립니다."</li>
                            <li>"• 진지한 수학적 접근을 끊임없이 시도합니다."</li>
                            <li>"• 수학은 자유의 학문입니다."</li>
                            <li>"• 동기부여 없는 강제적 학습을 지양합니다."</li>
                        </ul>
                        <div class="mt-8 bg-gray-50 rounded-xl p-6">
                            <h4 class="text-lg font-bold text-brand-600 mb-4">"특별한 수학 ★별을셀입니다"</h4>
                            <ul class="space-y-2 text-gray-700 text-sm">
                                <li>"• 작은 학원입니다. 학생 한 명 한 명이 존중되는 곳입니다."</li>
                                <li>"• Contact : " <a href="#contact" class="text-brand-600 hover:underline">"문의/입회"</a></li>
                                <li>"• 홈페이지나 연락처는 특별하지 않습니다. "<strong>"수학이 특별한 곳"</strong>", ★별을셀입니다."</li>
                                <li>"• 전화 : 010-5102-0841"</li>
                                <li>"• 카카오 : " <a href="https://pf.kakao.com/_MxbVfT" target="_blank" rel="noopener noreferrer" class="text-brand-600 hover:underline">"별을셀"</a> " 검색"</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Programs section (from programs_content.html)
#[component]
fn ProgramsSection() -> impl IntoView {
    view! {
        <section id="programs" class="py-16 bg-gray-50 scroll-mt-16">
            <div class="container-section">
                <h2 class="section-title">"Our Programs"</h2>
                <blockquote class="section-subtitle">
                    <p>"왕도는 없습니다."</p>
                    <p>"꼴찌부터 1등까지 아이들이 행복하기를 바랍니다."</p>
                </blockquote>

                // 고등부
                <div class="mb-12">
                    <h3 class="text-2xl font-bold mb-6 flex items-center gap-3">
                        <span class="w-3 h-3 rounded-full bg-brand-600"></span>
                        "고등부"
                    </h3>
                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                        <div class="bg-white rounded-xl p-6 border border-gray-200">
                            <h4 class="text-lg font-bold text-brand-600 mb-4">"내신반"</h4>
                            <ul class="space-y-2 text-gray-700">
                                <li>"• 원생의 반 이상이 수학 1등급 유지하는 유일한 곳 (철저한 내신관리, 수준관리)"</li>
                                <li>"• 고등부 내신반의 놀라운 성취를 꼭 확인해 보세요"</li>
                            </ul>
                        </div>
                        <div class="bg-white rounded-xl p-6 border border-gray-200">
                            <h4 class="text-lg font-bold text-brand-600 mb-4">"정시반"</h4>
                            <ul class="space-y-2 text-gray-700">
                                <li>"• 수능 1등급 유지 또는 100점 목표"</li>
                                <li>"• 고난도 문항(14,15,21,22,28,29,30) 이해와 실전 연습"</li>
                                <li>"• 고3 및 독학재수 학원생 대상"</li>
                            </ul>
                        </div>
                    </div>
                </div>

                // 중등부
                <div class="mb-12">
                    <h3 class="text-2xl font-bold mb-6 flex items-center gap-3">
                        <span class="w-3 h-3 rounded-full bg-brand-600"></span>
                        "중등부"
                    </h3>
                    <div class="space-y-4">
                        <div class="bg-white rounded-xl p-6 border border-gray-200">
                            <span class="font-semibold text-brand-600">"정규반"</span>
                            <span class="text-gray-700">" : 학년별 과정을 충실히 하며, 개인별 선행 진행합니다."</span>
                        </div>
                        <div class="bg-white rounded-xl p-6 border border-gray-200">
                            <span class="font-semibold text-brand-600">"실력정석반"</span>
                            <span class="text-gray-700">" : 무학년제. 하나의 교재만 집중하되 교재안의 단 한 문제도 놓치지 않습니다. (주 1회반, 주 2회반)"</span>
                        </div>
                        <div class="bg-white rounded-xl p-6 border border-gray-200">
                            <span class="font-semibold text-brand-600">"KMO"</span>
                            <span class="text-gray-700">" : KMO/IMO 수학 경시 특강. 월4회 수업. 2년 과정. (정수론, 기하론, 대수론, 조합론)"</span>
                        </div>
                    </div>
                </div>

                // 초등부
                <div class="mb-12">
                    <h3 class="text-2xl font-bold mb-6 flex items-center gap-3">
                        <span class="w-3 h-3 rounded-full bg-brand-600"></span>
                        "초등부"
                    </h3>
                    <div class="space-y-4">
                        <div class="bg-white rounded-xl p-6 border border-gray-200">
                            <span class="font-semibold text-brand-600">"정규반"</span>
                            <span class="text-gray-700">" : 학년별 과정을 충실히 하며, 개인별 선행 진행합니다."</span>
                        </div>
                        <div class="bg-white rounded-xl p-6 border border-gray-200">
                            <span class="font-semibold text-brand-600">"실력정석반"</span>
                            <span class="text-gray-700">" : 무학년제. 하나의 교재만 집중하되 교재안의 단 한 문제도 놓치지 않습니다. (주 1회반, 주 2회반)"</span>
                        </div>
                        <div class="bg-white rounded-xl p-6 border border-gray-200 text-gray-700">
                            "Class별 4명 이하 제한, 세밀한 지도합니다. 긴 관점으로 확실한 실력향상을 원하는 학생만 받습니다."
                        </div>
                    </div>
                </div>

                // 중3 유의사항
                <div class="mb-12">
                    <h3 class="text-2xl font-bold mb-6 flex items-center gap-3">
                        <span class="w-3 h-3 rounded-full bg-brand-600"></span>
                        "중3 유의사항"
                    </h3>
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 11월부터 고1 예비반 과정 진행합니다."</li>
                            <li>"• 기존 중등부 학생은 자동으로 11월부터 고1 예비반 전환됩니다."</li>
                        </ul>
                    </div>
                </div>

                // 교실
                <div>
                    <h3 class="text-2xl font-bold mb-6 flex items-center gap-3">
                        <span class="w-3 h-3 rounded-full bg-brand-600"></span>
                        "교실"
                    </h3>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div class="bg-white rounded-xl p-6 border border-gray-200 text-center">
                            <span class="text-sm text-gray-400">"01| "</span>
                            <span class="font-semibold">"Shooting star"</span>
                            <span class="text-gray-600">" (별똥별)"</span>
                            <span class="ml-2 text-xs bg-brand-100 text-brand-700 px-2 py-1 rounded">"M"</span>
                        </div>
                        <div class="bg-white rounded-xl p-6 border border-gray-200 text-center">
                            <span class="text-sm text-gray-400">"02| "</span>
                            <span class="font-semibold">"Polaris"</span>
                            <span class="text-gray-600">" (북극성)"</span>
                            <span class="ml-2 text-xs bg-brand-100 text-brand-700 px-2 py-1 rounded">"E, H"</span>
                        </div>
                        <div class="bg-white rounded-xl p-6 border border-gray-200 text-center">
                            <span class="text-sm text-gray-400">"03| "</span>
                            <span class="font-semibold">"Milkyway"</span>
                            <span class="text-gray-600">" (은하수)"</span>
                            <span class="ml-2 text-xs bg-brand-100 text-brand-700 px-2 py-1 rounded">"M"</span>
                        </div>
                    </div>
                </div>

                <div class="mt-12 bg-white rounded-xl p-6 border border-gray-200 text-center">
                    <h2 class="text-3xl font-bold mb-4">"어떤 프로그램이 맞을지 고민되시나요?"</h2>
                    <p class="text-gray-600 mb-8 max-w-2xl mx-auto">
                        "입시 목표와 상황에 맞는 최적의 방향을 함께 찾겠습니다."
                    </p>
                    <a href="#contact" class="btn-primary">
                        "상담 신청하기"
                    </a>
                </div>
            </div>
        </section>
    }
}

/// Admissions section (from admissions.html)
#[component]
fn AdmissionsSection() -> impl IntoView {
    view! {
        <section id="admissions" class="py-16 scroll-mt-16">
            <div class="container-section">
                <h2 class="section-title">"Admissions"</h2>
                <blockquote class="section-subtitle">
                    "특별한 수학 ★별을셀"
                </blockquote>

                <div class="mb-8 space-y-2">
                    <p class="text-gray-700 bg-yellow-50 border-l-4 border-yellow-400 pl-4 py-2">
                        "등록은 한달 이상 걸릴 수 있습니다."
                    </p>
                    <p class="text-gray-700">
                        "기다림 이후 등록됩니다. 가장 적은 학생수를 유지하지만, 학생이 있든 없든 대기 후 입회합니다."
                    </p>
                    <p class="text-gray-700 font-medium">"미리 연락주세요."</p>
                </div>

                // 상담절차
                <div class="mb-12">
                    <h3 class="text-2xl font-bold mb-6">"상담절차"</h3>
                    <div class="bg-white rounded-xl p-6 border border-gray-200 mb-6">
                        <p class="text-gray-700 text-lg">
                            <strong>"전화 또는 대면 상담"</strong>
                            " → 등록결정 → 수강료납입"
                        </p>
                    </div>
                    <p class="text-gray-700 mb-4">"아래 내용 문자 남겨 주시면 연락 드립니다. ( 010-5102-0841 별을셀 )"</p>
                    <ul class="space-y-2 text-gray-700 bg-gray-50 rounded-xl p-6">
                        <li>"• 학생이름/학교/학년/성별"</li>
                        <li>"• 최종 수학 등급(내신/모의고사)"</li>
                        <li>"• 1년내 목표 등급 또는 점수"</li>
                    </ul>
                </div>

                // 모집대상
                <div class="mb-12">
                    <h3 class="text-2xl font-bold mb-6">"모집대상"</h3>
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                        <div class="bg-white rounded-xl p-6 border border-gray-200">
                            <h4 class="text-lg font-bold text-brand-600 mb-2">"고등부"</h4>
                            <p class="text-gray-700"><strong>"1,2등급"</strong>" (학교 성적으로 판단)"</p>
                        </div>
                        <div class="bg-white rounded-xl p-6 border border-gray-200">
                            <h4 class="text-lg font-bold text-brand-600 mb-2">"중등부"</h4>
                            <p class="text-gray-700"><strong>"A등급"</strong>" (학교 성적 & 테스트 통과 필요)"</p>
                        </div>
                        <div class="bg-white rounded-xl p-6 border border-gray-200">
                            <h4 class="text-lg font-bold text-brand-600 mb-2">"초등부"</h4>
                            <p class="text-gray-700"><strong>"수학을 좋아하는 학생"</strong>" (테스트 통과 필요)"</p>
                        </div>
                    </div>
                    <ul class="space-y-2 text-gray-700">
                        <li>"• "<strong>"학교 성적으로 레벨 테스트 대체"</strong>", 필요시 추가 테스트"</li>
                        <li>"• 개인이 직접 가르치는 "<strong>"작은 학원"</strong>"입니다. 중대형 학원을 원하시면 다른 곳으로."</li>
                        <li>"• 전화 또는 대면 상담"</li>
                        <li>"• "<strong>"10분 소요"</strong></li>
                        <li>"• 다른 것은 없습니다. "<strong>"결과"</strong>"만 다릅니다."</li>
                    </ul>
                </div>

                // 별을셀수학 모집안내
                <div class="mb-12 card-highlight">
                    <h3 class="text-2xl font-bold text-brand-800 mb-6">"별을셀수학 모집안내"</h3>
                    <ul class="space-y-2 text-brand-700">
                        <li>"• 수학을 좋아하는 학생이면 별을셀에서 환영합니다."</li>
                        <li>"• 고등부는 무리하게 원생을 늘리지 않습니다. 고1,2,3 을 합쳐서 대형학원의 1개 반 구성에도 못 미치도록 관리됩니다."</li>
                        <li>"• 군포지역 일반고 현역 수학 1등급은 극소수 입니다. 4%가 아니라 1%라고 생각하시기 바랍니다."</li>
                        <li>"• "<strong>"원생의 반을 1등급"</strong>"으로 관리하는 수학학원 "<strong>"★별을셀"</strong>"입니다."</li>
                        <li class="font-semibold">"• 예시된 반 이외에도 학생들 모집중입니다."</li>
                    </ul>
                </div>

                // 예비고반 모집
                <div class="mb-12">
                    <h3 class="text-2xl font-bold mb-6">"예비고반 모집"</h3>
                    <div class="overflow-x-auto">
                        <table class="w-full bg-white rounded-xl border border-gray-200">
                            <thead class="bg-gray-50">
                                <tr>
                                    <th class="px-6 py-4 text-left font-semibold text-gray-700 border-b">"구분"</th>
                                    <th class="px-6 py-4 text-left font-semibold text-gray-700 border-b">"내용"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td class="px-6 py-4 border-b text-gray-600">"학년"</td>
                                    <td class="px-6 py-4 border-b text-gray-700">"현재 중3"</td>
                                </tr>
                                <tr>
                                    <td class="px-6 py-4 border-b text-gray-600">"성적 기준"</td>
                                    <td class="px-6 py-4 border-b text-gray-700">"반 1등, 성실한 2~3등"</td>
                                </tr>
                                <tr>
                                    <td class="px-6 py-4 border-b text-gray-600">"모집 인원"</td>
                                    <td class="px-6 py-4 border-b text-gray-700">"소수명"</td>
                                </tr>
                                <tr>
                                    <td class="px-6 py-4 border-b text-gray-600">"모집 기간"</td>
                                    <td class="px-6 py-4 border-b text-gray-700">"상시"</td>
                                </tr>
                                <tr>
                                    <td class="px-6 py-4 text-gray-600">"수업시작"</td>
                                    <td class="px-6 py-4 text-gray-700">"대기후 합류원칙"</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>

                // 중등반 모집
                <div class="mb-12">
                    <h3 class="text-2xl font-bold mb-6">"중등반 모집"</h3>
                    <div class="overflow-x-auto">
                        <table class="w-full bg-white rounded-xl border border-gray-200">
                            <thead class="bg-gray-50">
                                <tr>
                                    <th class="px-6 py-4 text-left font-semibold text-gray-700 border-b">"구분"</th>
                                    <th class="px-6 py-4 text-left font-semibold text-gray-700 border-b">"내용"</th>
                                </tr>
                            </thead>
                            <tbody>
                                <tr>
                                    <td class="px-6 py-4 border-b text-gray-600">"학년"</td>
                                    <td class="px-6 py-4 border-b text-gray-700">"학년 관계없으며 중등 과정을 모두 완료한 학생"</td>
                                </tr>
                                <tr>
                                    <td class="px-6 py-4 border-b text-gray-600">"성적 기준"</td>
                                    <td class="px-6 py-4 border-b text-gray-700">"N/A"</td>
                                </tr>
                                <tr>
                                    <td class="px-6 py-4 border-b text-gray-600">"학습내용"</td>
                                    <td class="px-6 py-4 border-b text-gray-700">"내신병행. 질의된 모든 문항 지원"</td>
                                </tr>
                                <tr>
                                    <td class="px-6 py-4 border-b text-gray-600">"교재"</td>
                                    <td class="px-6 py-4 border-b text-gray-700">"고난도 및 교과외 진행"</td>
                                </tr>
                                <tr>
                                    <td class="px-6 py-4 text-gray-600">"비고"</td>
                                    <td class="px-6 py-4 text-gray-700">"대기후 합류원칙"</td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>

                // 문의 안내 CTA
                <div class="py-12 bg-brand-600 text-white rounded-xl text-center">
                    <h3 class="text-2xl font-bold mb-6">"문의안내"</h3>
                    <p class="mb-4">"아래 내용 문자 남겨 주시면 연락 드립니다. ( 010-5102-0841 별을셀 )"</p>
                    <ul class="space-y-2 mb-8">
                        <li>"• 학생이름/학교/학년/성별"</li>
                        <li>"• 최종 수학 등급(내신/모의고사)"</li>
                        <li>"• 1년내 목표 등급 또는 점수"</li>
                    </ul>
                    <a href="#contact" class="btn-secondary-inverse">"문의 및 입회등록"</a>
                </div>
            </div>
        </section>
    }
}

/// Policies section (from policies.html)
#[component]
fn PoliciesSection() -> impl IntoView {
    view! {
        <section id="policies" class="py-16 bg-gray-50 scroll-mt-16">
            <div class="container-section">
                <h2 class="section-title">"Policies"</h2>
                <blockquote class="section-subtitle">
                    "Stay hungry, Stay foolish"
                </blockquote>

                <div class="mb-8">
                    <h3 class="text-2xl font-bold mb-4">"학원규칙"</h3>
                    <p class="text-gray-500 mb-2">"(학생, 학부모, 학원간의 몇 가지 주요 규칙 게시)"</p>
                    <p class="text-gray-700 mb-4">"주요 규칙만 게시합니다."</p>
                    <p class="text-gray-700">"학원 규칙을 어긴다 하여 잘못된 것은 아닙니다. 각자에게 맞는 곳을 빨리 찾도록 하여 소중한 시간을 낭비하지 않기를 소망합니다."</p>
                </div>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    // 보강규칙
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-bold mb-2">"보강규칙"</h4>
                        <p class="text-brand-600 font-semibold mb-2">"보강 없습니다."</p>
                        <p class="text-gray-700 text-sm">"학원 강의를 녹화하여 그 동영상을 보여주는 방식의 보강이 유행입니다. 그럴 것이면 차라리 해당 과정의 EBS 인강 추천드립니다."</p>
                    </div>

                    // 지각규칙
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-bold mb-2">"지각규칙"</h4>
                        <p class="text-brand-600 font-semibold mb-2">"지각 없습니다."</p>
                        <p class="text-gray-700 text-sm">"늦더라도 10분정도 선에서 최선을 다해서 준비하여 옵니다. 그러나 오면서 뛰거나 하지 말고 사고없이 도착하여야 합니다."</p>
                    </div>

                    // 품행규칙
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-bold mb-3">"품행규칙"</h4>
                        <ul class="space-y-2 text-gray-700 text-sm">
                            <li>"• 흡연, 음주 등 금지합니다."</li>
                            <li>"• 친구와 재미있게 지냅니다."</li>
                        </ul>
                    </div>

                    // 낙서규칙
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-bold mb-3">"낙서규칙"</h4>
                        <p class="text-gray-700 text-sm mb-2">"낙서 허용합니다. 학원의 칠판과 나누어준 연습장에 낙서하세요."</p>
                        <p class="text-gray-700 text-sm">"여러 분의 스트레스가 없도록 일부러 책상도 넓은 것으로, 의자도 편한 것으로 준비하려고 노력합니다."</p>
                    </div>

                    // 학습규칙
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-bold mb-3">"학습규칙"</h4>
                        <div class="space-y-2 text-sm">
                            <div>
                                <span class="font-semibold text-brand-600">"고등학생: "</span>
                                <span class="text-gray-700">"목표 설정하기. 학원 입학시의 내신 등수보다 떨어질 경우 퇴원 권유"</span>
                            </div>
                            <div>
                                <span class="font-semibold text-brand-600">"중학생, 초등생: "</span>
                                <span class="text-gray-700">"즐거울수록 좋습니다. 신나게 노세요. 항상 질문하세요!!"</span>
                            </div>
                        </div>
                    </div>

                    // 기타규칙
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-bold mb-3">"기타규칙"</h4>
                        <ul class="space-y-2 text-gray-700 text-sm">
                            <li>"• "<strong>"휴대폰"</strong>": 책가방안에 보관"</li>
                            <li>"• "<strong>"게임"</strong>": 집에서 게임 안 하기"</li>
                            <li>"• "<strong>"과제물"</strong>": 3번 연속 숙제를 완료하지 않으면 타학원으로 안내"</li>
                        </ul>
                    </div>
                </div>

                <h3 class="text-2xl font-bold mb-4 mt-12">"식쓰기규칙"</h3>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-bold mb-4">"평소"</h4>
                        <ul class="space-y-2 text-gray-700 text-sm">
                            <li>"• 번호를 붙여가며 식쓰기 연습"</li>
                            <li>"• 고정된 방식을 강요하지 않음. 그러나 식쓰기는 권고함"</li>
                            <li>"• 풀이를 쓰라고 하는 것은 머릿속 용량이 부족하기 때문임. 그것을 극복하는 학생은 그 학생의 자유를 더 존중해 줌. 그러나 일반적으로 고등과정 이상은 풀이없이 문제를 풀어내기는 어렵습니다."</li>
                        </ul>
                    </div>

                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-bold mb-4">"시험"</h4>
                        <ul class="space-y-2 text-gray-700 text-sm">
                            <li>"• 소신 껏 풀기"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Validate Korean mobile phone number format
fn is_valid_phone(phone: &str) -> bool {
    let re = regex::Regex::new(r"^01[016789]-?\d{3,4}-?\d{4}$").unwrap();
    re.is_match(phone)
}

/// Contact section with form
#[component]
fn ContactSection() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (phone, set_phone) = signal("010-".to_string());
    let (message, set_message) = signal(String::new());
    let (phone_error, set_phone_error) = signal(Option::<String>::None);

    let submit_action = Action::new(move |_: &()| {
        let name_val = name.get();
        let phone_val = phone.get();
        let message_val = message.get();
        async move { submit_contact(name_val, phone_val, message_val).await }
    });

    let is_pending = submit_action.pending();
    let submission_result = submit_action.value();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        // Validate phone number before submission
        let phone_val = phone.get();
        if !is_valid_phone(&phone_val) {
            set_phone_error.set(Some(
                "올바른 휴대폰 번호를 입력해주세요 (예: 010-1234-5678)".to_string(),
            ));
            return;
        }

        set_phone_error.set(None);
        submit_action.dispatch(());
    };

    let reset_form = move |_| {
        set_name.set(String::new());
        set_phone.set("010-".to_string());
        set_message.set(String::new());
        set_phone_error.set(None);
        submit_action.value().set(None);
    };

    view! {
        <section id="contact" class="py-16 scroll-mt-16">
            <div class="container-section">
                <h2 class="section-title">"Contact"</h2>
                <p class="text-xl text-gray-600 max-w-2xl mb-12">
                    "문의 및 입회등록을 원하시면 메시지를 남겨 주세요. 성실히 답변드리겠습니다."
                </p>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    // Contact Form
                    <div>
                        {move || {
                            if let Some(Ok(())) = submission_result.get() {
                                view! {
                                    <div class="bg-green-50 border border-green-200 rounded-xl p-8 text-center">
                                        <div class="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                                            <CheckIcon class="w-8 h-8 text-green-600"/>
                                        </div>
                                        <h3 class="text-xl font-semibold text-green-800 mb-2">"메시지가 전송되었습니다!"</h3>
                                        <p class="text-green-600">"문의해 주셔서 감사합니다. 빠른 시일 내에 답변드리겠습니다."</p>
                                        <button
                                            type="button"
                                            class="mt-4 text-green-700 underline"
                                            on:click=reset_form
                                        >
                                            "추가 메시지 보내기"
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <form on:submit=on_submit class="space-y-6">
                                        {move || {
                                            submission_result.get().and_then(|r| r.err()).map(|e| {
                                                view! {
                                                    <div class="bg-red-50 border border-red-200 rounded-lg p-4 text-red-700">
                                                        {e.to_string()}
                                                    </div>
                                                }
                                            })
                                        }}
                                        <div>
                                            <label for="name" class="block text-sm font-medium text-gray-700 mb-2">"이름"</label>
                                            <input
                                                type="text"
                                                id="name"
                                                name="name"
                                                required
                                                class="form-input"
                                                placeholder="이름을 입력하세요"
                                                prop:value=move || name.get()
                                                on:input=move |ev| set_name.set(event_target_value(&ev))
                                                disabled=move || is_pending.get()
                                            />
                                        </div>
                                        <div>
                                            <label for="phone" class="block text-sm font-medium text-gray-700 mb-2">"휴대폰 번호"</label>
                                            <input
                                                type="tel"
                                                id="phone"
                                                name="phone"
                                                required
                                                class=move || {
                                                    if phone_error.get().is_some() {
                                                        "form-input border-red-500"
                                                    } else {
                                                        "form-input"
                                                    }
                                                }
                                                placeholder="010-1234-5678"
                                                inputmode="tel"
                                                autocomplete="tel"
                                                maxlength="13"
                                                prop:value=move || phone.get()
                                                on:input=move |ev| {
                                                    set_phone.set(event_target_value(&ev));
                                                    set_phone_error.set(None);
                                                }
                                                disabled=move || is_pending.get()
                                            />
                                            {move || phone_error.get().map(|err| view! {
                                                <p class="mt-1 text-sm text-red-600">{err}</p>
                                            })}
                                        </div>
                                        <div>
                                            <label for="message" class="block text-sm font-medium text-gray-700 mb-2">"내용"</label>
                                            <textarea
                                                id="message"
                                                name="message"
                                                required
                                                rows="5"
                                                class="form-textarea"
                                                placeholder="문의하실 내용을 적어주세요."
                                                prop:value=move || message.get()
                                                on:input=move |ev| set_message.set(event_target_value(&ev))
                                                disabled=move || is_pending.get()
                                            />
                                        </div>
                                        <button
                                            type="submit"
                                            class="btn-primary w-full"
                                            disabled=move || is_pending.get()
                                        >
                                            {move || if is_pending.get() { "전송 중..." } else { "메시지 보내기" }}
                                        </button>
                                    </form>
                                }.into_any()
                            }
                        }}
                    </div>

                    // Contact Info
                    <div class="lg:pl-8">
                        <h3 class="text-2xl font-bold mb-6">"문의 정보"</h3>
                        <div class="space-y-6">
                            <div class="flex gap-4">
                                <div class="flex-shrink-0 w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center">
                                    <LocationIcon class="w-5 h-5 text-brand-600"/>
                                </div>
                                <div>
                                    <h4 class="font-semibold">"주소"</h4>
                                    <p class="text-gray-600">"경기도 군포시 번영로 489 중앙타워 2층 ★별을셀수학 (지번: 산본동 1142-7)"</p>
                                </div>
                            </div>
                            <div class="flex gap-4">
                                <div class="flex-shrink-0 w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center">
                                    <EmailIcon class="w-5 h-5 text-brand-600"/>
                                </div>
                                <div>
                                    <h4 class="font-semibold">"웹사이트"</h4>
                                    <p class="text-gray-600">"https://starrystarry.kr"</p>
                                </div>
                            </div>
                            <div class="flex gap-4">
                                <div class="flex-shrink-0 w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center">
                                    <PhoneIcon class="w-5 h-5 text-brand-600"/>
                                </div>
                                <div>
                                    <h4 class="font-semibold">"전화"</h4>
                                    <p class="text-gray-600">"010-5102-0841"</p>
                                </div>
                            </div>
                            <div class="flex gap-4">
                                <div class="flex-shrink-0 w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center">
                                    <ChatIcon class="w-5 h-5 text-brand-600"/>
                                </div>
                                <div>
                                    <h4 class="font-semibold">"카카오"</h4>
                                    <p class="text-gray-600">"별을셀 검색"</p>
                                </div>
                            </div>
                            <div class="flex gap-4">
                                <div class="flex-shrink-0 w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center">
                                    <ClockIcon class="w-5 h-5 text-brand-600"/>
                                </div>
                                <div>
                                    <h4 class="font-semibold">"운영 안내"</h4>
                                    <p class="text-gray-600">"상담 및 수업 시간은 문의 시 안내드립니다"</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
