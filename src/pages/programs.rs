use leptos::prelude::*;
use leptos_router::components::A;

/// Programs page component - matches legacy programs_content.html
#[component]
pub fn ProgramsPage() -> impl IntoView {
    view! {
        <div>
            <ProgramsHeader/>
            <HighSchoolSection/>
            <MiddleSchoolSection/>
            <MiddleSchoolEssaySection/>
            <ElementarySection/>
            <NoticesSection/>
            <ClassroomsSection/>
            <CtaSection/>
        </div>
    }
}

/// Page header with lead quote
#[component]
fn ProgramsHeader() -> impl IntoView {
    view! {
        <section class="bg-white py-16">
            <div class="container-section">
                <h1 class="text-4xl md:text-5xl font-bold mb-4">"Our Programs"</h1>
                <blockquote class="text-xl text-gray-600 max-w-2xl border-l-4 border-brand-600 pl-4 mt-6">
                    <p>"왕도는 없습니다."</p>
                    <p>"꼴찌부터 1등까지 아이들이 행복하기를 바랍니다."</p>
                </blockquote>
            </div>
        </section>
    }
}

/// High school programs section
#[component]
fn HighSchoolSection() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-8 flex items-center gap-3">
                    <span class="w-3 h-3 rounded-full bg-brand-600"></span>
                    "고등부"
                </h2>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                    // 내신반
                    <div class="card p-6">
                        <h3 class="text-xl font-semibold text-brand-600 mb-4">"내신반"</h3>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 원생의 반 이상이 수학 1등급 유지하는 유일한 곳 (철저한 내신관리, 수준관리)"</li>
                            <li>"• 고등부 내신반의 놀라운 성취를 꼭 확인해 보세요"</li>
                        </ul>
                    </div>

                    // 정시반
                    <div class="card p-6">
                        <h3 class="text-xl font-semibold text-brand-600 mb-4">"정시반"</h3>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 수능 1등급 유지 또는 100점 목표"</li>
                            <li>"• 고난도 문항(14,15,21,22,28,29,30) 이해와 실전 연습"</li>
                            <li>"• 고3 및 독학재수 학원생 대상"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Middle school programs section
#[component]
fn MiddleSchoolSection() -> impl IntoView {
    view! {
        <section class="py-16 bg-white">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-8 flex items-center gap-3">
                    <span class="w-3 h-3 rounded-full bg-brand-600"></span>
                    "중등부"
                </h2>

                <div class="space-y-4">
                    <ProgramItem
                        title="정규반"
                        description="학년별 과정을 충실히 하며, 개인별 선행 진행합니다."
                    />
                    <ProgramItem
                        title="실력정석반"
                        description="무학년제. 하나의 교재만 집중하되 교재안의 단 한 문제도 놓치지 않습니다. (주 1회반, 주 2회반)"
                    />
                    <div class="bg-white rounded-lg p-4 border border-gray-200">
                        <span class="font-semibold text-brand-600">"KMO"</span>
                        <span class="text-gray-700">" : KMO/IMO 수학 경시 특강. 월4회 수업. 2년 과정. (정수론, 기하론, 대수론, 조합론)"</span>
                    </div>
                    <ProgramItem
                        title="정수론"
                        description="KMO 준비과목. 교과 과정 아니지만 수학 기초를 튼튼히 쌓고자 하는 학생에게 적합. (주 1회반)"
                    />
                    <ProgramItem
                        title="기하론"
                        description="KMO 준비과목. 교과 과정 아니지만 수학 기초를 튼튼히 쌓고자 하는 학생에게 적합. (주 1회반)"
                    />
                </div>
            </div>
        </section>
    }
}

/// Middle school essay section
#[component]
fn MiddleSchoolEssaySection() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-8 flex items-center gap-3">
                    <span class="w-3 h-3 rounded-full bg-brand-600"></span>
                    "중등수리논술반"
                </h2>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-8">
                    <div>
                        <h3 class="text-lg font-semibold mb-4">"학습대상"</h3>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• "<strong>"학년 관계없으며"</strong>" 중등 과정을 모두 완료한 학생"</li>
                            <li>"• 아직 실력은 되지 않으나 열의 있는 학생은 별을셀에서 준비과정 거친 후에 합류합니다."</li>
                            <li>"• "<strong>"고등부는 3학년 때 수리논술 따로 진행"</strong>"합니다."</li>
                        </ul>
                    </div>

                    <div>
                        <h3 class="text-lg font-semibold mb-4">"학습내용"</h3>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 기존 "<strong>"실력정석반"</strong>"과 "<strong>"평면기하반"</strong>"을 합쳤습니다."</li>
                            <li>"• 실력정석 및 평면기하를 소재로 수리논술 연습"</li>
                            <li>"• 내신병행"</li>
                        </ul>
                    </div>
                </div>

                <div class="mt-8 bg-brand-50 rounded-xl p-6 border border-brand-100">
                    <p class="text-gray-700 mb-4">"학생들이 연습문제까지 다 풀 수 있도록 이끌어 갑니다."</p>
                    <p class="text-gray-700 mb-4">"대형학원 등에서 실력정석 교재 2~3번 완주하고 경시대회 준비한 자재분을 집에서 직접 테스트 하여 보세요. 연습문제 실력파트 5문제만 주어보면 0점이 수두룩 합니다. 50점 나왔다면 훌륭하며 기존 해당 학원 열심히 다니시면 됩니다."</p>
                    <p class="text-gray-700">"점수에 연연하지 않습니다. 수학적 사고를 기본으로 고등부에 올라가서 내신 1등급 유지하도록 지원합니다."</p>
                </div>
            </div>
        </section>
    }
}

