use leptos::prelude::*;

/// Mission page component - showcases the academy's philosophy and achievements
#[component]
pub fn MissionPage() -> impl IntoView {
    view! {
        <div>
            <MissionHeader/>
            <MissionContent/>
            <AchievementsSection/>
            <StarsMessageSection/>
            <TeachingPhilosophySection/>
        </div>
    }
}

/// Page header
#[component]
fn MissionHeader() -> impl IntoView {
    view! {
        <section class="bg-gray-50 py-16">
            <div class="container-section">
                <h1 class="text-4xl md:text-5xl font-bold mb-4">"Mission"</h1>
                <h2 class="text-2xl font-semibold">
                    "수학을 가르칩니다"
                    <sup class="text-sm align-super">"®"</sup>
                </h2>
            </div>
        </section>
    }
}

/// Main mission content section
#[component]
fn MissionContent() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
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
                    <p>"과거부터 지금까지 언제나 그랬드이 확실한 미래는 알 수가 없습니다. 미래의 흐름에 벌써 너무 걱정하기보다는 눈 앞의 단순한 입시나 조금 더 나아가서는 순순하게 사유하는 것이 좋고, 논리적 해석을 해 보고 싶은 학생들이 있다면 함께 그 길을 같이 가고 싶습니다."</p>
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
                <h2 class="text-2xl font-bold mb-8">"성과"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    // 2022 Achievement
                    <div class="bg-white rounded-xl p-8 shadow-sm border border-gray-200">
                        <h3 class="text-xl font-bold text-brand-600 mb-4">"2022년 11월 수능수학포함 2년 연속 전원 1등급"</h3>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 재원생 전원 수능수학 1등급 성취"</li>
                            <li>"• 진솔한 소수 학생과 함께 하고자 합니다."</li>
                        </ul>
                    </div>

                    // 2021 Achievement
                    <div class="bg-white rounded-xl p-8 shadow-sm border border-gray-200">
                        <h3 class="text-xl font-bold text-brand-600 mb-4">"2021년 11월 수능수학 전원 1등급"</h3>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 충분하고 넉넉한 점수로 재원생 전원 1등급 성취 (졸업수강인원 4명)"</li>
                            <li>"• 수리논술 지원자 전원 합격"</li>
                            <li>"• 2022년 11월 수능수학 또한 1등급 도전해 봅니다"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Stars message section
#[component]
fn StarsMessageSection() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
                <div class="bg-brand-50 rounded-xl p-8 border border-brand-100">
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
                        <ul class="space-y-3 text-gray-700">
                            <li>"• "<strong>"원생의 반 이상이 수학 1등급"</strong>" 유지하는 유일한 곳 (간혹 안 되기도 합니다만 성취도는 월등합니다.)"</li>
                            <li>"• 별을셀이 자랑하는 "<strong>"1등급 정시반"</strong>" 운영"</li>
                            <li>"• 미사여구는 없습니다. "<strong>"올바른 과정과 결과"</strong>"입니다. 언제든 문의 주세요."</li>
                            <li>"• 타학원과 달리 ★별을셀은 "<strong>"단순히 몇 명이다"</strong>"가 아닌 "<strong>"정확히 몇 명중의 몇 명이 1등급"</strong>"인지를 말합니다."</li>
                        </ul>
                    </div>

                    // 수업방향
                    <div>
                        <h3 class="text-2xl font-bold mb-6">"수업방향"</h3>
                        <ul class="space-y-3 text-gray-700">
                            <li>"• 수학이 왜 자유로운 상상의 학문인지 보여 드립니다."</li>
                            <li>"• 진지한 수학적 접근을 끊임없이 시도합니다."</li>
                            <li>"• 수학은 자유의 학문입니다."</li>
                            <li>"• 동기부여 없는 강제적 학습을 지양합니다."</li>
                        </ul>
                        <div class="mt-8 bg-gray-50 rounded-xl p-6">
                            <h4 class="text-lg font-semibold mb-3">"특별한 수학 ★별을셀입니다"</h4>
                            <ul class="space-y-2 text-gray-700 text-sm">
                                <li>"• 작은 학원입니다. 학생 한 명 한 명이 존중되는 곳입니다."</li>
                                <li>"• Contact : " <a href="/contact">"문의/입회"</a></li>
                                <li>"• 홈페이지나 연락처는 특별하지 않습니다. "<strong>"수학이 특별한 곳"</strong>", ★별을셀입니다."</li>
                                <li>"• 전화 : 010-5102-0841"</li>
                                <li>"• 카카오 : " <a href="https://pf.kakao.com/_MxbVfT" target="_blank" rel="noopener noreferrer">"별을셀"</a> " 검색"</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
