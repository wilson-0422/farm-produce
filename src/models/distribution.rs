use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Distribution {
    pub id: i64,
    pub channel_name: String,
    pub contact_person: String,
    pub contact_phone: String,
    pub product_name: String,
    pub quantity: f64,
    pub unit_price: f64,
    pub total_amount: f64,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct DistributionForm {
    pub channel_name: String,
    pub contact_person: String,
    pub contact_phone: String,
    pub product_name: String,
    pub quantity: String,
    pub unit_price: String,
}
