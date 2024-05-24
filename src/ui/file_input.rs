use leptos::*;
use web_sys::{Event, File};

#[component]
pub fn FileInput(
    id: &'static str,
    disabled: ReadSignal<bool>,
    value: RwSignal<Option<File>>,
    error: RwSignal<Option<String>>,
) -> impl IntoView {
    let input_element: NodeRef<html::Input> = create_node_ref();

    let onchange = move |_: Event| {
        match input_element.get() {
            Some(files) => match files.files() {
                Some(files) => match files.get(0) {
                    Some(file) => value.set(Some(file)),
                    None => error.set(Some("Can't get file".to_string())),
                },
                None => error.set(Some("<input> should be file type".to_string())),
            },
            None => error.set(Some("<input> should be mounted".to_string())),
        };
    };

    view! {
        <input
            type="file"
            disabled=disabled
            id=id
            accept="image/*"
            class="file_input"
            on:change=onchange
            node_ref=input_element
        />
    }
}
