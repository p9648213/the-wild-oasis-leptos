use leptos::*;
use tailwind_fuse::tw_merge;

enum ButtonVariant {
    Primary,
    Secondary,
    Danger,
}

enum ButtonSize {
    Small,
    Medium,
    Large,
}

#[component]
pub fn Button(
    children: Children,
    #[prop(default = ButtonVariant::Primary)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Medium)] size: ButtonSize,
) -> impl IntoView {
    let variant_classes = match variant {
        ButtonVariant::Primary => "text-[var(--color-brand-50)] bg-[var(--color-brand-600)] hover:bg-[var(--color-brand-700)]",
        ButtonVariant::Secondary => "text-[var(--color-grey-600))] bg-[var(--color-grey-0))] hover:bg-[var(--color-grey-50)] border border-solid border-[var(--color-grey-200)]",
        ButtonVariant::Danger => "text-[var(--color-red-100)] bg-[var(--color-red-700)] hover:bg-[var(--color-red-800)]",
    };

    let size_classes = match size {
        ButtonSize::Small => "text-[1.2rem] p-[0.4rem_0.8rem] uppercase font-[600] text-center",
        ButtonSize::Medium => "text-[1.4rem] p-[1.2rem_1.6rem] font-[500]",
        ButtonSize::Large => "text-[1.6rem] p-[1.2rem_2.4rem] font-[500]",
    };

    view! {
        <button class=tw_merge!(
            "border-none rounded-[var(--border-radius-sm)] shadow-[var(--shadow-sm)]",
            variant_classes, size_classes
        )>{children()}</button>
    }
}
