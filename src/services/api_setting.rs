use super::supabase::create_client;
use crate::model::setting::Setting;
use leptos_query::{create_query, QueryOptions, QueryScope};
use serde_json::Error;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct SettingKey;

pub async fn get_setting() -> Result<Setting, String> {
    let client = create_client();
    let resp = client.from("settings").select("*").single().execute().await;

    match resp {
        Ok(response) => {
            let convert_repsonse_err = response.error_for_status();
            match convert_repsonse_err {
                Ok(response) => {
                    let body = response.text().await;
                    match body {
                        Ok(text) => {
                            let setting: Result<Setting, Error> = serde_json::from_str(&text);
                            match setting {
                                Ok(setting) => Ok(setting),
                                Err(err) => Err(err.to_string()),
                            }
                        }
                        Err(err) => Err(err.to_string()),
                    }
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    }
}

pub async fn update_setting(setting: Setting) -> Result<String, String> {
    let client = create_client();
    let setting_json = serde_json::to_string(&setting);

    let create_setting_result = match setting_json {
        Ok(setting_json) => {
            let resp = client
                .from("settings")
                .update(&setting_json)
                .eq("id", "1")
                .execute()
                .await;

            match resp {
                Ok(response) => {
                    let convert_repsonse_err = response.error_for_status();
                    match convert_repsonse_err {
                        Ok(response) => {
                            let body = response.text().await;
                            match body {
                                Ok(text) => {
                                    setting_query().set_query_data(SettingKey, Ok(setting));
                                    setting_query().invalidate_query(SettingKey);
                                    Ok(text)
                                }
                                Err(err) => Err(err.to_string()),
                            }
                        }
                        Err(err) => Err(err.to_string()),
                    }
                }
                Err(err) => Err(err.to_string()),
            }
        }
        Err(err) => Err(err.to_string()),
    };

    create_setting_result
}

pub fn setting_query() -> QueryScope<SettingKey, Result<Setting, String>> {
    create_query(
        move |_| async move { get_setting().await },
        QueryOptions::default(),
    )
}
