use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Setting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(rename = "minBookingLenght")]
    pub min_booking_lenght: u32,
    #[serde(rename = "maxBookingLength")]
    pub max_booking_length: u32,
    #[serde(rename = "maxGuestsPerBooking")]
    pub max_guests_per_booking: u32,
    #[serde(rename = "breakfastPrice")]
    pub breakfast_price: u32,
}
