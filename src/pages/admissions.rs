use leptos::prelude::*;
use leptos_router::components::A;

/// Admissions page component - matches legacy admissions.html
#[component]
pub fn AdmissionsPage() -> impl IntoView {
    view! {
        <div>
            <AdmissionsHeader/>
            <ConsultationSection/>
            <TargetSection/>
            <RecruitmentInfoSection/>
            <PreHighSchoolSection/>
            <MiddleSchoolClassSection/>
            <InquirySection/>
        </div>
    }
}

/// Page header with lead quote
#[component]
fn AdmissionsHeader() -> impl IntoView {
    view! {
        <section class="bg-gray-50 py-16">
            <div class="container-section">
                <h1 class="text-4xl md:text-5xl font-bold mb-4">"Admissions"</h1>
                <blockquote class="text-xl text-gray-600 font-medium mt-6 border-l-4 border-brand-600 pl-4">
                    "특별한 수학 ★별을셀"
                </blockquote>

                <div class="mt-8 space-y-2">
                    <p class="text-gray-700 bg-yellow-50 border-l-4 border-yellow-400 pl-4 py-2">
                        "등록은 한달 이상 걸릴 수 있습니다."
                    </p>
                    <p class="text-gray-700">
                        "기다림 이후 등록됩니다. 가장 적은 학생수를 유지하지만, 학생이 있든 없든 대기 후 입회합니다."
                    </p>
                    <p class="text-gray-700 font-medium">
                        "미리 연락주세요."
                    </p>
                </div>
            </div>
        </section>
    }
}

/// Consultation process section
#[component]
fn ConsultationSection() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-6">"상담절차"</h2>
                <div class="bg-white rounded-xl p-6 border border-gray-200">
                    <p class="text-gray-700 text-lg">
                        <strong>"전화 또는 대면 상담"</strong>
                        " → 등록결정 → 수강료납입"
                    </p>
                    <p class="text-sm text-gray-500 mt-4">
                        "네이버 서비스 정책 변경으로 위 링크 접속이 제한될 수 있습니다. 전화로 먼저 문의 주세요."
                    </p>
                </div>

                <div class="mt-8">
                    <h3 class="text-xl font-semibold mb-4">"또는"</h3>
                    <p class="text-gray-700 mb-4">
                        "아래 내용 문자 남겨 주시면 연락 드립니다. ( 010-5102-0841 별을셀 )"
                    </p>
                    <ul class="space-y-2 text-gray-700 bg-gray-50 rounded-lg p-4">
                        <li>"• 학생이름/학교/학년/성별"</li>
                        <li>"• 최종 수학 등급(내신/모의고사)"</li>
                        <li>"• 1년내 목표 등급 또는 점수"</li>
                    </ul>
                </div>
            </div>
        </section>
    }
}

/// Target students section
#[component]
fn TargetSection() -> impl IntoView {
    view! {
        <section class="py-16 bg-gray-50">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-6">"모집대상"</h2>

                <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h3 class="text-lg font-semibold text-brand-600 mb-2">"고등부"</h3>
                        <p class="text-gray-700"><strong>"1,2등급"</strong>" (학교 성적으로 판단)"</p>
                    </div>
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h3 class="text-lg font-semibold text-brand-600 mb-2">"중등부"</h3>
                        <p class="text-gray-700"><strong>"A등급"</strong>" (학교 성적 & 테스트 통과 필요)"</p>
                    </div>
                    <div class="bg-white rounded-xl p-6 border border-gray-200">
                        <h3 class="text-lg font-semibold text-brand-600 mb-2">"초등부"</h3>
                        <p class="text-gray-700"><strong>"수학을 좋아하는 학생"</strong>" (테스트 통과 필요)"</p>
                    </div>
                </div>

                <ul class="space-y-3 text-gray-700">
                    <li>"• "<strong>"학교 성적으로 레벨 테스트 대체"</strong>", 필요시 추가 테스트"</li>
                    <li>"• 개인이 직접 가르치는 "<strong>"작은 학원"</strong>"입니다. 중대형 학원을 원하시면 다른 곳으로."</li>
                    <li>"• 전화 또는 대면 상담"</li>
                    <li>"• "<strong>"10분 소요"</strong></li>
                    <li>"• 다른 것은 없습니다. "<strong>"결과"</strong>"만 다릅니다."</li>
                </ul>
            </div>
        </section>
    }
}

