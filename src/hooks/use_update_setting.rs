use crate::{model::setting::Setting, services::api_setting::update_setting};
use leptos::{create_action, Action, ReadSignal, SignalWith};

pub fn use_update_setting() -> (
    ReadSignal<bool>,
    impl Fn() -> Result<Option<String>, String>,
    Action<Setting, Result<String, String>>,
) {
    let update_setting_action = create_action(move |input: &Setting| {
        let setting = input.clone();
        async move { update_setting(setting).await }
    });

    let updating = update_setting_action.pending();
    let data = update_setting_action.value();

    let update_setting_error = move || {
        data.with(|resp| match resp {
            Some(resp) => match resp {
                Ok(msg) => Ok(Some(msg.clone())),
                Err(err) => Err(err.clone()),
            },
            None => Ok(None),
        })
    };

    (updating, update_setting_error, update_setting_action)
}
