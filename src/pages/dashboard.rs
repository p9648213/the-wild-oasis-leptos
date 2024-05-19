use crate::ui::{
    heading::{HeaderVariant, Heading},
    row::{Row, RowVariant},
};
use leptos::*;

#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <Row variant=RowVariant::Horizontal>
            <Heading variant=HeaderVariant::H1>"Dashboard"</Heading>
            <p>"TEST"</p>
        </Row>
    }
}
