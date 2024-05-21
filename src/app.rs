use crate::{
    pages::{
        account::Account, bookings::Bookings, cabins::Cabins, dashboard::Dashboard, login::Login,
        not_found::PageNotFound, settings::Settings, users::Users,
    },
    ui::app_layout::AppLayout,
};
use leptoaster::*;
use leptos::*;
use leptos_query::*;
use leptos_query_devtools::LeptosQueryDevtools;
use leptos_router::*;

#[component]
pub fn App() -> impl IntoView {
    provide_query_client();
    provide_toaster();

    view! {
        <LeptosQueryDevtools/>
        <Toaster stacked=false/>
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
    }
}
