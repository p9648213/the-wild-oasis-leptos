use crate::{
    hooks::{use_toast::use_toast, use_update_setting::use_update_setting},
    model::setting::Setting,
    services::api_setting::{setting_query, SettingKey},
    ui::{
        form::{Form, FormType},
        form_row::FormRow,
        input::InputOnBlur,
        spinner::Spinner,
        toast::ToastType,
    },
};
use ev::FocusEvent;
use leptos::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlInputElement;

#[component]
pub fn UpdateSettingForm() -> impl IntoView {
    let setting = setting_query().use_query(|| SettingKey);
    let data = setting.data;

    let setting_view = move || {
        data.with(|setting| match setting {
            Some(setting) => match setting {
                Ok(setting) => view! { <SettingForm setting=setting.clone()/> }.into_view(),
                Err(err) => view! { <p>{err.to_string()}</p> }.into_view(),
            },
            None => view! { <Spinner/> }.into_view(),
        })
    };

    setting_view
}

#[component]
fn SettingForm(setting: Setting) -> impl IntoView {
    let min_booking_length = create_rw_signal(setting.min_booking_lenght.to_string());
    let max_booking_length = create_rw_signal(setting.max_booking_length.to_string());
    let max_guests_per_booking = create_rw_signal(setting.max_guests_per_booking.to_string());
    let breakfast_price = create_rw_signal(setting.breakfast_price.to_string());

    let create_toast = use_toast();

    let (updating, update_setting_error, update_setting_action) = use_update_setting();

    let update_new_setting = move |ev: FocusEvent| {
        let input_el = ev
            .target()
            .and_then(|target| Some(target.dyn_into::<HtmlInputElement>()));

        if let Some(result) = input_el {
            match result {
                Ok(input_element) => {
                    let id = input_element.id();
                    match id.as_str() {
                        "min-nights" => {
                            if min_booking_length.get().parse::<u32>().unwrap_or_default()
                                == setting.min_booking_lenght
                            {
                                return;
                            }
                        }
                        "max-nights" => {
                            if max_booking_length.get().parse::<u32>().unwrap_or_default()
                                == setting.max_booking_length
                            {
                                return;
                            }
                        }
                        "max-guests" => {
                            if max_guests_per_booking
                                .get()
                                .parse::<u32>()
                                .unwrap_or_default()
                                == setting.max_guests_per_booking
                            {
                                return;
                            }
                        }
                        "breakfast-price" => {
                            if breakfast_price.get().parse::<u32>().unwrap_or_default()
                                == setting.breakfast_price
                            {
                                return;
                            }
                        }
                        _ => return,
                    }
                }
                Err(_) => return,
            }

            let new_setting = Setting {
                id: None,
                created_at: None,
                min_booking_lenght: min_booking_length.get().parse().unwrap_or_default(),
                max_booking_length: max_booking_length.get().parse().unwrap_or_default(),
                max_guests_per_booking: max_guests_per_booking.get().parse().unwrap_or_default(),
                breakfast_price: breakfast_price.get().parse().unwrap_or_default(),
            };

            update_setting_action.dispatch(new_setting);
        } else {
            return;
        };
    };

    create_effect(move |_| match update_setting_error() {
        Ok(res) => match res {
            Some(_) => create_toast("Setting succesfully edited", ToastType::Success),
            None => (),
        },
        Err(_) => create_toast("Failed to edit setting", ToastType::Error),
    });

    view! {
        <Form form_type=FormType::NoneModal on_submit=move |_| {}>
            <FormRow label="Minimum nights/booking" id="min-nights">
                <InputOnBlur
                    input_type="number"
                    id="min-nights"
                    disabled=updating
                    value=min_booking_length
                    on_blur=update_new_setting
                />
            </FormRow>
            <FormRow label="Maximum nights/booking" id="max-nights">
                <InputOnBlur
                    input_type="number"
                    id="max-nights"
                    disabled=updating
                    value=max_booking_length
                    on_blur=update_new_setting
                />
            </FormRow>
            <FormRow label="Maximum guests/booking" id="max-guests">
                <InputOnBlur
                    input_type="number"
                    id="max-guests"
                    disabled=updating
                    value=max_guests_per_booking
                    on_blur=update_new_setting
                />
            </FormRow>
            <FormRow label="Breakfast price" id="breakfast-price">
                <InputOnBlur
                    input_type="number"
                    id="breakfast-price"
                    disabled=updating
                    value=breakfast_price
                    on_blur=update_new_setting
                />
            </FormRow>
        </Form>
    }
}
