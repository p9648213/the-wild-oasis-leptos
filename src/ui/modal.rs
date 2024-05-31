use icondata::HiXMarkOutlineLg;
use leptos::*;
use leptos_icons::Icon;

#[component]
pub fn Modal(children: Children) -> impl IntoView {
    let (open_name, set_open_name) = create_signal("");

    run_as_child(move || {
        provide_context(open_name);
        provide_context(set_open_name);

        view! { <span>{children()}</span> }
    })
}

#[component]
pub fn ModalOpen(children: Children, open_windown_name: &'static str) -> impl IntoView {
    let set_open_name = use_context::<WriteSignal<&str>>().expect("set_open_name is not provided");

    view! { <span on:click=move |_| set_open_name(open_windown_name)>{children()}</span> }
}

#[component]
pub fn ModalWindow(children: ChildrenFn, name: &'static str) -> impl IntoView {
    let open_name = use_context::<ReadSignal<&str>>().expect("open_name is not provided");
    let set_open_name = use_context::<WriteSignal<&str>>().expect("set_open_name is not provided");
    let children = store_value(children);

    create_effect(move |_| leptos::logging::log!("{:#?} {}", open_name.get(), name));

    view! {
        <Portal>
            <Show when=move || { open_name.get() == name }>
                <div class="fixed top-0 left-0 w-full h-screen bg-[var(--backdrop-color)] backdrop-blur-sm z-[1000] transition-all duration-[0.5s]">
                    <div class="fixed top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] bg-[var(--color-grey-0)] rounded-[var(--border-radius-lg)] shadow p-[3.2rem_4rem] transition-all duration-[0.5s]">
                        <button
                            on:click=move |_| set_open_name("")
                            class="bg-none border-none p-[0.4rem] rounded-[var(--border-radius-sm)] translate-x-[0.8rem] transition-all duration-[0.2s] absolute top-[1.2rem] right-[1.9rem] hover:bg-[var(--color-grey-100)] modal-button"
                        >
                            <Icon icon=HiXMarkOutlineLg/>
                        </button>
                        <div>{children.with_value(|children| children())}</div>
                    </div>
                </div>
            </Show>
        </Portal>
    }
}
