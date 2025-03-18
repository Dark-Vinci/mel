use {crate::connections::db::DB, async_trait::async_trait};

#[async_trait]
pub trait HistoryRepository {}

pub struct HistoryRepo(DB);

impl HistoryRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl HistoryRepository for HistoryRepo {}
