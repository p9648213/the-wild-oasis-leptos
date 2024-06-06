use ev::MouseEvent;
use icondata::HiEllipsisVerticalOutlineLg;
use leptos::*;
use leptos_icons::Icon;
use leptos_use::on_click_outside;
use wasm_bindgen::JsCast;
use web_sys::{window, Element};

#[derive(Clone, Copy)]
struct Position {
    x: f64,
    y: f64,
}

#[derive(Clone)]
struct MenuOpenId(ReadSignal<String>);
#[derive(Clone)]
struct MenuOpenIdSetter(WriteSignal<String>);
#[derive(Clone)]
struct MenuPosition(ReadSignal<Position>);
#[derive(Clone)]
struct MenuPositionSetter(WriteSignal<Position>);

#[component]
pub fn Menus(children: Children) -> impl IntoView {
    let (open_id, set_open_id) = create_signal(String::from(""));
    let (position, set_position) = create_signal(Position { x: 0.0, y: 0.0 });

    run_as_child(move || {
        provide_context(MenuOpenId(open_id));
        provide_context(MenuOpenIdSetter(set_open_id));
        provide_context(MenuPosition(position));
        provide_context(MenuPositionSetter(set_position));

        view! { <div>{children()}</div> }
    })
}

#[component]
pub fn MenusToggle(id: String) -> impl IntoView {
    let open_id = use_context::<MenuOpenId>()
        .expect("open_id is not provided")
        .0;
    let set_open_id = use_context::<MenuOpenIdSetter>()
        .expect("set_open_id is not provided")
        .0;
    let set_position = use_context::<MenuPositionSetter>()
        .expect("set_position is not provided")
        .0;

    let handle_click = move |ev: MouseEvent| {
        if let Some(target) = ev.target() {
            if let Ok(element) = target.dyn_into::<Element>() {
                if let Some(button) = element.closest("button").unwrap() {
                    let rect = button.get_bounding_client_rect();
                    let x = window().unwrap().inner_width().unwrap().as_f64().unwrap()
                        - rect.width()
                        - rect.x();
                    let y = rect.y() + rect.height() + 8.0;

                    set_position.update(|pos| {
                        pos.x = x;
                        pos.y = y;
                    });

                    if open_id.get() == "" || open_id.get() != id {
                        set_open_id(id.clone());
                    } else {
                        set_open_id("".to_string());
                    }
                }
            }
        }
    };

    view! {
        <button
            class="bg-none border-none p-[0.4rem] rounded-[var(--border-radius-sm)] translate-x-[0.8rem] transition-all duration-[0.2s] hover:bg-[var(--color-grey-100)] toggle-icon"
            on:click=handle_click
        >
            <Icon icon=HiEllipsisVerticalOutlineLg/>
        </button>
    }
}

#[component]
pub fn MenusList(children: ChildrenFn, id: String) -> impl IntoView {
    let children = store_value(children);
    let node_ref = create_node_ref::<html::Ul>();

    let open_id = use_context::<MenuOpenId>()
        .expect("open_id is not provided")
        .0;
    let set_open_id = use_context::<MenuOpenIdSetter>()
        .expect("set_open_id is not provided")
        .0;
    let position = use_context::<MenuPosition>()
        .expect("position is not provided")
        .0;

    let _ = on_click_outside(node_ref, move |_| {
        set_open_id("".to_string());
    });

    let id_cloned = create_memo(move |_| id.clone());

    view! {
        <Portal>
            <Show when=move || { open_id.get() == id_cloned() }>
                <ul
                    style:right=move || format!("{}px", position.get().x)
                    style:top=move || format!("{}px", position.get().y)
                    class="fixed bg-[var(--color-grey-0)] shadow-lg rounded-[var(--border-radius-md)]"
                    node_ref=node_ref
                >
                    {children.with_value(|children| children())}
                </ul>
            </Show>
        </Portal>
    }
}

#[component]
pub fn MenusButton<T: Fn() -> () + 'static>(children: Children, on_click: T) -> impl IntoView {
    let set_open_id = use_context::<MenuOpenIdSetter>()
        .expect("set_open_id is not provided")
        .0;

    let handle_click = move |_| {
        on_click();
        set_open_id("".to_string())
    };

    view! {
        <li>
            <button
                on:click=handle_click
                class="w-full text-left bg-none border-none p-[1.2rem_2.4rem] text-[1.4rem] transition-all duration-[0.2s] flex items-center gap-[1.6rem] hover:bg-[var(--color-grey-50)] menu-button"
            >
                {children()}
            </button>
        </li>
    }
}
