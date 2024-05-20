use postgrest::Postgrest;
use serde_json::Error;
use crate::model::cabins::Cabin;

const SUPABASE_PUBLIC_API_KEY: &str="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InpqcmlnY2xld3VnbWpvY29pc3JrIiwicm9sZSI6ImFub24iLCJpYXQiOjE2OTQ1OTQyMjAsImV4cCI6MjAxMDE3MDIyMH0.zVr1KP_mQ4ImlHgCeCfHf5jXWcKYUrWb92vDu4fp9a8";

const SUPABASE_PUBLIC_API_URL: &str="https://zjrigclewugmjocoisrk.supabase.co/rest/v1";

pub async fn get_cabins() -> Result<Vec<Cabin>, String> {
    let client = Postgrest::new(SUPABASE_PUBLIC_API_URL)
        .insert_header("apikey", SUPABASE_PUBLIC_API_KEY)
        .insert_header("Authorization", format!("{} {}", "Bearer ", SUPABASE_PUBLIC_API_KEY));
    
    let resp = client.from("cabins").select("*").execute().await;

    match resp {
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
}
