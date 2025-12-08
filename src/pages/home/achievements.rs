use leptos::prelude::*;

/// Achievements section showing exam results
#[component]
pub fn AchievementsSection() -> impl IntoView {
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
