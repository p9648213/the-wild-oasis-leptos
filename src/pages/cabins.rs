use crate::ui::{
    cabin_table::CabinTable,
    heading::{HeaderVariant, Heading},
    row::{Row, RowVariant},
};
use leptos::*;

#[component]
pub fn Cabins() -> impl IntoView {
    view! {
        <Row variant=RowVariant::Horizontal>
            <Heading variant=HeaderVariant::H1>"All Cabins"</Heading>
            <p>"Filter / Sort"</p>
        </Row>
        <Row>
            <CabinTable/>
        </Row>
    }
}
