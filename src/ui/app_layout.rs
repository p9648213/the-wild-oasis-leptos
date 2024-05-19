use crate::ui::{header::Header, sidebar::Sidebar};
use leptos::*;
use leptos_router::Outlet;

#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <div class="grid h-screen grid-cols-[26rem_1fr] grid-rows-[auto_1fr]">
            <Sidebar/>
            <Header/>
            <main class="bg-[var(--color-grey-50)] p-[4rem_4.8rem_6.4rem]">
                <Outlet/>
            </main>
        </div>
    }
}
