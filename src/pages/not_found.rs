use crate::ui::heading::{HeaderVariant, Heading};
use leptos::*;
use leptos_router::use_navigate;

#[component]
pub fn PageNotFound() -> impl IntoView {
    let navigate = use_navigate();

    view! {
        <main class="h-screen bg-[var(--color-grey-50)] flex items-center justify-center p-[4.8rem]">
            <div class="bg-[var(--color-grey-0)] border border-[var(--color-grey-100)] rounded-[var(--border-radius-md)] border-solid p-[4.8rem] flex-[0_1_96rem] text-center">
                <Heading class="mb-[3.2rem]" variant=HeaderVariant::H1>
                    "The page you are looking for could not be found 😢"
                </Heading>
                <button on:click=move |_| navigate("/", Default::default())>"Go back"</button>
            </div>
        </main>
    }
}
