use sea_orm::prelude::DateTime;

pub struct CreateUserRequest {
    pub first_name: String,
    pub last_name: String,
    pub date_of_birth: DateTime,
    pub email: String,
    pub password: String,
}
