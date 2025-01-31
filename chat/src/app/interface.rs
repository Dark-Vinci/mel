// use async_trait::async_trait;
// struct CTX {
//     request_id: Uuid,
//     user_id: Option<Uuid>,
//     time_zone: String,
//     auth_token: String,
//     language: String,
// }

pub trait Auth {
    // async fn create_user(&self, ctx: &CTX, user: CreateUserRequest) -> User;
}

pub trait Account {}


pub trait Settings {}

#[cfg(test)]
mod tests {
}
