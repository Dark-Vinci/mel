use {chrono::Utc, sea_orm::prelude::DateTime, uuid::Uuid};

#[derive(Debug)]
pub struct CreateWorkspaceRequest {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: DateTime,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UpdateWorkspaceRequest {
    pub id: Uuid,
    pub name: Option<String>,
    pub description: Option<String>,
    pub workspace_admin: Option<String>,
    pub password: Option<String>,
    pub date_of_birth: Option<chrono::DateTime<Utc>>,
}

pub struct CreateWorkspaceUser {}

pub struct UpdateWorkspaceUser {}
