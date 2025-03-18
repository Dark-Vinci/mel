// use sdk::errors::ConnectionError::DB;
use crate::connections::db::DB;
use async_trait::async_trait;

#[async_trait]
pub trait EmailRepository {}

pub struct EmailRepo(DB);

impl EmailRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

impl EmailRepository for EmailRepo {}
