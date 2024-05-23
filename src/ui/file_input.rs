use leptos::*;

#[component]
pub fn FileInput(id: &'static str, disabled: ReadSignal<bool>) -> impl IntoView {
    view! { <input type="file" disabled=move || disabled id=id accept="image/*" class="file_input"/> }
}
