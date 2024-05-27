use crate::ui::create_cabin_form::CabinAction;
use leptos::logging;
use leptos_query::*;
use serde_json::Error;
use wasm_bindgen::JsValue;
use crate::model::cabins::Cabin;
use crate::services::supabase::create_client;
use web_sys::File;
use reqwest::multipart::{Form, Part};
use wasm_bindgen_futures::JsFuture;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct AllCabinsKey;

pub async fn get_cabins() -> Result<Vec<Cabin>, String> {
    let client = create_client();
    
    let resp = client.from("cabins").select("*").execute().await;

    match resp {
        Ok(response) => {
            let convert_repsonse_err = response.error_for_status();
            match convert_repsonse_err {
                Ok(response) => {
                    let body = response.text().await;
                    match body {
                        Ok(text) => {
                            let cabins: Result<Vec<Cabin>, Error> = serde_json::from_str(&text);
                            match cabins {
                                Ok(cabins) => Ok(cabins),
                                Err(err) => Err(err.to_string()),
                            }
                        },
                        Err(err) => Err(err.to_string()),
                    }
                },
                Err(err) => Err(err.to_string()),
            }
        },
        Err(err) => Err(err.to_string()),
    }
}

pub async fn delete_cabin(id: u32) -> Result<String, String>{
    let client = create_client();

    let resp = client.from("cabins").delete().eq("id", id.to_string()).execute().await;

    match resp {
        Ok(response) => {
            let convert_repsonse_err = response.error_for_status();
            match convert_repsonse_err {
                Ok(response) => {
                    let body = response.text().await;
                    match body {
                        Ok(text) => {
                            all_cabins_query().invalidate_query(AllCabinsKey);
                            Ok(text)
                        },
                        Err(err) => Err(err.to_string()),
                    }
                },
                Err(err) => Err(err.to_string())
            } 
        },
        Err(err) => Err(err.to_string()),
    }
}

pub async fn create_cabin(data: CabinAction, edit: bool)-> Result<String, String>{
    let cabin = data.cabin;
    let image = data.image_file;

    let client = create_client();
    let cabin_json = serde_json::to_string(&cabin);

    let create_cabin_result = match cabin_json {
        Ok(cabin_json) => {
            let resp = match edit {
                true => client.from("cabins").update(&cabin_json).eq("id", &cabin.id.unwrap_or_default().to_string()).execute().await,
                false => client.from("cabins").insert(&cabin_json).execute().await,
            };
            match resp {
                Ok(response) => {
                    let convert_repsonse_err = response.error_for_status();
                    match convert_repsonse_err {
                        Ok(response) => {
                            let body = response.text().await;
                            match body {
                                Ok(text) => {
                                    if let None = image {
                                        all_cabins_query().invalidate_query(AllCabinsKey);
                                    }
                                    Ok(text)
                                },
                                Err(err) => Err(err.to_string()),
                            }
                        },
                        Err(err) => Err(err.to_string())
                    } 
                },
                Err(err) => Err(err.to_string()),
            }
        },
        Err(err) => Err(err.to_string()),
    };

    if let Some(image_file) = image{
        let res = upload_image(image_file).await;
        let res = match res {
            Ok(_) => Ok("Image upload successfully".to_string()),
            Err(_) => Err("Failed to upload image".to_string()),
        };
        all_cabins_query().invalidate_query(AllCabinsKey);
        res
    } else {
        create_cabin_result
    }
}

pub async fn upload_image(file: File) -> Result<String, String> {
    let array_buffer_promise: JsFuture = file
        .array_buffer()
        .into();

    let array_buffer: JsValue = array_buffer_promise
        .await
        .expect("Could not get ArrayBuffer from file");

    let file_u8 = js_sys::Uint8Array
        ::new(&array_buffer)
        .to_vec();

    let client = reqwest::Client::new();
    let form= Form::new().part("file", Part::stream(file_u8).file_name(file.name())
        .mime_str(&file.type_())
        .map_err(|e| e.to_string())?);

    let response = client
        .post("http://localhost:3000/upload")
        .multipart(form)
        .send()
        .await;

    match response {
        Ok(response) => {
            let text = response.text().await.unwrap_or("".to_string());
            Ok(text)
        },
        Err(err) => Err(err.to_string()),
    }
}

pub fn all_cabins_query() -> QueryScope<AllCabinsKey, Result<Vec<Cabin>, String>> {
    create_query(move |_| async move { get_cabins().await },  QueryOptions::default())
}