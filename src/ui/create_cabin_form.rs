use crate::model::cabins::Cabin;
use crate::services::api_cabins::create_cabin;
use crate::ui::{
    button::Button,
    file_input::FileInput,
    form::{Form, FormType},
    form_row::FormRow,
    input::Input,
    text_area::TextArea,
};
use crate::ui::toast::{Toast,ToastType};
use leptos_toaster::*;
use leptos::*;
use web_sys::SubmitEvent;

#[component]
pub fn CreateCabinForm() -> impl IntoView {
    let toast_context = expect_context::<Toasts>();

    let create_toast = move |msg: &'static str, toast_type: ToastType| {
        let toast_id = ToastId::new();
        toast_context.toast(
            view! { <Toast msg=msg toast_type=toast_type/> },
            Some(toast_id),
            None
        );
    };

    let create_cabin_action = create_action(|input: &Cabin| {
        let cabin = input.clone();
        async move {create_cabin(cabin).await}
    });

    let loading = create_cabin_action.pending();
    let data = create_cabin_action.value();

    let create_cabin_error = move || data.with(|resp| {
        match resp {
            Some(resp) => {
                match resp {
                    Ok(msg) => Ok(Some(msg.clone())),
                    Err(err) => Err(err.clone()),
                }
            },
            None => Ok(None),
        }
    });

    let cabin_name = create_rw_signal(String::from(""));
    let max_capacity = create_rw_signal(String::from("0"));
    let regular_price = create_rw_signal(String::from("0"));
    let discount = create_rw_signal(String::from("0"));
    let description = create_rw_signal(String::from(""));

    let cabin_name_error = create_rw_signal::<Option<String>>(None);
    let max_capacity_error = create_rw_signal::<Option<String>>(None);
    let regular_price_error = create_rw_signal::<Option<String>>(None);
    let discount_error = create_rw_signal::<Option<String>>(None);
    let description_error = create_rw_signal::<Option<String>>(None);

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let discount_value;
        let regular_price_value;
        let max_capacity_value;

        if cabin_name.get().is_empty() {
            return cabin_name_error.set(Some("Cabin name is required".to_string()));
        }

        if max_capacity.get().is_empty() {
            return max_capacity_error.set(Some("Max capacity is required".to_string()));
        } else {
            let input = max_capacity.get().parse::<u32>();
            match input {
                Ok(value) => {
                    max_capacity_value = value;
                    if value == 0 {
                        return max_capacity_error
                            .set(Some("Capacity should be at least 1".to_string()));
                    }
                }
                Err(_) => {
                    return max_capacity_error.set(Some("Input must be a number".to_string()))
                }
            }
        }

        if regular_price.get().is_empty() {
            return regular_price_error.set(Some(String::from("Regular price is required")));
        } else {
            let input = regular_price.get().parse::<u32>();
            match input {
                Ok(value) => {
                    regular_price_value = value;
                    if value == 0 {
                        return regular_price_error
                            .set(Some("Capacity should be at least 1".to_string()));
                    }
                }
                Err(_) => {
                    return regular_price_error.set(Some("Input must be a number".to_string()))
                }
            }
        }

        if discount.get().is_empty() {
            return discount_error.set(Some(String::from("Discount is required")));
        } else {
            let input = discount.get().parse::<u32>();
            match input {
                Ok(value) => {
                    discount_value = value;
                    if value == 0 {
                        return discount_error
                            .set(Some("Capacity should be at least 1".to_string()));
                    } else {
                        let regular_price = regular_price.get().parse::<u32>();
                        match regular_price {
                            Ok(regular_price) => {
                                if value > regular_price {
                                    return discount_error.set(Some(String::from(
                                        "Discount should be less than regular price",
                                    )));
                                }
                            }
                            Err(_) => {
                                return regular_price_error
                                    .set(Some("Input must be a number".to_string()))
                            }
                        }
                    }
                }
                Err(_) => return discount_error.set(Some("Input must be a number".to_string())),
            }
        }

        if description.get().is_empty() {
            return description_error.set(Some(String::from("Description is required")));
        }

        let cabin = Cabin {
            created_at: None,
            id: None,
            name: cabin_name.get(),
            description: Some(description.get()),
            max_capacity: max_capacity_value,
            discount: discount_value,
            regular_price: regular_price_value,
            image: Some("".to_string()),
        };

        create_cabin_action.dispatch(cabin)
    };

    let clearForm = move || {
        cabin_name.set("".to_string());
        max_capacity.set("0".to_string());
        regular_price.set("0".to_string());
        discount.set("0".to_string());
        description.set("".to_string());
    };

    create_effect(move |_| {
        if cabin_name.get().is_empty() == false {
            cabin_name_error.set(None)
        }
        if max_capacity.get().is_empty() == false {
            max_capacity_error.set(None)
        }
        if regular_price.get().is_empty() == false {
            regular_price_error.set(None)
        }
        if discount.get().is_empty() == false {
            discount_error.set(None)
        }
        if description.get().is_empty() == false {
            description_error.set(None)
        }
    });

    create_effect(move |_| {
        match create_cabin_error() {
            Ok(res) => {
                match res {
                    Some(_) => {
                        create_toast("New cabin succesfully created", ToastType::Success);
                        clearForm()
                    },
                    None => (),
                }
            },
            Err(_) => create_toast("Failed to add cabin", ToastType::Error)
        }
    });

    view! {
        <Form form_type=FormType::NoneModal on_submit=on_submit>
            <FormRow label="Cabin name" error=cabin_name_error id="name">
                <Input input_type="text" id="name" disabled=loading value=cabin_name/>
            </FormRow>

            <FormRow label="Maximum capacity" error=max_capacity_error id="max_capacity">
                <Input input_type="number" id="max_capacity" disabled=loading value=max_capacity/>
            </FormRow>

            <FormRow label="Regular price" error=regular_price_error id="Regular price">
                <Input input_type="number" id="regular_price" disabled=loading value=regular_price/>
            </FormRow>

            <FormRow label="Discount" error=discount_error id="discount">
                <Input input_type="number" id="discount" disabled=loading value=discount/>
            </FormRow>

            <FormRow label="Description for website" error=description_error id="description">
                <TextArea id="description" disabled=loading value=description/>
            </FormRow>

            <FormRow label="Cabin photo" id="image">
                <FileInput id="image" disabled=loading/>
            </FormRow>

            <FormRow label="" id="submit_button">
                <Button
                    variant=crate::ui::button::ButtonVariant::Secondary
                    button_type="button"
                    disabled=loading
                    on_click=move |_| { clearForm() }
                >
                    "Cancel"
                </Button>
                <Button on_click=move |_| {} disabled=loading>
                    "Add cabin"
                </Button>
            </FormRow>
        </Form>
    }
}
