use leptos_query::*;
use serde_json::Error;
use crate::model::cabins::Cabin;
use crate::services::supabase::create_client;

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

pub async fn create_cabin(cabin:Cabin)-> Result<String, String>{
    let client = create_client();
    let cabin_json = serde_json::to_string(&cabin);
    match cabin_json {
        Ok(cabin_json) => {
            let resp = client.from("cabins").insert(&cabin_json).execute().await;
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
        },
        Err(err) => Err(err.to_string()),
    }
}

pub fn all_cabins_query() -> QueryScope<AllCabinsKey, Result<Vec<Cabin>, String>> {
    create_query(move |_| async move { get_cabins().await },  QueryOptions::default())
}