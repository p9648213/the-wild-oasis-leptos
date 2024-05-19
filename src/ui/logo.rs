use leptos::*;

#[component]
pub fn Logo() -> impl IntoView {
    view! {
        <div class="flex justify-center">
            <img class="h-[9.6rem] w-auto" src="/public/logo-light.png" alt="Logo"/>
        </div>
    }
}
