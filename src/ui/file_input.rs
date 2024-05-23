use leptos::*;

#[component]
pub fn FileInput(id: &'static str, disabled: bool) -> impl IntoView {
    view! { <input type="file" disabled=disabled id=id accept="image/*" class="file_input"/> }
}
