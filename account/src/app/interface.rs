use async_trait::async_trait;
#[cfg(test)]
use mockall::automock;
// struct CTX {
//     request_id: Uuid,
//     user_id: Option<Uuid>,
//     time_zone: String,
//     auth_token: String,
//     language: String,
// }

#[cfg_attr(test, automock)]
pub trait Auth {
    // async fn create_user(&self, ctx: &CTX, user: CreateUserRequest) -> User;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait Account {
    async fn name(&self) -> Result<String, String>;
}

#[async_trait]
#[cfg_attr(test, automock)]
pub trait Settings {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_settings() {
        let mut a = MockAccount::new();
        let response = a.expect_name().with().once().return_const("Ok");

        assert!(response.eq("Ok"));
    }
}
