use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WarehouseEntry {
    pub id: i64,
    pub harvest_id: i64,
    pub product_name: String,
    pub keeper_id: i64,
    pub keeper_name: String,
    pub location: String,
    pub quantity_stored: f64,
    pub intake_date: String,
    pub status: String,
    pub notes: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct WarehouseIntakeForm {
    pub harvest_id: String,
    pub location: String,
    pub quantity_stored: String,
    pub notes: String,
}
