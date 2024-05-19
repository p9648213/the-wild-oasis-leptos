use crate::ui::{
    heading::{HeaderVariant, Heading},
    row::{Row, RowVariant},
};
use leptos::*;

#[component]
pub fn Bookings() -> impl IntoView {
    view! {
        <Row variant=RowVariant::Horizontal>
            <Heading variant=HeaderVariant::H1>"All Bookings"</Heading>
            <p>"TEST"</p>
        </Row>
    }
}
