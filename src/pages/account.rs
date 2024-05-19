use crate::ui::{
    heading::{HeaderVariant, Heading},
    row::Row,
};
use leptos::*;

#[component]
pub fn Account() -> impl IntoView {
    view! {
        <Heading variant=HeaderVariant::H1>"Update your account"</Heading>

        <Row>
            <Heading variant=HeaderVariant::H3>"Update user data"</Heading>
            <p>"Update user data form"</p>
        </Row>

        <Row>
            <Heading variant=HeaderVariant::H3>"Update password"</Heading>
            <p>"Update user password form"</p>
        </Row>
    }
}
