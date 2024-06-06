use crate::hooks::use_create_cabin::use_create_cabin;
use crate::hooks::use_toast::use_toast;
use crate::model::cabins::Cabin;
use crate::services::supabase::SUPABASE_STORAGE_IMAGE_URL;
use crate::ui::toast::ToastType;
use crate::ui::{
    button::Button,
    file_input::FileInput,
    form::{Form, FormType},
    form_row::FormRow,
    input::Input,
    text_area::TextArea,
};
use leptos::*;
use web_sys::{File, SubmitEvent};

#[derive(Clone)]
pub struct CabinAction {
    pub cabin: Cabin,
    pub image_file: Option<File>,
}

struct DefaultCabinValue {
    cabin_name: String,
    max_capacity: String,
    regular_price: String,
    discount: String,
    description: String,
}

#[component]
pub fn CreateCabinForm(#[prop(default = None)] cabin: Option<Cabin>) -> impl IntoView {
    let set_open_name = use_context::<WriteSignal<&str>>().expect("set_open_name is not provided");

    let is_edit_session = match &cabin {
        Some(_) => true,
        None => false,
    };

    let default_value = match &cabin {
        Some(cabin_value) => DefaultCabinValue {
            cabin_name: cabin_value.name.clone(),
            max_capacity: cabin_value.max_capacity.to_string(),
            description: cabin_value.description.clone().unwrap_or_default(),
            discount: cabin_value.discount.to_string(),
            regular_price: cabin_value.regular_price.to_string(),
        },
        None => DefaultCabinValue {
            cabin_name: String::from(""),
            max_capacity: String::from(""),
            description: String::from(""),
            discount: String::from(""),
            regular_price: String::from(""),
        },
    };

    let cabin_name = create_rw_signal(default_value.cabin_name);
    let max_capacity = create_rw_signal(default_value.max_capacity);
    let regular_price = create_rw_signal(default_value.regular_price);
    let discount = create_rw_signal(default_value.discount);
    let description = create_rw_signal(default_value.description);
    let image = create_rw_signal::<Option<File>>(None);

    let cabin_name_error = create_rw_signal::<Option<String>>(None);
    let max_capacity_error = create_rw_signal::<Option<String>>(None);
    let regular_price_error = create_rw_signal::<Option<String>>(None);
    let discount_error = create_rw_signal::<Option<String>>(None);
    let description_error = create_rw_signal::<Option<String>>(None);
    let image_error = create_rw_signal::<Option<String>>(None);

    let form_element: NodeRef<html::Form> = create_node_ref();

    let create_toast = use_toast();

    let (creating, create_cabin_error, create_cabin) = use_create_cabin(is_edit_session);

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

        let image_file = image.get();

        let cabin = Cabin {
            created_at: None,
            id: match is_edit_session {
                true => Some(cabin.clone().unwrap().id.unwrap()),
                false => None,
            },
            name: cabin_name.get(),
            description: Some(description.get()),
            max_capacity: max_capacity_value,
            discount: discount_value,
            regular_price: regular_price_value,
            image: match &image_file {
                Some(image_file) => Some(format!(
                    "{}/{}",
                    SUPABASE_STORAGE_IMAGE_URL,
                    image_file.name().replace("/", "")
                )),
                None => None,
            },
        };

        let cabin_action_input = CabinAction { cabin, image_file };

        create_cabin(cabin_action_input)
    };

    let clearForm = move || {
        match form_element.get() {
            Some(form) => form.reset(),
            None => (),
        };
        cabin_name.set("".to_string());
        max_capacity.set("".to_string());
        regular_price.set("".to_string());
        discount.set("".to_string());
        description.set("".to_string());
        image.set(None);
        set_open_name("");
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
        if image.get().is_none() {
            image_error.set(None)
        }
    });

    create_effect(move |_| match create_cabin_error() {
        Ok(res) => match res {
            Some(_) => {
                match form_element.get() {
                    Some(form) => form.reset(),
                    None => (),
                };
                match is_edit_session {
                    true => create_toast("Cabin succesfully edited", ToastType::Success),
                    false => create_toast("New cabin succesfully created", ToastType::Success),
                };
                clearForm();
            }
            None => (),
        },
        Err(_) => create_toast("Failed to add cabin", ToastType::Error),
    });

    view! {
        <Form form_ref=form_element on_submit=on_submit form_type=FormType::Modal>

            <FormRow label="Cabin name" error=cabin_name_error id="name">
                <Input input_type="text" id="name" disabled=creating value=cabin_name/>
            </FormRow>

            <FormRow label="Maximum capacity" error=max_capacity_error id="max_capacity">
                <Input input_type="number" id="max_capacity" disabled=creating value=max_capacity/>
            </FormRow>

            <FormRow label="Regular price" error=regular_price_error id="Regular price">
                <Input
                    input_type="number"
                    id="regular_price"
                    disabled=creating
                    value=regular_price
                />
            </FormRow>

            <FormRow label="Discount" error=discount_error id="discount">
                <Input input_type="number" id="discount" disabled=creating value=discount/>
            </FormRow>

            <FormRow label="Description for website" error=description_error id="description">
                <TextArea id="description" disabled=creating value=description/>
            </FormRow>

            <FormRow label="Cabin photo" error=image_error id="image">
                <FileInput id="image" disabled=creating value=image error=image_error/>
            </FormRow>

            <FormRow label="" id="submit_button">
                <Button
                    variant=crate::ui::button::ButtonVariant::Secondary
                    button_type="button"
                    disabled=creating
                    on_click=move |_| { clearForm() }
                >
                    "Cancel"
                </Button>
                <Button on_click=move |_| {} disabled=creating>
                    {if is_edit_session == true { "Edit cabin" } else { "Create new cabin" }}
                </Button>
            </FormRow>
        </Form>
    }
}
