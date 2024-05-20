use num_format::{Locale, ToFormattedString};

pub fn format_currency(value: u32) -> String {
    let formatted_value = value.to_formatted_string(&Locale::en);
    format!("${}", formatted_value)
}
