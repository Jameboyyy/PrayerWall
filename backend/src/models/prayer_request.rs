
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct PrayerRequest {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub user_id: i32,
}