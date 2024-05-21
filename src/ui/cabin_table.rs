use crate::services::api_cabins::{all_cabins_query, AllCabinsKey};
use crate::ui::{cabin_row::CabinRow, spinner::Spinner};
use leptos::*;

#[component]
pub fn CabinTable() -> impl IntoView {
    let query = all_cabins_query().use_query(|| AllCabinsKey);
    let data = query.data;

    let cabin = move || {
        data.with(|cabin| match cabin {
            Some(cabin) => match cabin {
                Ok(cabin) => {
                    if cabin.is_empty() {
                        view! { <p>"No cabin found"</p> }.into_view()
                    } else {
                        cabin
                            .into_iter()
                            .map(|cabin| view! { <CabinRow cabin=cabin.clone()/> })
                            .collect_view()
                    }
                }
                Err(err) => view! { <p>{err.to_string()}</p> }.into_view(),
            },
            None => view! { <Spinner/> }.into_view(),
        })
    };

    view! {
        <div class="border border-solid border-[var(--color-grey-200)] text-[1.4rem] bg-[var(--color-grey-0)] rounded-[7px] overflow-hidden">
            <header class="grid grid-cols-[0.6fr_1.8fr_2.2fr_1fr_1fr_1fr] gap-x-[2.4rem] items-center bg-[var(--color-grey-50)] border-b border-solid border-[var(--color-grey-100)] uppercase tracking-[0.4px] font-[600] text-[var(--color-grey-600)] p-[1.6rem_2.4rem]">
                <div></div>
                <div>"Cabin"</div>
                <div>"Capacity"</div>
                <div>"Price"</div>
                <div>"Discount"</div>
                <div></div>
            </header>
            {cabin}
        </div>
    }
}
