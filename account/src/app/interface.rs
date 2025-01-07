use {async_trait::async_trait, sdk::errors::GrpcError, uuid::Uuid};

struct CTX {
    request_id: Uuid,
    user_id: Option<Uuid>,
    time_zone: String,
    auth_token: String,
    language: String,
}

pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: String,
    pub email: String,
    pub password: String,
}

pub struct User {
    pub name: String,
}

pub trait Auth {
    async fn create_user(&self, ctx: &CTX, user: CreateUserRequest) -> User;
}

#[async_trait]
pub trait Account {
    async fn name(&self) -> Result<String, GrpcError>;
}

pub trait Settings {}
