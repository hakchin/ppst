use leptos::prelude::*;

use crate::constants::contact;

/// Mission section from legacy mission.html
#[component]
pub fn MissionSection() -> impl IntoView {
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
                        <li><strong>"author"</strong>": " {contact::NAME} ", " {contact::PHONE}</li>
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
