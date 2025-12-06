use leptos::prelude::*;

/// Policies page component - matches legacy policies.html
#[component]
pub fn PoliciesPage() -> impl IntoView {
    view! {
        <div>
            <PoliciesHeader/>
            <RulesIntroSection/>
            <MakeupClassSection/>
            <LatenessSection/>
            <ConductSection/>
            <DoodleSection/>
            <StudyRulesSection/>
            <WritingRulesSection/>
            <OtherRulesSection/>
        </div>
    }
}

/// Page header with lead quote
#[component]
fn PoliciesHeader() -> impl IntoView {
    view! {
        <section class="bg-gray-50 py-16">
            <div class="container-section">
                <h1 class="text-4xl md:text-5xl font-bold mb-4">"Policies"</h1>
                <blockquote class="text-xl text-gray-600 italic mt-6 border-l-4 border-brand-600 pl-4">
                    "Stay hungry, Stay foolish"
                </blockquote>
            </div>
        </section>
    }
}

/// Introduction to rules
#[component]
fn RulesIntroSection() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-6">"학원규칙"</h2>
                <p class="text-gray-500 mb-2">"(학생, 학부모, 학원간의 몇 가지 주요 규칙 게시)"</p>
                <p class="text-gray-700 mb-4">"주요 규칙만 게시합니다."</p>
                <p class="text-gray-700">
                    "학원 규칙을 어긴다 하여 잘못된 것은 아닙니다. 각자에게 맞는 곳을 빨리 찾도록 하여 소중한 시간을 낭비하지 않기를 소망합니다."
                </p>
            </div>
        </section>
    }
}

/// Makeup class rules
#[component]
fn MakeupClassSection() -> impl IntoView {
    view! {
        <section class="py-12 bg-gray-50">
            <div class="container-section">
                <RuleCard
                    title="보강규칙"
                    highlight="보강 없습니다."
                    description="학원 강의를 녹화하여 그 동영상을 보여주는 방식의 보강이 유행입니다. 그럴 것이면 차라리 해당 과정의 EBS 인강 추천드립니다. 필요시 교재까지 무료로 제공해 드립니다. 재원생 누구든 편하게 요청하세요."
                />
            </div>
        </section>
    }
}

/// Lateness rules
#[component]
fn LatenessSection() -> impl IntoView {
    view! {
        <section class="py-12">
            <div class="container-section">
                <RuleCard
                    title="지각규칙"
                    highlight="지각 없습니다."
                    description="늦더라도 10분정도 선에서 최선을 다해서 준비하여 옵니다. 그러나 오면서 뛰거나 하지 말고 사고없이 도착하여야 합니다."
                />
            </div>
        </section>
    }
}

/// Conduct rules
#[component]
fn ConductSection() -> impl IntoView {
    view! {
        <section class="py-12 bg-gray-50">
            <div class="container-section">
                <h3 class="text-xl font-bold mb-4">"품행규칙"</h3>
                <ul class="space-y-2 text-gray-700">
                    <li>"• 흡연, 음주 등 금지합니다."</li>
                    <li>"• 친구와 재미있게 지냅니다."</li>
                </ul>
            </div>
        </section>
    }
}

/// Doodle rules
#[component]
fn DoodleSection() -> impl IntoView {
    view! {
        <section class="py-12">
            <div class="container-section">
                <h3 class="text-xl font-bold mb-4">"낙서규칙"</h3>
                <p class="text-gray-700 mb-4">
                    "낙서 허용합니다. 학원의 칠판과 나누어준 연습장에 낙서하세요. 연습장을 충분히 제공합니다."
                </p>
                <ul class="space-y-2 text-gray-700">
                    <li>"• 칠판"</li>
                    <li>"• 연습장"</li>
                    <li>"• 여러 분의 스트레스가 없도록 일부러 책상도 넓은 것으로, 의자도 편한 것으로 준비하려고 노력합니다."</li>
                </ul>
            </div>
        </section>
    }
}

/// Study rules section
#[component]
fn StudyRulesSection() -> impl IntoView {
    view! {
        <section class="py-12 bg-gray-50">
            <div class="container-section">
                <h3 class="text-xl font-bold mb-6">"학습규칙"</h3>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-semibold text-brand-600 mb-4">"고등학생"</h4>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 목표 설정하기"</li>
                            <li>"• 학원 입학시의 내신 등수보다 떨어질 경우(2번째 시험부터 적용)에는 상호간에 정만 남기고 퇴원 권유"</li>
                        </ul>
                    </div>

                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-semibold text-brand-600 mb-4">"중학생, 초등생"</h4>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 즐거울수록 좋습니다. 신나게 노세요."</li>
                            <li>"• 항상 질문하세요 !!"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Writing rules section
#[component]
fn WritingRulesSection() -> impl IntoView {
    view! {
        <section class="py-12">
            <div class="container-section">
                <h3 class="text-xl font-bold mb-6">"식쓰기규칙"</h3>

                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    <div>
                        <h4 class="text-lg font-semibold mb-4">"평소"</h4>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 번호를 붙여가며 식쓰기 연습"</li>
                            <li>"• 고정된 방식을 강요하지 않음. 그러나 식쓰기는 권고함"</li>
                            <li>"• 풀이를 쓰라고 하는 것은 머릿속 용량이 부족하기 때문임. 그것을 극복하는 학생은 그 학생의 자유를 더 존중해 줌. 그러나 일반적으로 고등과정 이상은 풀이없이 문제를 풀어내기는 어렵습니다."</li>
                        </ul>
                    </div>

                    <div>
                        <h4 class="text-lg font-semibold mb-4">"시험"</h4>
                        <ul class="space-y-2 text-gray-700">
                            <li>"• 소신 껏 풀기"</li>
                        </ul>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Other rules section
#[component]
fn OtherRulesSection() -> impl IntoView {
    view! {
        <section class="py-12 bg-gray-50">
            <div class="container-section">
                <h3 class="text-xl font-bold mb-6">"기타규칙"</h3>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-6">
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-semibold text-brand-600 mb-3">"휴대폰"</h4>
                        <p class="text-gray-700">"• 책가방안에 보관"</p>
                    </div>

                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-semibold text-brand-600 mb-3">"게임"</h4>
                        <p class="text-gray-700">"• 집에서 게임 안 하기"</p>
                    </div>

                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h4 class="text-lg font-semibold text-brand-600 mb-3">"과제물"</h4>
                        <p class="text-gray-700">"• 3번 연속 숙제를 완료하지 않으면 타학원으로 안내"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Reusable rule card component
#[component]
fn RuleCard(
    title: &'static str,
    highlight: &'static str,
    description: &'static str,
) -> impl IntoView {
    view! {
        <div>
            <h3 class="text-xl font-bold mb-4">{title}</h3>
            <p class="text-brand-600 font-semibold mb-2">{highlight}</p>
            <p class="text-gray-700">{description}</p>
        </div>
    }
}
