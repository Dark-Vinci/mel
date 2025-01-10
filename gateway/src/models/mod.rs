pub mod response;

#[derive(Debug)]
pub struct LoginStruct {
    pub username: String,
    pub password: String,
}

pub struct LoginResponse {
    pub token: String,
    pub refresh_token: String,
    pub user_id: String,
}

#[derive(Debug, Clone)]
struct WsRequest<T: Clone> {
    pub action: String,
    pub payload: T,
    pub token: String,
}

#[derive(Debug, Clone)]
struct Message<'a> {
    pub content: String,
    pub to: Type,
    pub files: Vec<&'a str>,
}

#[derive(Debug, Clone)]
enum Type {
    // user id
    Private(String),
    // channel id
    Channel(String),
}

struct WsResponse {
    message: String,
}
