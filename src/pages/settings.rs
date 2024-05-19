use crate::ui::heading::{HeaderVariant, Heading};
use leptos::*;

#[component]
pub fn Settings() -> impl IntoView {
    view! { <Heading variant=HeaderVariant::H1>"Update hotel settings"</Heading> }
}
