use crate::{common::utils::Images, utils::Pagination};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
    pub pagination: Option<Pagination>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalCommonTypeResponse {
    pub mal_id: u32,
    pub r#type: Option<String>,
    pub name: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MalCommonImageResponse {
    pub mal_id: u32,
    pub url: String,
    pub images: Option<Images>,
    pub name: String,
}