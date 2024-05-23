use leptos::*;

#[component]
pub fn Input(
    input_type: &'static str,
    id: &'static str,
    disabled: bool,
    value: RwSignal<String>,
) -> impl IntoView {
    view! {
        <input
            type=input_type
            id=id
            disabled=disabled
            class="border border-solid border-[var(--color-grey-300)] bg-[var(--color-grey-0)] rounded-[var(--border-radius-sm)] p-[0.8rem_1.2rem] shadow shadow-[var(--shadow-sm)]"
            prop:value=move || value.get()
            on:input=move |ev| value.set(event_target_value(&ev))
        />
    }
}
