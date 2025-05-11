use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genre {
    pub mal_id: u32,
    pub name: String,
    pub url: String,
    pub count: u32,
}
