use serde::{Deserialize, Serialize};

// schedule.rs
use crate::{JikanClient, JikanError, anime::*, common::Pagination};

pub enum ScheduleFilter {
    None,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
    Unknown,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleResponse {
    pub data: Vec<Anime>,
    pub pagination: Pagination,
}

impl JikanClient {
    pub async fn get_schedules(
        &self,
        filter: Option<ScheduleFilter>,
        kids: Option<bool>,
        sfw: Option<bool>,
        unapproved: Option<bool>,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> Result<ScheduleResponse, JikanError> {
        let mut params = Vec::new();

        if let Some(f) = filter {
            let filter_str = match f {
                ScheduleFilter::None => String::new(),
                ScheduleFilter::Monday => "monday".to_string(),
                ScheduleFilter::Tuesday => "tuesday".to_string(),
                ScheduleFilter::Wednesday => "wednesday".to_string(),
                ScheduleFilter::Thursday => "thursday".to_string(),
                ScheduleFilter::Friday => "friday".to_string(),
                ScheduleFilter::Saturday => "saturday".to_string(),
                ScheduleFilter::Sunday => "sunday".to_string(),
                ScheduleFilter::Unknown => "unknown".to_string(),
                ScheduleFilter::Other => "other".to_string(),
            };
            if !filter_str.is_empty() {
                params.push(format!("filter={}", filter_str));
            }
        }

        if let Some(k) = kids {
            params.push(format!("kids={}", k));
        }

        if let Some(s) = sfw {
            params.push(format!("sfw={}", s));
        }

        if let Some(u) = unapproved {
            if u {
                params.push("unapproved".to_string());
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

        self.get(&format!("/schedules{}", query)).await
    }
}
