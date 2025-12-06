use leptos::prelude::*;
use leptos_router::components::A;

/// 404 Not Found page component
#[component]
pub fn NotFoundPage() -> impl IntoView {
    view! {
        <div class="min-h-[60vh] flex items-center justify-center">
            <div class="text-center px-4">
                <h1 class="text-9xl font-bold text-gray-200">"404"</h1>
                <h2 class="text-3xl font-bold text-gray-900 mt-4 mb-2">"페이지를 찾을 수 없습니다"</h2>
                <p class="text-gray-600 mb-8">
                    "요청하신 페이지가 존재하지 않거나 삭제되었습니다."
                </p>
                <A href="/" attr:class="btn-primary">
                    "홈으로 돌아가기"
                </A>
            </div>
        </div>
    }
}
