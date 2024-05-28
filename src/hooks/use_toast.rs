use crate::ui::toast::{Toast, ToastType};
use leptos::{expect_context, view};
use leptos_toaster::{ToastId, Toasts};

pub fn use_toast() -> impl Fn(&'static str, ToastType) {
    let toast_context = expect_context::<Toasts>();

    let create_toast = move |msg: &'static str, toast_type: ToastType| {
        let toast_id = ToastId::new();
        toast_context.toast(
            view! { <Toast msg=msg toast_type=toast_type/> },
            Some(toast_id),
            None,
        );
    };

    create_toast
}
