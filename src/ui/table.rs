use leptos::*;
use tailwind_fuse::tw_merge;

#[component]
pub fn Table(children: Children, column: &'static str) -> impl IntoView {
    run_as_child(move || {
        provide_context(column);

        view! {
            <div class="border border-solid border-[var(--color-grey-200)] text-[1.4rem] bg-[var(--color-grey-0)] rounded-[7px] overflow-hidden">
                {children()}
            </div>
        }
    })
}

#[component]
pub fn TableHeader(children: Children) -> impl IntoView {
    let column = use_context::<&str>().expect("column is not provided");
    let header_class = tw_merge!("grid p-[1.6rem_2.4rem] bg-[var(--color-grey-50)] border-b border-solid border-b-[var(--color-grey-100)] uppercase tracking-[0.4px] font-[600] text-[var(--color-grey-600)] gap-x-[2.4rem] items-center transition-none", column);

    view! { <header class=header_class>{children()}</header> }
}

#[component]
pub fn TableRow(children: Children) -> impl IntoView {
    let column = use_context::<&str>().expect("column is not provided");
    let row_class = tw_merge!(
        "grid gap-x-[2.4rem] items-center transition-none",
        column,
        "row-table"
    );

    view! { <div class=row_class>{children()}</div> }
}

#[component]
pub fn TableBody(children: Children) -> impl IntoView {
    view! { <div class="m-[0.4rem_0]">{children()}</div> }
}
