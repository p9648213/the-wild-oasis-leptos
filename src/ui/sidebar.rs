use crate::ui::{logo::Logo, main_nav::MainNav};
use leptos::*;

#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <aside class="bg-[var(--color-grey-0)] p-[3.2rem_2.4rem] border-[var(--color-grey-100)] border-r border-solid row-[1/-1] flex flex-col gap-[3.2rem]">
            <Logo/>
            <MainNav/>
        </aside>
    }
}
