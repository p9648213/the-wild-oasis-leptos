use leptos::*;
use tailwind_fuse::tw_merge;

pub enum RowVariant {
    Vertical,
    Horizontal,
}

#[component]
pub fn Row(
    #[prop(default = RowVariant::Vertical)] variant: RowVariant,
    children: Children,
) -> impl IntoView {
    let classes = match variant {
        RowVariant::Horizontal => "justify-between items-center",
        RowVariant::Vertical => "flex-col gap-[1.6rem]",
    };

    view! { <div class=tw_merge!("flex", classes)>{children()}</div> }
}
