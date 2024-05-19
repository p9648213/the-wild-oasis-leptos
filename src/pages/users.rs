use crate::ui::heading::{HeaderVariant, Heading};
use leptos::*;

#[component]
pub fn Users() -> impl IntoView {
    view! { <Heading variant=HeaderVariant::H1>"Create a new user"</Heading> }
}
