use crate::services::api_cabins::{all_cabins_query, AllCabinsKey};
use crate::ui::{
    cabin_row::CabinRow,
    spinner::Spinner,
    table::{Table, TableBody, TableHeader},
};
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
        <Table column="grid-cols-[0.6fr_1.8fr_2.2fr_1fr_1fr_1fr]">
            <TableHeader>
                <div></div>
                <div>"Cabin"</div>
                <div>"Capacity"</div>
                <div>"Price"</div>
                <div>"Discount"</div>
                <div></div>
            </TableHeader>
            <TableBody>{cabin}</TableBody>
        </Table>
    }
}
