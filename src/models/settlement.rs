use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settlement {
    pub id: i64,
    pub distribution_id: i64,
    pub harvest_id: i64,
    pub farmer_id: i64,
    pub farmer_name: String,
    pub amount: f64,
    pub status: String,
    pub payment_method: String,
    pub paid_at: String,
    pub notes: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct SettlementForm {
    pub distribution_id: String,
    pub harvest_id: String,
    pub amount: String,
    pub payment_method: String,
    pub notes: String,
}
