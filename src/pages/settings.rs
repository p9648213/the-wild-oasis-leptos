use crate::ui::{
    heading::{HeaderVariant, Heading},
    row::Row,
    update_setting_form::UpdateSettingForm,
};
use leptos::*;

#[component]
pub fn Settings() -> impl IntoView {
    view! {
        <Row>
            <Heading variant=HeaderVariant::H1>"Update hotel settings"</Heading>
            <UpdateSettingForm/>
        </Row>
    }
}
