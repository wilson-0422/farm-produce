use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Harvest {
    pub id: i64,
    pub farmer_id: i64,
    pub farmer_name: String,
    pub product_name: String,
    pub quantity: f64,
    pub unit: String,
    pub price_per_unit: f64,
    pub harvest_date: String,
    pub status: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct HarvestForm {
    pub product_name: String,
    pub quantity: String,
    pub unit: String,
    pub price_per_unit: String,
    pub harvest_date: String,
    pub description: String,
}
