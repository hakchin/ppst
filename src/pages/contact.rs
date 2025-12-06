use leptos::prelude::*;

use crate::components::icons::{ChatIcon, CheckIcon, ClockIcon, GlobeIcon, LocationIcon, PhoneIcon};
use crate::components::DirectionsSection;
use crate::server_fns::submit_contact;

/// Validate Korean mobile phone number format
fn is_valid_phone(phone: &str) -> bool {
    let re = regex::Regex::new(r"^01[016789]-?\d{3,4}-?\d{4}$").unwrap();
    re.is_match(phone)
}

/// Contact page component
#[component]
pub fn ContactPage() -> impl IntoView {
    view! {
        <div>
            <ContactHeader/>
            <ContactFormSection/>
            <DirectionsSection/>
        </div>
    }
}

/// Page header section
#[component]
fn ContactHeader() -> impl IntoView {
    view! {
        <section class="bg-white py-16">
            <div class="container-section">
                <h1 class="text-4xl md:text-5xl font-bold mb-4">"Contact"</h1>
                <p class="text-xl text-gray-600 max-w-2xl">
                    "문의 및 입회등록을 원하시면 메시지를 남겨 주세요. 성실히 답변드리겠습니다."
                </p>
            </div>
        </section>
    }
}

/// Contact form section with form and contact info
#[component]
fn ContactFormSection() -> impl IntoView {
    view! {
        <section class="py-16">
            <div class="container-section">
                <div class="grid grid-cols-1 lg:grid-cols-2 gap-12">
                    <ContactForm/>
                    <ContactInfoPanel/>
                </div>
            </div>
        </section>
    }
}

/// Contact form with submission handling
#[component]
fn ContactForm() -> impl IntoView {
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
            set_phone_error.set(Some("올바른 휴대폰 번호를 입력해주세요 (예: 010-1234-5678)".to_string()));
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
        <div>
            {move || {
                if let Some(Ok(())) = submission_result.get() {
                    view! { <SuccessMessage on_reset=reset_form/> }.into_any()
                } else {
                    view! {
                        <ContactFormFields
                            name=name
                            set_name=set_name
                            phone=phone
                            set_phone=set_phone
                            phone_error=phone_error
                            set_phone_error=set_phone_error
                            message=message
                            set_message=set_message
                            on_submit=on_submit
                            is_pending=is_pending
                            submission_result=submission_result
                        />
                    }.into_any()
                }
            }}
        </div>
    }
}

/// Success message after form submission
#[component]
fn SuccessMessage<F>(on_reset: F) -> impl IntoView
where
    F: Fn(leptos::ev::MouseEvent) + 'static,
{
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
                on:click=on_reset
            >
                "추가 메시지 보내기"
            </button>
        </div>
    }
}

/// Form fields component
#[component]
fn ContactFormFields<S>(
    name: ReadSignal<String>,
    set_name: WriteSignal<String>,
    phone: ReadSignal<String>,
    set_phone: WriteSignal<String>,
    phone_error: ReadSignal<Option<String>>,
    set_phone_error: WriteSignal<Option<String>>,
    message: ReadSignal<String>,
    set_message: WriteSignal<String>,
    on_submit: impl Fn(leptos::ev::SubmitEvent) + 'static,
    is_pending: Memo<bool>,
    submission_result: S,
) -> impl IntoView
where
    S: Get<Value = Option<Result<(), ServerFnError>>> + Send + Sync + Copy + 'static,
{
    view! {
        <form on:submit=on_submit class="space-y-6">
            <ErrorMessage submission_result=submission_result/>
            <FormField
                id="name"
                label="이름"
                input_type="text"
                placeholder="이름을 입력하세요"
                value=name
                set_value=set_name
                is_pending=is_pending
            />
            <div>
                <label for="phone" class="block text-sm font-medium text-gray-700 mb-2">
                    "휴대폰 번호"
                </label>
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
            <MessageField
                value=message
                set_value=set_message
                is_pending=is_pending
            />
            <button
                type="submit"
                class="btn-primary w-full"
                disabled=move || is_pending.get()
            >
                {move || if is_pending.get() { "전송 중..." } else { "메시지 보내기" }}
            </button>
        </form>
    }
}

/// Error message display
#[component]
fn ErrorMessage<S>(submission_result: S) -> impl IntoView
where
    S: Get<Value = Option<Result<(), ServerFnError>>> + Send + Sync + Copy + 'static,
{
    move || {
        submission_result.get().and_then(|r| r.err()).map(|e| {
            view! {
                <div class="bg-red-50 border border-red-200 rounded-lg p-4 text-red-700">
                    {e.to_string()}
                </div>
            }
        })
    }
}

/// Generic form input field
#[component]
fn FormField(
    id: &'static str,
    label: &'static str,
    input_type: &'static str,
    placeholder: &'static str,
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    is_pending: Memo<bool>,
) -> impl IntoView {
    view! {
        <div>
            <label for=id class="block text-sm font-medium text-gray-700 mb-2">
                {label}
            </label>
            <input
                type=input_type
                id=id
                name=id
                required
                class="form-input"
                placeholder=placeholder
                prop:value=move || value.get()
                on:input=move |ev| set_value.set(event_target_value(&ev))
                disabled=move || is_pending.get()
            />
        </div>
    }
}

/// Message textarea field
#[component]
fn MessageField(
    value: ReadSignal<String>,
    set_value: WriteSignal<String>,
    is_pending: Memo<bool>,
) -> impl IntoView {
    view! {
        <div>
            <label for="message" class="block text-sm font-medium text-gray-700 mb-2">
                "내용"
            </label>
            <textarea
                id="message"
                name="message"
                required
                rows="5"
                class="form-textarea"
                placeholder="문의하실 내용을 적어주세요."
                prop:value=move || value.get()
                on:input=move |ev| set_value.set(event_target_value(&ev))
                disabled=move || is_pending.get()
            />
        </div>
    }
}

/// Contact information panel
#[component]
fn ContactInfoPanel() -> impl IntoView {
    view! {
        <div class="lg:pl-8">
            <h2 class="text-2xl font-bold mb-6">"문의 정보"</h2>
            <div class="space-y-6">
                <ContactInfoItem title="주소" content="경기도 군포시 번영로 489 중앙타워 2층 ★별을셀수학 (지번: 산본동 1142-7)">
                    <LocationIcon class="w-5 h-5 text-brand-600"/>
                </ContactInfoItem>
                <ContactInfoItem title="웹사이트" content="https://starrystarry.kr">
                    <GlobeIcon class="w-5 h-5 text-brand-600"/>
                </ContactInfoItem>
                <ContactInfoItem title="전화" content="010-5102-0841">
                    <PhoneIcon class="w-5 h-5 text-brand-600"/>
                </ContactInfoItem>
                <ContactInfoItem title="카카오" content="별을셀 검색">
                    <ChatIcon class="w-5 h-5 text-brand-600"/>
                </ContactInfoItem>
                <ContactInfoItem title="운영 안내" content="상담 및 수업 시간은 문의 시 안내드립니다">
                    <ClockIcon class="w-5 h-5 text-brand-600"/>
                </ContactInfoItem>
            </div>
        </div>
    }
}

/// Single contact info item with icon slot
#[component]
fn ContactInfoItem(
    title: &'static str,
    content: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="flex gap-4">
            <div class="flex-shrink-0 w-10 h-10 bg-brand-100 rounded-lg flex items-center justify-center">
                {children()}
            </div>
            <div>
                <h3 class="font-semibold">{title}</h3>
                <p class="text-gray-600">{content}</p>
            </div>
        </div>
    }
}

