use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
    pub last_visible_page: u32,
    pub has_next_page: bool,
    pub current_page: Option<u32>,
    pub items: Option<PaginationItems>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationItems {
    pub count: u32,
    pub total: u32,
    pub per_page: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Images {
    pub jpg: Option<ImageSet>,
    pub webp: Option<ImageSet>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageSet {
    pub image_url: Option<String>,
    pub small_image_url: Option<String>,
    pub large_image_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangePropFromTo {
    pub day: Option<u32>,
    pub month: Option<u32>,
    pub year: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRangeProp {
    pub from: DateRangePropFromTo,
    pub to: Option<DateRangePropFromTo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub from: Option<DateTime<Utc>>,
    pub to: Option<DateTime<Utc>>,
    pub prop: DateRangeProp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Title {
    pub r#type: String,
    pub title: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalEntry {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Score {
    pub score: u32,
    pub votes: u32,
    pub percentage: f32,
}
