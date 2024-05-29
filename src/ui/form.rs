use leptos::*;
use tailwind_fuse::tw_merge;
use web_sys::SubmitEvent;

pub enum FormType {
    Modal,
    NoneModal,
}

#[component]
pub fn Form<F>(
    form_type: FormType,
    children: Children,
    on_submit: F,
    #[prop(optional)] form_ref: NodeRef<html::Form>,
) -> impl IntoView
where
    F: Fn(SubmitEvent) + 'static,
{
    let form_style = match form_type {
        FormType::NoneModal => " p-[2.4rem_4rem] bg-[var(--color-grey-0)] border border-solid border-[var(--color-grey-100)] rounded-[var(--border-radius-md)]",
        FormType::Modal => "w-[80rem]",
    };

    view! {
        <form
            node_ref=form_ref
            on:submit=on_submit
            class=tw_merge!("overflow-hidden text-[1.4rem]", form_style)
        >
            {children()}
        </form>
    }
}
