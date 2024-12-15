use crate::{
    connections::db::DB,
    repository::user::{User, UserRepository},
};

pub mod user;

#[derive(Debug)]
pub struct Repository {
    pub user: Box<dyn UserRepository>,
}

impl Repository {
    pub fn new(db: &DB) -> Self {
        let user = User::new(db);

        Self {
            user: Box::new(user),
        }
    }
}
