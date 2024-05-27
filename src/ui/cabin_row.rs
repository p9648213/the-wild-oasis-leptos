use crate::model::cabins::Cabin;
use crate::services::{
    api_cabins::delete_cabin,
    helpers::format_currency,
};
use leptos::*;
use leptos_toaster::*;
use crate::ui::{toast::{Toast,ToastType}, create_cabin_form::CreateCabinForm};

#[component]
pub fn CabinRow(cabin: Cabin) -> impl IntoView {
    let (show_edit_form, set_show_edit_form) = create_signal(false);

    let toast_context = expect_context::<Toasts>();

    let create_toast = move |msg: &'static str, toast_type: ToastType| {
        let toast_id = ToastId::new();
        toast_context.toast(
            view! { <Toast msg=msg toast_type=toast_type/> },
            Some(toast_id),
            None
        );
    };

    let delete_cabin_action = create_action(|input: &u32| {
        let id = input.clone();
        async move {delete_cabin(id).await}
    });

    let loading = delete_cabin_action.pending();
    let data = delete_cabin_action.value();

    let delete_error = 
        move || data.with(|resp| {
            match resp {
                Some(resp) => match resp {
                    Ok(msg) => Ok(Some(msg.clone())),
                    Err(err) => Err(err.clone()),
                },
                None => Ok(None),
            }
        });
    

    let onDeleteCabin = move |id: u32| {
        delete_cabin_action.dispatch(id)
    };

    create_effect(move |_| {
        match delete_error() {
            Ok(res) => {
                match res {
                    Some(_) => create_toast("Cabin delete successfully", ToastType::Success),
                    None => (),
                }
            },
            Err(_) => create_toast("Failed to delete cabin", ToastType::Error)
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
                    on:click=move |_| set_show_edit_form.update(|show| *show = !*show)
                    disabled=loading
                >
                    "Edit"
                </button>

                <button on:click=move |_| onDeleteCabin(cabin.id.unwrap_or(0)) disabled=loading>
                    "Delete"
                </button>
            </div>
        </div>
        <Show when=move || show_edit_form.get() == true fallback=move || view! {}>
            <CreateCabinForm cabin=Some(cabin.clone())/>
        </Show>
    }
}
