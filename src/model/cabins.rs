use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Cabin {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    pub name: String,
    #[serde(rename = "maxCapacity")]
    pub max_capacity: u32,
    #[serde(rename = "regularPrice")]
    pub regular_price: u32,
    pub discount: u32,
    pub description: Option<String>,
    pub image: Option<String>,
}
