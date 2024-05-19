use icondata::{
    HiCalendarDaysOutlineLg, HiCog6ToothOutlineLg, HiHomeModernOutlineLg, HiHomeOutlineLg,
    HiUsersOutlineLg,
};
use leptos::*;
use leptos_icons::Icon;
use leptos_router::A;

#[component]
pub fn MainNav() -> impl IntoView {
    view! {
        <nav>
            <ul class="flex flex-col gap-[0.8rem]">
                <li>
                    <A href="/" active_class="active" class="navlink">
                        <Icon icon=HiHomeOutlineLg/>
                        <span>"Home"</span>
                    </A>
                </li>
                <li>
                    <A href="/bookings" active_class="active" class="navlink">
                        <Icon icon=HiHomeModernOutlineLg/>
                        <span>"Bookings"</span>
                    </A>
                </li>
                <li>
                    <A href="/cabins" active_class="active" class="navlink">
                        <Icon icon=HiCalendarDaysOutlineLg/>
                        <span>"Cabins"</span>
                    </A>
                </li>
                <li>
                    <A href="/users" active_class="active" class="navlink">
                        <Icon icon=HiUsersOutlineLg/>
                        <span>"Users"</span>
                    </A>
                </li>
                <li>
                    <A href="/settings" active_class="active" class="navlink">
                        <Icon icon=HiCog6ToothOutlineLg/>
                        <span>"Settings"</span>
                    </A>
                </li>
            </ul>
        </nav>
    }
}
