use leptos::prelude::*;

/// Programs section (from programs_content.html)
#[component]
pub fn ProgramsSection() -> impl IntoView {
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
