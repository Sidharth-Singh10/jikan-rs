// producers.rs
use crate::{
    JikanClient, JikanError,
    enums::{common::Sort, producers::ProducersOrder},
    response::Response,
    structs::producers::Producer,
    utils::ExternalEntry,
};

impl JikanClient {
    pub async fn get_producer_by_id(&self, id: u32) -> Result<Response<Producer>, JikanError> {
        self.get(&format!("/producers/{}", id)).await
    }

    pub async fn get_producer_full_by_id(&self, id: u32) -> Result<Response<Producer>, JikanError> {
        self.get(&format!("/producers/{}/full", id)).await
    }

    pub async fn get_producer_external(
        &self,
        id: u32,
    ) -> Result<Response<Vec<ExternalEntry>>, JikanError> {
        self.get(&format!("/producers/{}/external", id)).await
    }

    pub async fn get_producer_search(
        &self,
        page: Option<u32>,
        limit: Option<u32>,
        query: Option<String>,
        order_by: Option<ProducersOrder>,
        sort: Option<Sort>,
        letter: Option<String>,
    ) -> Result<Response<Vec<Producer>>, JikanError> {
        let mut params: Vec<String> = Vec::new();

        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        if let Some(lim) = limit {
            params.push(format!("limit={}", lim));
        }
        if let Some(q) = query {
            if !q.is_empty() {
                params.push(format!("q={}", q));
            }
        }
        if let Some(o) = order_by {
            params.push(format!("order_by={}", o.as_str()));
        }
        if let Some(s) = sort {
            params.push(format!("sort={}", s.as_str()));
        }

        if let Some(l) = letter {
            if l.len() != 1 {
                return Err(JikanError::BadRequest(
                    "Letter must be a single character".to_string(),
                ));
            }
            params.push(format!("letter={}", l));
        }
        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };
        self.get(&format!("/producers{}", query)).await
    }

    pub async fn get_producers(&self) -> Result<Response<Vec<Producer>>, JikanError> {
        self.get("/producers").await
    }
}
