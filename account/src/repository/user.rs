use crate::connections::db::DB;

pub trait UserRepository {}

pub struct User(DatabaseConnection);

impl User {
    pub fn new(db: &DB) -> Self {
        Self { db }
    }
}

impl UserRepository for User {}
