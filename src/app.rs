use crate::services::api_cabins::{all_cabins_query, AllCabinsKey};
use crate::{
    pages::{
        account::Account, bookings::Bookings, cabins::Cabins, dashboard::Dashboard, login::Login,
        not_found::PageNotFound, settings::Settings, users::Users,
    },
    ui::app_layout::AppLayout,
};
use leptos::*;
use leptos_query::*;
use leptos_query_devtools::LeptosQueryDevtools;
use leptos_router::*;
use leptos_toaster::*;
use leptos_use::use_window_focus;

#[component]
pub fn App() -> impl IntoView {
    provide_query_client();

    let query = all_cabins_query().use_query(|| AllCabinsKey);
    let window_focus = use_window_focus();

    create_effect(move |_| {
        if window_focus.get() {
            (query.refetch)()
        }
    });

    view! {
        <Toaster position=ToasterPosition::TopRight expand=true>
            <LeptosQueryDevtools/>
            <Router>
                <Routes>
                    <Route path="/" view=AppLayout>
                        <Route path="/" view=Dashboard/>
                        <Route path="/bookings" view=Bookings/>
                        <Route path="/cabins" view=Cabins/>
                        <Route path="/users" view=Users/>
                        <Route path="/settings" view=Settings/>
                        <Route path="/account" view=Account/>
                    </Route>
                    <Route path="/login" view=Login/>
                    <Route path="/*any" view=PageNotFound/>
                </Routes>
            </Router>
        </Toaster>
    }
}
