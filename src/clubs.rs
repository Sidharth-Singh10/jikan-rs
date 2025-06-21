use crate::{
    JikanClient, JikanError,
    enums::{
        clubs::{ClubCategory, ClubOrder, ClubType},
        common::Sort,
    },
    format_search_query,
    response::Response,
    structs::clubs::{Club, ClubMember, ClubRelations, ClubStaff},
};

// New struct for club search parameters
#[derive(Default)]
pub struct ClubSearchParams {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub q: Option<String>,
    pub type_: Option<ClubType>,
    pub category: Option<ClubCategory>,
    pub order_by: Option<ClubOrder>,
    pub sort: Option<Sort>,
    pub letter: Option<String>,
}

// impl ClubSearchParams {
//     pub fn new() -> Self {
//         Self::default()
//     }

//     pub fn with_page(mut self, page: u32) -> Self {
//         self.page = Some(page);
//         self
//     }

//     pub fn with_limit(mut self, limit: u32) -> Self {
//         self.limit = Some(limit);
//         self
//     }

//     pub fn with_query(mut self, q: impl Into<String>) -> Self {
//         self.q = Some(q.into());
//         self
//     }

//     pub fn with_category(mut self, category: impl Into<String>) -> Self {
//         self.category = Some(category.into());
//         self
//     }

//     pub fn with_order_by(mut self, order_by: impl Into<String>) -> Self {
//         self.order_by = Some(order_by.into());
//         self
//     }

//     pub fn with_sort(mut self, sort: impl Into<String>) -> Self {
//         self.sort = Some(sort.into());
//         self
//     }

//     pub fn with_letter(mut self, letter: impl Into<String>) -> Self {
//         self.letter = Some(letter.into());
//         self
//     }

//     fn to_query_params(&self) -> String {
//         let mut params = Vec::new();

//         if let Some(p) = self.page {
//             params.push(format!("page={}", p));
//         }

//         if let Some(l) = self.limit {
//             params.push(format!("limit={}", l));
//         }

//         if let Some(qr) = &self.q {
//             params.push(format!("q={}", qr));
//         }

//         if let Some(c) = &self.category {
//             params.push(format!("category={}", c));
//         }

//         if let Some(ob) = &self.order_by {
//             params.push(format!("order_by={}", ob));
//         }

//         if let Some(s) = &self.sort {
//             params.push(format!("sort={}", s));
//         }

//         if let Some(lt) = &self.letter {
//             params.push(format!("letter={}", lt));
//         }

//         if !params.is_empty() {
//             format!("?{}", params.join("&"))
//         } else {
//             String::new()
//         }
//     }
// }

impl JikanClient {
    pub async fn get_club_by_id(&self, id: u32) -> Result<Response<Club>, JikanError> {
        self.get(&format!("/clubs/{}", id)).await
    }

    pub async fn get_club_members(
        &self,
        id: u32,
        page: Option<u32>,
    ) -> Result<Response<Vec<ClubMember>>, JikanError> {
        let mut params = Vec::new();
        if let Some(p) = page {
            params.push(format!("page={}", p));
        }
        let query = if !params.is_empty() {
            format!("?{}", params.join("&"))
        } else {
            String::new()
        };
        self.get(&format!("/clubs/{}/members{}", id, query)).await
    }

    pub async fn get_club_staff(&self, id: u32) -> Result<Response<Vec<ClubStaff>>, JikanError> {
        self.get(&format!("/clubs/{}/staff", id)).await
    }

    pub async fn get_club_relations(&self, id: u32) -> Result<Response<ClubRelations>, JikanError> {
        self.get(&format!("/clubs/{}/relations", id)).await
    }

    // Refactored search function that takes a struct instead of many parameters
    pub async fn get_club_search(
        &self,
        params: Option<ClubSearchParams>,
    ) -> Result<Response<Vec<Club>>, JikanError> {
        let mut query_params = Vec::new();

        if let Some(p) = params {
            if let Some(q) = p.q {
                let formatted_q = format_search_query(q);
                query_params.push(format!("q={}", formatted_q));
            }

            if let Some(p) = p.page {
                query_params.push(format!("page={}", p));
            }

            if let Some(l) = p.limit {
                query_params.push(format!("limit={}", l));
            }

            if let Some(c) = p.category {
                query_params.push(format!("category={}", c.as_str()));
            }

            if let Some(o) = p.order_by {
                query_params.push(format!("order_by={}", o.as_str()));
            }

            if let Some(s) = p.sort {
                query_params.push(format!("sort={}", s.as_str()));
            }

            if let Some(l) = p.letter {
                query_params.push(format!("letter={}", l));
            }
        }

        let query = format!("?{}", query_params.join("&"));
        self.get(&format!("/clubs{}", query)).await
    }
}
