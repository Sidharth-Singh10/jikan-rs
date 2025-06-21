use crate::utils::{ExternalEntry, Images, Title};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Producer {
    pub mal_id: u32,
    pub url: String,
    pub titles: Vec<Title>,
    pub images: Images,
    pub favorites: u32,
    pub count: u32,
    pub established: Option<String>,
    pub about: Option<String>,
    pub external: Option<Vec<ExternalEntry>>,
}
