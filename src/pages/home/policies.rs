use leptos::prelude::*;

/// Policies section (from policies.html)
#[component]
pub fn PoliciesSection() -> impl IntoView {
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
