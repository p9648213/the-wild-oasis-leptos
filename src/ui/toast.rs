use leptos::*;

pub enum ToastType {
    Success,
    Error,
}

#[component]
pub fn Toast(msg: &'static str, toast_type: ToastType) -> impl IntoView {
    match toast_type {
        ToastType::Success => view! {
            <div class="flex bg-white items-center px-6 py-4 text-sm border-t-2 rounded-b shadow-sm border-green-500">
                <svg
                    xmlns="http://www.w3.org/2000/svg"
                    class="w-8 h-8 text-green-500 stroke-current"
                    fill="none"
                    viewBox="0 0 24 24"
                >
                    <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        stroke-width="2"
                        d="M5 13l4 4L19 7"
                    ></path>
                </svg>
                <div class="ml-3">
                    <div class="font-bold text-left text-black text-xl">"Success"</div>
                    <div class="w-full text-gray-900 text-xl mt-1">{msg}</div>
                </div>
            </div>
        },
        ToastType::Error => view! {
            <div class="flex bg-white  items-center px-6 py-4 text-sm border-t-2 rounded-b shadow-sm border-red-500">
                <svg
                    viewBox="0 0 24 24"
                    class="w-8 h-8 text-red-500 stroke-current"
                    fill="none"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        d="M12 8V12V8ZM12 16H12.01H12ZM21 12C21 16.9706 16.9706 21 12 21C7.02944 21 3 16.9706 3 12C3 7.02944 7.02944 3 12 3C16.9706 3 21 7.02944 21 12Z"
                        stroke-width="2"
                        stroke-linecap="round"
                        stroke-linejoin="round"
                    ></path>
                </svg>
                <div class="ml-3">
                    <div class="font-bold text-left text-xl text-black">"Error"</div>
                    <div class="w-full text-gray-900 text-xl mt-1">{msg}</div>
                </div>
            </div>
        },
    }
}
