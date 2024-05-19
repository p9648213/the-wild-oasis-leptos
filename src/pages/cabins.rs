use crate::ui::{
    heading::{HeaderVariant, Heading},
    row::{Row, RowVariant},
};
use leptos::*;

#[component]
pub fn Cabins() -> impl IntoView {
    view! {
        <Row variant=RowVariant::Horizontal>
            <Heading variant=HeaderVariant::H1>"All Cabins"</Heading>
            <p>"TEST"</p>
        </Row>
    }
}
