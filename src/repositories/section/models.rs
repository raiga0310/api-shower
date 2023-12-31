use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq, Eq, sqlx::FromRow)]
pub struct Section {
    pub id: i32,
    pub gender: String,
    pub building: String,
    pub floor: i32,
    pub total: i32,
    pub available: i32,
    pub occupied: i32,
    pub disabled_rooms: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SectionInfo {
    pub gender: String,
    pub building: String,
    pub floor: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreateSection {
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateSection {
    pub id: i32,
    pub current_status: String,
    pub next_status: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdatePayload {
    pub current_status: String,
    pub next_status: String,
}

pub struct Usage {
    pub available: i32,
    pub occupied: i32,
    pub disabled_rooms: i32,
}

impl Section {
    pub fn new(id: i32, gender: String, building: String, floor: i32, total: i32) -> Self {
        Self {
            id,
            gender,
            building,
            floor,
            total,
            available: total,
            occupied: 0,
            disabled_rooms: 0,
        }
    }
}
