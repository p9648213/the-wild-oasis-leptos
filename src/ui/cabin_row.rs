use crate::hooks::use_create_cabin::use_create_cabin;
use crate::hooks::use_delete_cabin::use_delete_cabin;
use crate::hooks::use_toast::use_toast;
use crate::model::cabins::Cabin;
use crate::services::helpers::format_currency;
use crate::ui::create_cabin_form::CabinAction;
use crate::ui::{create_cabin_form::CreateCabinForm, toast::ToastType};
use icondata::{HiPencilSolidLg, HiSquare2StackSolidLg, HiTrashSolidLg};
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn CabinRow(cabin: Cabin) -> impl IntoView {
    let (show_edit_form, set_show_edit_form) = create_signal(false);

    let create_toast = use_toast();

    let (deleting, delete_error, delete_cabin_by_id) = use_delete_cabin();
    let (creating, create_cabin_error, create_cabin) = use_create_cabin(false);

    let loading = move || {
        if deleting() == true {
            return true;
        } else {
            if creating() == true {
                return true;
            }
            false
        }
    };

    let cabin_clone = cabin.clone();

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

    create_effect(move |_| {
        match delete_error() {
            Ok(res) => match res {
                Some(_) => create_toast("Cabin delete successfully", ToastType::Success),
                None => (),
            },
            Err(_) => create_toast("Failed to delete cabin", ToastType::Error),
        };

        match create_cabin_error() {
            Ok(res) => match res {
                Some(_) => create_toast("New cabin succesfully created", ToastType::Success),
                None => (),
            },
            Err(_) => create_toast("Failed to add cabin", ToastType::Error),
        }
    });

    view! {
        <div class="grid grid-cols-[0.6fr_1.8fr_2.2fr_1fr_1fr_1fr] gap-x-[2.4rem] items-center p-[1.4rem_2.4rem] [&:not(:last-child)]:border-b [&:not(:last-child)]:border-solid [&:not(:last-child)]:border-[var(--color-grey-100)]">
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
                    disabled=loading
                    class="focus:outline-none focus-visible:outline-none"
                >
                    <Icon class="fill-slate-700" icon=HiSquare2StackSolidLg/>
                </button>

                <button
                    on:click=move |_| set_show_edit_form.update(|show| *show = !*show)
                    disabled=loading
                    class="focus:outline-none focus-visible:outline-none"
                >
                    <Icon class="fill-slate-700" icon=HiPencilSolidLg/>
                </button>

                <button
                    class="focus:outline-none focus-visible:outline-none"
                    on:click=move |_| delete_cabin_by_id(cabin.id.unwrap_or(0))
                    disabled=loading
                >
                    <Icon class="fill-slate-700" icon=HiTrashSolidLg/>
                </button>
            </div>
        </div>
        <Show when=move || show_edit_form.get() == true fallback=move || view! {}>
            <CreateCabinForm cabin=Some(cabin.clone())/>
        </Show>
    }
}