/// Elementary school section
#[component]
fn ElementarySection() -> impl IntoView {
    view! {
        <section class="py-16 bg-white">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-8 flex items-center gap-3">
                    <span class="w-3 h-3 rounded-full bg-brand-600"></span>
                    "초등부"
                </h2>

                <div class="space-y-4">
                    <ProgramItem
                        title="정규반"
                        description="학년별 과정을 충실히 하며, 개인별 선행 진행합니다."
                    />
                    <ProgramItem
                        title="실력정석반"
                        description="무학년제. 하나의 교재만 집중하되 교재안의 단 한 문제도 놓치지 않습니다. (주 1회반, 주 2회반)"
                    />
                    <div class="bg-white rounded-lg p-4 border border-gray-200 text-gray-700">
                        "Class별 4명 이하 제한, 세밀한 지도합니다. 긴 관점으로 확실한 실력향상을 원하는 학생만 받습니다."
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Notices section for middle 3 students
#[component]
fn NoticesSection() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-8 flex items-center gap-3">
                    <span class="w-3 h-3 rounded-full bg-brand-600"></span>
                    "중3 유의사항"
                </h2>

                <ul class="space-y-2 text-gray-700">
                    <li>"• 11월부터 고1 예비반 과정 진행합니다."</li>
                    <li>"• 기존 중등부 학생은 자동으로 11월부터 고1 예비반 전환됩니다."</li>
                </ul>
            </div>
        </section>
    }
}

/// Classrooms section
#[component]
fn ClassroomsSection() -> impl IntoView {
    view! {
        <section class="py-16 bg-white">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-8 flex items-center gap-3">
                    <span class="w-3 h-3 rounded-full bg-brand-600"></span>
                    "교실"
                </h2>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <ClassroomCard
                        number="01"
                        name="Shooting star"
                        korean="별똥별"
                        level="M"
                    />
                    <ClassroomCard
                        number="02"
                        name="Polaris"
                        korean="북극성"
                        level="E, H"
                    />
                    <ClassroomCard
                        number="03"
                        name="Milkyway"
                        korean="은하수"
                        level="M"
                    />
                </div>
            </div>
        </section>
    }
}

/// CTA section
#[component]
fn CtaSection() -> impl IntoView {
    view! {
        <section class="py-16 bg-brand-600 text-white">
            <div class="container-section text-center">
                <h2 class="text-3xl font-bold mb-4">"어떤 프로그램이 맞을지 고민되시나요?"</h2>
                <p class="text-brand-100 mb-8 max-w-2xl mx-auto">
                    "입시 목표와 상황에 맞는 최적의 방향을 함께 찾겠습니다."
                </p>
                <A href="/contact" attr:class="btn-primary bg-white text-brand-600 hover:bg-gray-100">
                    "상담 신청하기"
                </A>
            </div>
        </section>
    }
}

/// Reusable program item component
#[component]
fn ProgramItem(title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white rounded-lg p-4 border border-gray-200">
            <span class="font-semibold text-brand-600">{title}</span>
            <span class="text-gray-700">" : "{description}</span>
        </div>
    }
}

/// Classroom card component
#[component]
fn ClassroomCard(
    number: &'static str,
    name: &'static str,
    korean: &'static str,
    level: &'static str,
) -> impl IntoView {
    view! {
        <div class="bg-white rounded-xl p-6 border border-gray-200 text-center">
            <span class="text-sm text-gray-400">{number}"| "</span>
            <span class="font-semibold">{name}</span>
            <span class="text-gray-600">" ("{korean}")"</span>
            <span class="ml-2 text-xs bg-brand-100 text-brand-700 px-2 py-1 rounded">{level}</span>
        </div>
    }
}
