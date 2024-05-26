use postgrest::Postgrest;

pub const SUPABASE_PUBLIC_API_KEY: &str="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InpqcmlnY2xld3VnbWpvY29pc3JrIiwicm9sZSI6ImFub24iLCJpYXQiOjE2OTQ1OTQyMjAsImV4cCI6MjAxMDE3MDIyMH0.zVr1KP_mQ4ImlHgCeCfHf5jXWcKYUrWb92vDu4fp9a8";

pub const SUPABASE_PUBLIC_API_URL: &str = "https://zjrigclewugmjocoisrk.supabase.co/rest/v1";
pub const SUPABASE_STORAGE_IMAGE_URL: &str =
    "https://zjrigclewugmjocoisrk.supabase.co/storage/v1/object/public/cabin-images";

pub fn create_client() -> Postgrest {
    Postgrest::new(SUPABASE_PUBLIC_API_URL)
        .insert_header("apikey", SUPABASE_PUBLIC_API_KEY)
        .insert_header(
            "Authorization",
            format!("{} {}", "Bearer ", SUPABASE_PUBLIC_API_KEY),
        )
}
