use leptos::prelude::*;

use crate::components::icons::{ChatIcon, CheckIcon, ClockIcon, EmailIcon, LocationIcon, PhoneIcon};
use crate::constants::contact;
use crate::server_fns::submit_contact;

/// Validate Korean mobile phone number format
fn is_valid_phone(phone: &str) -> bool {
    let re = regex::Regex::new(r"^01[016789]-?\d{3,4}-?\d{4}$").unwrap();
    re.is_match(phone)
}

/// Contact section with form
#[component]
pub fn ContactSection() -> impl IntoView {
    let (name, set_name) = signal(String::new());
    let (phone, set_phone) = signal("010-".to_string());
    let (message, set_message) = signal(String::new());
    let (phone_error, set_phone_error) = signal(Option::<String>::None);

    let submit_action = Action::new(move |_: &()| {
        let name_val = name.get();
        let phone_val = phone.get();
        let message_val = message.get();
        async move { submit_contact(name_val, phone_val, message_val).await }
    });

    let is_pending = submit_action.pending();
    let submission_result = submit_action.value();

    let on_submit = move |ev: leptos::ev::SubmitEvent| {
        ev.prevent_default();

        // Validate phone number before submission
        let phone_val = phone.get();
        if !is_valid_phone(&phone_val) {
            set_phone_error.set(Some(
                "올바른 휴대폰 번호를 입력해주세요 (예: 010-1234-5678)".to_string(),
            ));
            return;
        }

        set_phone_error.set(None);
        submit_action.dispatch(());
    };

    let reset_form = move |_| {
        set_name.set(String::new());
        set_phone.set("010-".to_string());
        set_message.set(String::new());
        set_phone_error.set(None);
        submit_action.value().set(None);
    };

    view! {
        <section id="contact" class="py-16 scroll-mt-16">
            <div class="container-section">
                <h2 class="section-title">"Contact"</h2>
                <p class="text-xl text-gray-600 max-w-2xl mb-12">
                    "문의 및 입회등록을 원하시면 메시지를 남겨 주세요. 성실히 답변드리겠습니다."
                </p>

                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    // Contact Form
                    <div>
                        {move || {
                            if let Some(Ok(())) = submission_result.get() {
                                view! {
                                    <div class="bg-green-50 border border-green-200 rounded-xl p-8 text-center">
                                        <div class="w-16 h-16 bg-green-100 rounded-full flex items-center justify-center mx-auto mb-4">
                                            <CheckIcon class="w-8 h-8 text-green-600"/>
                                        </div>
                                        <h3 class="text-xl font-semibold text-green-800 mb-2">"메시지가 전송되었습니다!"</h3>
                                        <p class="text-green-600">"문의해 주셔서 감사합니다. 빠른 시일 내에 답변드리겠습니다."</p>
                                        <button
                                            type="button"
                                            class="mt-4 text-green-700 underline"
                                            on:click=reset_form
                                        >
                                            "추가 메시지 보내기"
                                        </button>
                                    </div>
                                }.into_any()
                            } else {
                                view! {
                                    <form on:submit=on_submit class="space-y-6">
                                        {move || {
                                            submission_result.get().and_then(|r| r.err()).map(|e| {
                                                view! {
                                                    <div class="bg-red-50 border border-red-200 rounded-lg p-4 text-red-700">
                                                        {e.to_string()}
                                                    </div>
                                                }
                                            })
                                        }}
                                        <div>
                                            <label for="name" class="block text-sm font-medium text-gray-700 mb-2">"이름"</label>
                                            <input
                                                type="text"
                                                id="name"
                                                name="name"
                                                required
                                                class="form-input"
                                                placeholder="이름을 입력하세요"
                                                prop:value=move || name.get()
                                                on:input=move |ev| set_name.set(event_target_value(&ev))
                                                disabled=move || is_pending.get()
                                            />
                                        </div>
                                        <div>
                                            <label for="phone" class="block text-sm font-medium text-gray-700 mb-2">"휴대폰 번호"</label>
                                            <input
                                                type="tel"
                                                id="phone"
                                                name="phone"
                                                required
                                                class=move || {
                                                    if phone_error.get().is_some() {
                                                        "form-input border-red-500"
                                                    } else {
                                                        "form-input"
                                                    }
                                                }
                                                placeholder="010-1234-5678"
                                                inputmode="tel"
                                                autocomplete="tel"
                                                maxlength="13"
                                                prop:value=move || phone.get()
                                                on:input=move |ev| {
                                                    set_phone.set(event_target_value(&ev));
                                                    set_phone_error.set(None);
                                                }
                                                disabled=move || is_pending.get()
                                            />
                                            {move || phone_error.get().map(|err| view! {
                                                <p class="mt-1 text-sm text-red-600">{err}</p>
                                            })}
                                        </div>
                                        <div>
                                            <label for="message" class="block text-sm font-medium text-gray-700 mb-2">"내용"</label>
                                            <textarea
                                                id="message"
                                                name="message"
                                                required
                                                rows="5"
                                                class="form-textarea"
                                                placeholder="문의하실 내용을 적어주세요."
                                                prop:value=move || message.get()
                                                on:input=move |ev| set_message.set(event_target_value(&ev))
                                                disabled=move || is_pending.get()
                                            />
                                        </div>
                                        <button
                                            type="submit"
                                            class="btn-primary w-full"
                                            disabled=move || is_pending.get()
                                        >
                                            {move || if is_pending.get() { "전송 중..." } else { "메시지 보내기" }}
                                        </button>
                                    </form>
                                }.into_any()
                            }
                        }}
                    </div>

                    // Contact Info
                    <div class="lg:pl-8">
                        <h3 class="text-2xl font-bold mb-6">"문의 정보"</h3>
                        <div class="space-y-6">
                            <div class="flex gap-4">
                                <div class="flex-shrink-0 w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center">
                                    <LocationIcon class="w-5 h-5 text-brand-600"/>
                                </div>
                                <div>
                                    <h4 class="font-semibold">"주소"</h4>
                                    <p class="text-gray-600">{contact::ADDRESS_WITH_LOT}</p>
                                </div>
                            </div>
                            <div class="flex gap-4">
                                <div class="flex-shrink-0 w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center">
                                    <EmailIcon class="w-5 h-5 text-brand-600"/>
                                </div>
                                <div>
                                    <h4 class="font-semibold">"웹사이트"</h4>
                                    <p class="text-gray-600">{contact::WEBSITE}</p>
                                </div>
                            </div>
                            <div class="flex gap-4">
                                <div class="flex-shrink-0 w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center">
                                    <PhoneIcon class="w-5 h-5 text-brand-600"/>
                                </div>
                                <div>
                                    <h4 class="font-semibold">"전화"</h4>
                                    <p class="text-gray-600">{contact::PHONE}</p>
                                </div>
                            </div>
                            <div class="flex gap-4">
                                <div class="flex-shrink-0 w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center">
                                    <ChatIcon class="w-5 h-5 text-brand-600"/>
                                </div>
                                <div>
                                    <h4 class="font-semibold">"카카오"</h4>
                                    <p class="text-gray-600">"별을셀 검색"</p>
                                </div>
                            </div>
                            <div class="flex gap-4">
                                <div class="flex-shrink-0 w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center">
                                    <ClockIcon class="w-5 h-5 text-brand-600"/>
                                </div>
                                <div>
                                    <h4 class="font-semibold">"운영 안내"</h4>
                                    <p class="text-gray-600">"상담 및 수업 시간은 문의 시 안내드립니다"</p>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </section>
    }
}
