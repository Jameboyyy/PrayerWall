
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct PrayerWall {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub user_id: i32,
}