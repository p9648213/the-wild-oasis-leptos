use leptos::*;

#[component]
pub fn FormRow(
    label: &'static str,
    #[prop(optional)] error: RwSignal<Option<String>>,
    id: &'static str,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="grid items-center grid-cols-[24rem_1fr_1.2fr] gap-[2.4rem] p-[1.2rem_0] form_container">
            <label for=id>{label}</label>
            {children()}
            <span class="text-[1.4rem] text-[var(--color-red-700)]">{move || error.get()}</span>
        </div>
    }
}
