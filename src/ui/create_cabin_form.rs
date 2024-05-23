use crate::ui::{
    button::Button,
    file_input::FileInput,
    form::{Form, FormType},
    form_row::FormRow,
    input::Input,
    text_area::TextArea,
};
use leptos::*;
use web_sys::SubmitEvent;

#[component]
pub fn CreateCabinForm() -> impl IntoView {
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

        if cabin_name.get().is_empty() {
            cabin_name_error.set(Some("Cabin name is required".to_string()))
        }

        if max_capacity.get().is_empty() {
            max_capacity_error.set(Some("Max capacity is required".to_string()))
        } else {
            let input = max_capacity.get().parse::<u32>();
            match input {
                Ok(value) => {
                    if value == 0 {
                        max_capacity_error.set(Some("Capacity should be at least 1".to_string()))
                    }
                }
                Err(_) => max_capacity_error.set(Some("Input must be a number".to_string())),
            }
        }

        if regular_price.get().is_empty() {
            regular_price_error.set(Some(String::from("Regular price is required")))
        } else {
            let input = regular_price.get().parse::<u32>();
            match input {
                Ok(value) => {
                    if value == 0 {
                        regular_price_error.set(Some("Capacity should be at least 1".to_string()))
                    }
                }
                Err(_) => regular_price_error.set(Some("Input must be a number".to_string())),
            }
        }

        if discount.get().is_empty() {
            discount_error.set(Some(String::from("Discount is required")))
        } else {
            let input = discount.get().parse::<u32>();
            match input {
                Ok(value) => {
                    if value == 0 {
                        discount_error.set(Some("Capacity should be at least 1".to_string()))
                    }
                }
                Err(_) => discount_error.set(Some("Input must be a number".to_string())),
            }
        }

        if description.get().is_empty() {
            description_error.set(Some(String::from("Description is required")))
        }
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

    view! {
        <Form form_type=FormType::NoneModal on_submit=on_submit>
            <FormRow label="Cabin name" error=cabin_name_error id="name">
                <Input input_type="text" id="name" disabled=false value=cabin_name/>
            </FormRow>

            <FormRow label="Maximum capacity" error=max_capacity_error id="max_capacity">
                <Input input_type="number" id="max_capacity" disabled=false value=max_capacity/>
            </FormRow>

            <FormRow label="Regular price" error=regular_price_error id="Regular price">
                <Input input_type="number" id="regular_price" disabled=false value=regular_price/>
            </FormRow>

            <FormRow label="Discount" error=discount_error id="discount">
                <Input input_type="number" id="discount" disabled=false value=discount/>
            </FormRow>

            <FormRow label="Description for website" error=description_error id="description">
                <TextArea id="description" disabled=false value=description/>
            </FormRow>

            <FormRow label="Cabin photo" id="image">
                <FileInput id="image" disabled=false/>
            </FormRow>

            <FormRow label="" id="submit_button">
                <Button
                    variant=crate::ui::button::ButtonVariant::Secondary
                    button_type="reset"
                    on_click=move |_| {}
                >
                    "Cancel"
                </Button>
                <Button on_click=move |_| {}>"Add cabin"</Button>
            </FormRow>
        </Form>
    }
}