/// Recruitment info section
#[component]
fn RecruitmentInfoSection() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-6">"별을셀수학 모집안내"</h2>

                <ul class="space-y-4 text-gray-700">
                    <li>"• 수학을 좋아하는 학생이면 별을셀에서 환영합니다."</li>
                    <li>"• 고등부는 무리하게 원생을 늘리지 않습니다. 고1,2,3 을 합쳐서 대형학원의 1개 반 구성에도 못 미치도록 관리됩니다."</li>
                    <li>"• 군포지역 일반고 현역 수학 1등급은 극소수 입니다. 4%가 아니라 1%라고 생각하시기 바랍니다."</li>
                    <li>"• "<strong>"원생의 반을 1등급"</strong>"으로 관리하는 수학학원 "<strong>"★별을셀"</strong>"입니다."</li>
                    <li class="text-brand-600 font-semibold">"• 예시된 반 이외에도 학생들 모집중입니다."</li>
                </ul>
            </div>
        </section>
    }
}

/// Pre-high school class section with table
#[component]
fn PreHighSchoolSection() -> impl IntoView {
    view! {
        <section class="py-16 bg-gray-50">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-6">"예비고반 모집"</h2>

                <div class="overflow-x-auto">
                    <table class="w-full bg-white rounded-xl border border-gray-200">
                        <thead class="bg-gray-50">
                            <tr>
                                <th class="px-6 py-4 text-left font-semibold text-gray-700 border-b">"구분"</th>
                                <th class="px-6 py-4 text-left font-semibold text-gray-700 border-b">"내용"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="px-6 py-4 border-b text-gray-600">"학년"</td>
                                <td class="px-6 py-4 border-b text-gray-700">"현재 중3"</td>
                            </tr>
                            <tr>
                                <td class="px-6 py-4 border-b text-gray-600">"성적 기준"</td>
                                <td class="px-6 py-4 border-b text-gray-700">"반 1등, 성실한 2~3등"</td>
                            </tr>
                            <tr>
                                <td class="px-6 py-4 border-b text-gray-600">"모집 인원"</td>
                                <td class="px-6 py-4 border-b text-gray-700">"소수명"</td>
                            </tr>
                            <tr>
                                <td class="px-6 py-4 border-b text-gray-600">"모집 기간"</td>
                                <td class="px-6 py-4 border-b text-gray-700">"상시"</td>
                            </tr>
                            <tr>
                                <td class="px-6 py-4 text-gray-600">"수업시작"</td>
                                <td class="px-6 py-4 text-gray-700">"대기후 합류원칙"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </section>
    }
}

/// Middle school class section with table
#[component]
fn MiddleSchoolClassSection() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-6">"중등반 모집"</h2>

                <div class="overflow-x-auto">
                    <table class="w-full bg-white rounded-xl border border-gray-200">
                        <thead class="bg-gray-50">
                            <tr>
                                <th class="px-6 py-4 text-left font-semibold text-gray-700 border-b">"구분"</th>
                                <th class="px-6 py-4 text-left font-semibold text-gray-700 border-b">"내용"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td class="px-6 py-4 border-b text-gray-600">"학년"</td>
                                <td class="px-6 py-4 border-b text-gray-700">"학년 관계없으며 중등 과정을 모두 완료한 학생"</td>
                            </tr>
                            <tr>
                                <td class="px-6 py-4 border-b text-gray-600">"성적 기준"</td>
                                <td class="px-6 py-4 border-b text-gray-700">"N/A"</td>
                            </tr>
                            <tr>
                                <td class="px-6 py-4 border-b text-gray-600">"학습내용"</td>
                                <td class="px-6 py-4 border-b text-gray-700">"내신병행. 질의된 모든 문항 지원"</td>
                            </tr>
                            <tr>
                                <td class="px-6 py-4 border-b text-gray-600">"교재"</td>
                                <td class="px-6 py-4 border-b text-gray-700">"고난도 및 교과외 진행"</td>
                            </tr>
                            <tr>
                                <td class="px-6 py-4 text-gray-600">"비고"</td>
                                <td class="px-6 py-4 text-gray-700">"대기후 합류원칙"</td>
                            </tr>
                        </tbody>
                    </table>
                </div>
            </div>
        </section>
    }
}

/// Inquiry section at the bottom
#[component]
fn InquirySection() -> impl IntoView {
    view! {
        <section class="py-16 bg-brand-600 text-white">
            <div class="container-section">
                <h2 class="text-2xl font-bold mb-6">"문의안내"</h2>
                <p class="mb-4">
                    "아래 내용 문자 남겨 주시면 연락 드립니다. ( 010-5102-0841 별을셀 )"
                </p>
                <ul class="space-y-2 mb-8">
                    <li>"• 학생이름/학교/학년/성별"</li>
                    <li>"• 최종 수학 등급(내신/모의고사)"</li>
                    <li>"• 1년내 목표 등급 또는 점수"</li>
                </ul>

                <A href="/contact" attr:class="btn-primary bg-white text-brand-600 hover:bg-gray-100">
                    "문의 및 입회등록"
                </A>
            </div>
        </section>
    }
}
