use crate::services::api_cabins::delete_cabin;
use leptos::{create_action, ReadSignal, SignalWith};

pub fn use_delete_cabin() -> (
    ReadSignal<bool>,
    impl Fn() -> Result<Option<String>, String>,
    impl Fn(u32),
) {
    let delete_cabin_action = create_action(|input: &u32| {
        let id = input.clone();
        async move { delete_cabin(id).await }
    });

    let deleting = delete_cabin_action.pending();
    let data = delete_cabin_action.value();

    let delete_error = move || {
        data.with(|resp| match resp {
            Some(resp) => match resp {
                Ok(msg) => Ok(Some(msg.clone())),
                Err(err) => Err(err.clone()),
            },
            None => Ok(None),
        })
    };

    let delete_cabin_by_id = move |id: u32| delete_cabin_action.dispatch(id);

    (deleting, delete_error, delete_cabin_by_id)
}
