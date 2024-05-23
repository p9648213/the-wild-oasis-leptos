use leptos::*;

#[component]
pub fn TextArea(id: &'static str, disabled: bool, value: RwSignal<String>) -> impl IntoView {
    view! {
        <textarea
            id=id
            disabled=disabled
            prop:value=move || value.get()
            on:input=move |ev| value.set(event_target_value(&ev))
            class="p-[0.8rem_1.2rem] border border-solid border-[var(--color-grey-300)] rounded-[5px] bg-[var(--color-grey-0)] shadow shadow-[var(--shadow-sm)] w-full h-[8rem]"
        >
            {value.get_untracked()}
        </textarea>
    }
}
