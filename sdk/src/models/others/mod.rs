pub mod auth;
pub mod extras;

pub struct Pagination {
    pub page_size: u64,
    pub page_number: u64,
    pub size: u64,
}

// pub struct PaginationResult {
//     total_pages: u64,
//     current_page: u64,
//     page_size: u64,
//     total_items: u64,
// }

pub struct Paginated<T> {
    pub result: T,
    pub total_pages: u64,
    pub current_page: u64,
    pub page_size: u64,
    pub total_items: u64,
}
