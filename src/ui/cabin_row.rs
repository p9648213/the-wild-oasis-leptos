use crate::hooks::use_create_cabin::use_create_cabin;
use crate::hooks::use_toast::use_toast;
use crate::model::cabins::Cabin;
use crate::services::helpers::format_currency;
use crate::ui::{
    confirm_delete::ConfirmDeleteCabin,
    create_cabin_form::{CabinAction, CreateCabinForm},
    modal::{Modal, ModalOpen, ModalWindow},
    table::TableRow,
    toast::ToastType,
};
use icondata::{HiPencilSolidLg, HiSquare2StackSolidLg, HiTrashSolidLg};
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn CabinRow(cabin: Cabin) -> impl IntoView {
    let create_toast = use_toast();
    let (creating, create_cabin_error, create_cabin) = use_create_cabin(false);

    let cabin_clone = cabin.clone();
    let cabin_clone_1 = cabin.clone();

    let create_duplicate_cabin = move |_| {
        let new_cabin = Cabin {
            id: None,
            created_at: None,
            image: Some(cabin_clone.clone().image.unwrap_or_default()),
            description: cabin_clone.clone().description,
            discount: cabin_clone.discount,
            max_capacity: cabin_clone.max_capacity,
            regular_price: cabin_clone.regular_price,
            name: cabin_clone.clone().name,
        };

        create_cabin(CabinAction {
            cabin: new_cabin,
            image_file: None,
        })
    };

    create_effect(move |_| match create_cabin_error() {
        Ok(res) => match res {
            Some(_) => create_toast("New cabin succesfully created", ToastType::Success),
            None => (),
        },
        Err(_) => create_toast("Failed to add cabin", ToastType::Error),
    });

    view! {
        <TableRow>
            <img
                class="block w-[6.4rem] aspect-[3/2] object-cover object-center scale-[1.5] translate-x-[-7px]"
                src=cabin.clone().image
                alt=&cabin.name
            />

            <div class="text-[1.6rem] font-[600] text-[var(--color-grey-600)] font-[Sono]">
                {&cabin.name}
            </div>

            <div>"Fits up to " {cabin.max_capacity} " guest"</div>

            <div class="font-[Sono] font-[600]">{format_currency(cabin.regular_price)}</div>

            <div class="font-[Sono] font-[600] text-[var(--color-green-700)]">
                {format_currency(cabin.discount)}
            </div>

            <div class="flex gap-4">
                <button
                    on:click=create_duplicate_cabin
                    disabled=creating
                    class="focus:outline-none focus-visible:outline-none"
                >
                    <Icon class="fill-slate-700" icon=HiSquare2StackSolidLg/>
                </button>

                <Modal>
                    <ModalOpen open_windown_name="edit-cabin">
                        <button
                            on:click=move |_| {}
                            disabled=creating
                            class="focus:outline-none focus-visible:outline-none"
                        >
                            <Icon class="fill-slate-700" icon=HiPencilSolidLg/>
                        </button>
                    </ModalOpen>
                    <ModalWindow name="edit-cabin">
                        <CreateCabinForm cabin=Some(cabin.clone())/>
                    </ModalWindow>
                </Modal>

                <Modal>
                    <ModalOpen open_windown_name="delete-cabin">
                        <button
                            class="focus:outline-none focus-visible:outline-none"
                            // on:click=move |_| delete_cabin_by_id(cabin_delete_clone.id.unwrap_or(0))
                            on:click=move |_| {}
                            disabled=creating
                        >
                            <Icon class="fill-slate-700" icon=HiTrashSolidLg/>
                        </button>
                    </ModalOpen>
                    <ModalWindow name="delete-cabin">
                        <ConfirmDeleteCabin id=cabin_clone_1.clone().id.unwrap_or_default()/>
                    </ModalWindow>
                </Modal>
            </div>
        </TableRow>
    }
}
