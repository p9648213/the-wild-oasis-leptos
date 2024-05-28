use crate::{services::api_cabins::create_cabin, ui::create_cabin_form::CabinAction};
use leptos::{create_action, ReadSignal, SignalWith};

pub fn use_create_cabin(
    is_edit_session: bool,
) -> (
    ReadSignal<bool>,
    impl Fn() -> Result<Option<String>, String>,
    impl Fn(CabinAction),
) {
    let create_cabin_action = create_action(move |input: &CabinAction| {
        let cabin_data = input.clone();
        async move { create_cabin(cabin_data, is_edit_session).await }
    });

    let creating = create_cabin_action.pending();
    let data = create_cabin_action.value();

    let create_cabin_error = move || {
        data.with(|resp| match resp {
            Some(resp) => match resp {
                Ok(msg) => Ok(Some(msg.clone())),
                Err(err) => Err(err.clone()),
            },
            None => Ok(None),
        })
    };

    let create_cabin =
        move |cabin_action_input: CabinAction| create_cabin_action.dispatch(cabin_action_input);

    (creating, create_cabin_error, create_cabin)
}
