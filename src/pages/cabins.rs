use crate::ui::{
    button::Button,
    cabin_table::CabinTable,
    create_cabin_form::CreateCabinForm,
    heading::{HeaderVariant, Heading},
    row::{Row, RowVariant},
};
use leptos::*;

#[component]
pub fn Cabins() -> impl IntoView {
    let (show_form, set_show_form) = create_signal(false);

    view! {
        <Row variant=RowVariant::Horizontal>
            <Heading variant=HeaderVariant::H1>"All Cabins"</Heading>
            <p>"Filter / Sort"</p>
        </Row>
        <Row>
            <CabinTable/>
            <Button on_click=move |_| {
                set_show_form.update(|value| *value = !*value)
            }>"Add new cabin"</Button>
            <Show when=move || show_form.get() == true fallback=move || view! {}>
                <CreateCabinForm/>
            </Show>
        </Row>
    }
}
