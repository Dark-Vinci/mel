pub mod auth;
pub mod channel;
pub mod extras;
pub mod messaging;

pub struct Pagination {
    pub page_size: u64,
    pub page_number: u64,
    pub size: u64,
}

impl Pagination {
    pub fn page_offset(&self) -> u64 {
        (self.page_number - 1) * self.page_size
    }

    pub fn total_pages(&self, count: u64) -> u64 {
        (count + self.page_size - 1) / self.page_size
    }
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

impl<T> Paginated<T> {
    pub fn new(
        result: T,
        total_pages: u64,
        current: u64,
        page_size: u64,
        total_items: u64,
    ) -> Self {
        Self {
            result,
            total_pages,
            current_page: current,
            page_size,
            total_items,
        }
    }
}
