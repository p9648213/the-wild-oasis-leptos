use crate::{
    hooks::{use_delete_cabin::use_delete_cabin, use_toast::use_toast},
    ui::{
        button::{Button, ButtonVariant},
        heading::{HeaderVariant, Heading},
        toast::ToastType,
    },
};
use leptos::*;

#[component]
pub fn ConfirmDeleteCabin(id: u32) -> impl IntoView {
    let create_toast = use_toast();
    let (deleting, delete_error, delete_cabin_by_id) = use_delete_cabin();
    let set_open_name = use_context::<WriteSignal<&str>>().expect("set_open_name is not provided");

    create_effect(move |_| {
        match delete_error() {
            Ok(res) => match res {
                Some(_) => create_toast("Cabin delete successfully", ToastType::Success),
                None => (),
            },
            Err(_) => create_toast("Failed to delete cabin", ToastType::Error),
        };
    });

    view! {
        <div class="w-[40rem] flex flex-col gap-[1.2rem]">
            <Heading variant=HeaderVariant::H3>"Delete cabin"</Heading>
            <p class="text-[var(--color-grey-500)] mb-[1.2rem]">
                "Are you sure you want to delete this cabin
                permanently? This action cannot be undone."
            </p>
            <div class="flex justify-end gap-[1.2rem]">
                <Button
                    variant=ButtonVariant::Secondary
                    disabled=deleting
                    on_click=move |_| set_open_name("")
                >
                    "Cancel"
                </Button>
                <Button
                    variant=ButtonVariant::Danger
                    disabled=deleting
                    on_click=move |_| delete_cabin_by_id(id)
                >
                    "Delete"
                </Button>
            </div>
        </div>
    }
}
