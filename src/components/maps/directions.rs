use leptos::prelude::*;

use crate::constants::contact;

/// Shared directions/location section component
/// Used in both HomePage and ContactPage
#[component]
pub fn DirectionsSection() -> impl IntoView {
    view! {
        <section id="directions" class="py-16 bg-gray-50 scroll-mt-16">
            <div class="container-section">
                <h2 class="section-title">"오시는 길"</h2>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
                    <div class="space-y-6">
                        <div>
                            <h4 class="font-semibold text-lg mb-2">"주소"</h4>
                            <p class="text-gray-700">{contact::ADDRESS_FULL}</p>
                            <p class="text-gray-500 text-sm">"지번 : 경기도 군포시 산본동 1142-7"</p>
                        </div>
                        <div>
                            <h4 class="font-semibold text-lg mb-2">"지하철"</h4>
                            <p class="text-gray-700">"4호선 산본역에서 하차하여 3번출구로 나오시면 ..."</p>
                        </div>
                        <div>
                            <h4 class="font-semibold text-lg mb-2">"버스"</h4>
                            <p class="text-gray-700">"산본역 또는 6단지 세종에서 하차"</p>
                        </div>
                        <div>
                            <h4 class="font-semibold text-lg mb-2">"자동차"</h4>
                            <p class="text-gray-700">"롯데피트인 맞은편 건물입니다."</p>
                        </div>
                    </div>

                    <div class="card-elevated p-8">
                        <h3 class="text-xl font-bold mb-4">"열심히 공부할 준비가 되셨나요?"</h3>
                        <ul class="space-y-3 text-gray-700">
                            <li>"• 수학은 어렵습니다."</li>
                            <li>"• 수학은 해 볼만 합니다."</li>
                            <li>"• 수학은 어렵지만 해 볼만 합니다."</li>
                        </ul>
                        <p class="mt-6 text-xl font-semibold text-brand-600">"별을셀이 돕겠습니다"</p>
                    </div>
                </div>
            </div>
        </section>
    }
}
