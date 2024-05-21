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
                <div class="max-w-[120rem] m-[0_auto] flex flex-col gap-[3.2rem]">
                    <Outlet/>
                </div>
            </main>
        </div>
    }
}
