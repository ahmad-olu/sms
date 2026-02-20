use serde::{Deserialize, Serialize};

// pagination.rs
#[derive(Debug, Serialize, Deserialize)]
pub struct Page {
    pub limit: u32,
    pub offset: u32, // or cursor-based
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PaginatedResult<T> {
    pub data: Vec<T>,
    pub total: u64,
    pub limit: u32,
    pub offset: u32,
    pub has_next: bool,
}

// Usage pattern in queries:
// LIMIT $limit START $offset
