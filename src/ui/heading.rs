use leptos::*;
use tailwind_fuse::tw_merge;

pub enum HeaderVariant {
    H1,
    H2,
    H3,
}

#[component]
pub fn Heading<'a>(
    variant: HeaderVariant,
    children: Children,
    #[prop(optional)] class: &'a str,
) -> impl IntoView {
    let classes = match variant {
        HeaderVariant::H1 => "text-[3rem] font-[600]",
        HeaderVariant::H2 => "text-[2rem] font-[600]",
        HeaderVariant::H3 => "text-[2rem] font-[500]",
    };

    view! { <div class=tw_merge!("leading-[1.4]", classes, class)>{children()}</div> }
}
