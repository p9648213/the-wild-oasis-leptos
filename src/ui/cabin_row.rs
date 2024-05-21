use crate::model::cabins::Cabin;
use crate::services::{
    api_cabins::delete_cabin,
    helpers::format_currency,
};
use leptos::*;
use leptoaster::*;

#[component]
pub fn CabinRow(cabin: Cabin) -> impl IntoView {
    let toaster = expect_toaster();

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
                    Some(_) => {
                        toaster.success("Delete cabin successfully")
                    },
                    None => (),
                }
            },
            Err(_) => {
                toaster.error("Failed to delete cabin")
            }
        }
    });

    view! {
        <div class="grid grid-cols-[0.6fr_1.8fr_2.2fr_1fr_1fr_1fr] gap-x-[2.4rem] items-center p-[1.4rem_2.4rem] [&:not(:last-child)]:border-b [&:not(:last-child)]:border-solid [&:not(:last-child)]:border-[var(--color-grey-100)]">
            <img
                class="block w-[6.4rem] aspect-[3/2] object-cover object-center scale-[1.5] translate-x-[-7px]"
                src=cabin.image
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

            <button on:click=move |_| onDeleteCabin(cabin.id) disabled=loading>
                "Delete"
            </button>
        </div>
    }
}
