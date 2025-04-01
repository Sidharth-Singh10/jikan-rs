use serde::{Deserialize, Serialize};

use crate::{JikanClient, JikanError, anime::*, common::Pagination};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonResponse {
    pub data: Vec<Anime>,
    pub pagination: Pagination,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonsListResponse {
    pub data: Vec<SeasonInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonInfo {
    pub year: u32,
    pub seasons: Vec<String>,
}

pub enum FilterType {
    None,
    TV,
    Movie,
    OVA,
    Special,
    ONA,
    Music,
}

impl JikanClient {
    /// Returns anime currently airing in the current season
    pub async fn get_season_now(
        &self,
        filter: Option<FilterType>,
        sfw: Option<bool>,
        unapproved: Option<bool>,
        continuing: Option<bool>,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<SeasonResponse, JikanError> {
        let mut params = Vec::new();

        if let Some(f) = filter {
            let filter_str = match f {
                FilterType::None => String::new(),
                FilterType::TV => "tv".to_string(),
                FilterType::Movie => "movie".to_string(),
                FilterType::OVA => "ova".to_string(),
                FilterType::Special => "special".to_string(),
                FilterType::ONA => "ona".to_string(),
                FilterType::Music => "music".to_string(),
            };
            if !filter_str.is_empty() {
                params.push(format!("filter={}", filter_str));
            }
        }

        if let Some(s) = sfw {
            if s {
                params.push("sfw".to_string());
            }
        }

        if let Some(u) = unapproved {
            if u {
                params.push("unapproved".to_string());
            }
        }

        if let Some(c) = continuing {
            if c {
                params.push("continuing".to_string());
            }
        }

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/seasons/now{}", query)).await
    }

    /// Returns anime for the specified season
    pub async fn get_season(
        &self,
        year: u32,
        season: &str,
        filter: Option<FilterType>,
        sfw: Option<bool>,
        unapproved: Option<bool>,
        continuing: Option<bool>,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<SeasonResponse, JikanError> {
        let mut params = Vec::new();

        if let Some(f) = filter {
            let filter_str = match f {
                FilterType::None => String::new(),
                FilterType::TV => "tv".to_string(),
                FilterType::Movie => "movie".to_string(),
                FilterType::OVA => "ova".to_string(),
                FilterType::Special => "special".to_string(),
                FilterType::ONA => "ona".to_string(),
                FilterType::Music => "music".to_string(),
            };
            if !filter_str.is_empty() {
                params.push(format!("filter={}", filter_str));
            }
        }

        if let Some(s) = sfw {
            if s {
                params.push("sfw".to_string());
            }
        }

        if let Some(u) = unapproved {
            if u {
                params.push("unapproved".to_string());
            }
        }

        if let Some(c) = continuing {
            if c {
                params.push("continuing".to_string());
            }
        }

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/seasons/{}/{}{}", year, season, query))
            .await
    }

    /// Returns the list of available seasons
    pub async fn get_seasons_list(&self) -> Result<SeasonsListResponse, JikanError> {
        self.get("/seasons").await
    }

    /// Returns anime airing in the upcoming season
    pub async fn get_season_upcoming(
        &self,
        filter: Option<FilterType>,
        sfw: Option<bool>,
        unapproved: Option<bool>,
        continuing: Option<bool>,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<SeasonResponse, JikanError> {
        let mut params = Vec::new();

        if let Some(f) = filter {
            let filter_str = match f {
                FilterType::None => String::new(),
                FilterType::TV => "tv".to_string(),
                FilterType::Movie => "movie".to_string(),
                FilterType::OVA => "ova".to_string(),
                FilterType::Special => "special".to_string(),
                FilterType::ONA => "ona".to_string(),
                FilterType::Music => "music".to_string(),
            };
            if !filter_str.is_empty() {
                params.push(format!("filter={}", filter_str));
            }
        }

        if let Some(s) = sfw {
            if s {
                params.push("sfw".to_string());
            }
        }

        if let Some(u) = unapproved {
            if u {
                params.push("unapproved".to_string());
            }
        }

        if let Some(c) = continuing {
            if c {
                params.push("continuing".to_string());
            }
        }

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }

        if let Some(l) = limit {
            params.push(format!("limit={}", l));
        }

        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };

        self.get(&format!("/seasons/upcoming{}", query)).await
    }
}
