use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cabin {
    pub id: u32,
    pub created_at: String,
    pub name: String,
    #[serde(rename = "maxCapacity")]
    pub max_capacity: u32,
    #[serde(rename = "regularPrice")]
    pub regular_price: u32,
    pub discount: u32,
    pub description: Option<String>,
    pub image: Option<String>,
}
