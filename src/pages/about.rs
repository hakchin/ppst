use leptos::prelude::*;

/// About page component - showcases the academy's mission and philosophy
#[component]
pub fn AboutPage() -> impl IntoView {
    view! {
        <div>
            <AboutHeader/>
            <PhilosophySection/>
            <ValuesSection/>
            <ApproachSection/>
        </div>
    }
}

/// Page header
#[component]
fn AboutHeader() -> impl IntoView {
    view! {
        <section class="bg-gray-50 py-16">
            <div class="container-section">
                <h1 class="text-4xl md:text-5xl font-bold mb-4">"소개"</h1>
                <p class="text-xl text-gray-600 max-w-2xl">
                    "별을셀은 소수의 작은 학습장을 고집하며 정성과 진심으로 수학을 가르칩니다."
                </p>
            </div>
        </section>
    }
}

/// Philosophy section
#[component]
fn PhilosophySection() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12 items-center">
                    <div>
                        <h2 class="text-3xl font-bold mb-6">
                            "수학을 가르칩니다"
                            <sup class="text-sm align-super">"®"</sup>
                        </h2>
                        <div class="space-y-4 text-gray-700">
                            <p>"수학이 재미있는 곳 "<strong>"★별을셀"</strong>"입니다. 반짝이는 밤하늘의 별처럼 여러 분도 반짝이는 별입니다."</p>
                            <p>"개인적 수학 아이디어의 많은 부분에 대하여 Georg Cantor와 Kurt Gödel에 대하여 감사합니다."</p>
                            <p>"세속의 수학(입시수학, 공업수학, 미적분, 일반 문제풀이,...)은 이들의 것과 많이 비슷하지는 않습니다. 세속의 수학에도 많은 감사를 드립니다."</p>
                            <p>"여러 분과 함께 세속의 수학을 합니다. 동시에 개인으로서는 상상과 논리의 동산에서 자신만의 수학적 사고를 하고자 합니다."</p>
                        </div>
                    </div>
                    <div class="bg-brand-100 rounded-2xl p-8 lg:p-12">
                        <blockquote class="text-xl text-brand-800 italic">
                            "수학은 자유의 학문입니다. 동기부여 없는 강제적 학습을 지양합니다."
                        </blockquote>
                        <cite class="text-brand-600 mt-4 block">"— ★별을셀수학"</cite>
                    </div>
                </div>
            </div>
        </section>
    }
}

/// Core values section
#[component]
fn ValuesSection() -> impl IntoView {
    view! {
        <section class="py-16 bg-gray-50">
            <div class="container-section">
                <h2 class="text-3xl font-bold text-center mb-12">"핵심 가치"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                    <ValueCard
                        title="소수의 작은 학습장"
                        description="미사여구와 그럴싸한 자료와 언어로 학생과 학부모님들을 현혹하지 않습니다."
                    />
                    <ValueCard
                        title="정성과 진심"
                        description="쏟아지는 광고처럼 누가 어떤 책을 썼고, 어떤 특별한 문제집을 풀어야 된다는 말씀은 드리지 않습니다."
                    />
                    <ValueCard
                        title="과정에 최선"
                        description="결과보다는 과정에 최선을 다하고, 어렵고 힘듦이 있음을 주저하지 않으며 나아갑니다."
                    />
                    <ValueCard
                        title="의지의 학생님과 함께"
                        description="여러 분은 과정에 최선을 다하고 별을셀은 결과를 드리고자 노력합니다."
                    />
                </div>
            </div>
        </section>
    }
}

/// Teaching approach section
#[component]
fn ApproachSection() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    <div>
                        <h2 class="text-2xl font-bold mb-6">"수업방향"</h2>
                        <ul class="space-y-3 text-gray-700">
                            <li>"• 수학이 왜 자유로운 상상의 학문인지 보여 드립니다."</li>
                            <li>"• 진지한 수학적 접근을 끊임없이 시도합니다."</li>
                            <li>"• 수학은 자유의 학문입니다."</li>
                            <li>"• 동기부여 없는 강제적 학습을 지양합니다."</li>
                        </ul>
                    </div>

                    <div>
                        <h2 class="text-2xl font-bold mb-6">"특별한 수학 ★별을셀입니다"</h2>
                        <ul class="space-y-3 text-gray-700">
                            <li>"• 작은 학원입니다. 학생 한 명 한 명이 존중되는 곳입니다."</li>
                            <li>"• 홈페이지나 연락처는 특별하지 않습니다. "<strong>"수학이 특별한 곳"</strong>", ★별을셀입니다."</li>
                            <li>"• 웹사이트 : https://starrystarry.kr"</li>
                            <li>"• 전화 : 010-5102-0841"</li>
                            <li>"• 카카오 : 별을셀 검색"</li>
                        </ul>
                    </div>
                </div>

                <div class="mt-12 bg-brand-50 rounded-xl p-8 border border-brand-100">
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

/// Value card component
#[component]
fn ValueCard(title: &'static str, description: &'static str) -> impl IntoView {
    view! {
        <div class="bg-white rounded-xl p-6 shadow-sm border border-gray-200">
            <h3 class="text-lg font-semibold mb-2">{title}</h3>
            <p class="text-gray-600 text-sm">{description}</p>
        </div>
    }
}
