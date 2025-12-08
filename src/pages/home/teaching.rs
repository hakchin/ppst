use leptos::prelude::*;

use crate::constants::contact;

/// Teaching philosophy section
#[component]
pub fn TeachingPhilosophySection() -> impl IntoView {
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
                                <li>"• 전화 : " {contact::PHONE}</li>
                                <li>"• 카카오 : " <a href="https://pf.kakao.com/_MxbVfT" target="_blank" rel="noopener noreferrer" class="text-brand-600 hover:underline">"별을셀"</a> " 검색"</li>
                            </ul>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
