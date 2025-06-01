use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, FromForm, Deserialize)]
pub struct PaginationParams {
    #[field(default = 1)]
    pub page: u32,
    #[field(default = 10)]
    pub limit: u32,
}

impl Default for PaginationParams {
    fn default() -> Self {
        Self {
            page: 1,
            limit: 10
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub total: u32,
    pub page: u32,
    pub limit: u32,
    pub pages: u32
}

impl<T> PaginatedResponse<T> {
    pub fn new(items: Vec<T>, total: u32, params: &PaginationParams) -> Self {
        let pages = (total as f64 / params.limit as f64).ceil() as u32;
        Self {
            items,
            total,
            page: params.page,
            limit: params.limit,
            pages
        }
    }
}