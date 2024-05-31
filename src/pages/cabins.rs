use crate::ui::{
    button::Button,
    cabin_table::CabinTable,
    create_cabin_form::CreateCabinForm,
    heading::{HeaderVariant, Heading},
    modal::{Modal, ModalOpen, ModalWindow},
    row::{Row, RowVariant},
};
use leptos::*;

#[component]
pub fn Cabins() -> impl IntoView {
    let (disable, _) = create_signal(false);

    view! {
        <Row variant=RowVariant::Horizontal>
            <Heading variant=HeaderVariant::H1>"All Cabins"</Heading>
            <p>"Filter / Sort"</p>
        </Row>
        <Row>
            <CabinTable/>
        // <div>
        // <Modal>
        // <ModalOpen open_windown_name="cabin-form">
        // <Button disabled=disable on_click=move |_| {}>
        // "Add new cabin"
        // </Button>
        // </ModalOpen>
        // <ModalWindow name="cabin-form">
        // <CreateCabinForm/>
        // </ModalWindow>
        // </Modal>
        // </div>
        </Row>
    }
}
