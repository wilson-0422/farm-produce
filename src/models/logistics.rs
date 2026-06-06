use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Logistics {
    pub id: i64,
    pub distribution_id: i64,
    pub channel_name: String,
    pub driver_name: String,
    pub driver_phone: String,
    pub vehicle_plate: String,
    pub departure_time: String,
    pub estimated_arrival: String,
    pub actual_arrival: String,
    pub status: String,
    pub notes: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct LogisticsForm {
    pub distribution_id: String,
    pub driver_name: String,
    pub driver_phone: String,
    pub vehicle_plate: String,
    pub departure_time: String,
    pub estimated_arrival: String,
    pub notes: String,
}
