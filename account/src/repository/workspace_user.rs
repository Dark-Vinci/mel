use {crate::connections::db::DB, async_trait::async_trait};

#[async_trait]
pub trait WorkspaceUserRepository {}

pub struct WorkspaceUserRepo(DB);

impl WorkspaceUserRepo {
    pub fn new(db: DB) -> Self {
        Self(db)
    }
}

#[async_trait]
impl WorkspaceUserRepository for WorkspaceUserRepo {}
