use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use uuid::Uuid;
use sdk::constants::request_id;

pub struct RequestID;

impl<B> FromRequestParts<B> for RequestID where B: Send + Sync{
    type Rejection = (); // todo: this has to be app_error

    async fn from_request_parts(parts: &mut Parts, _state: &B) -> Result<Self, Self::Rejection> {
        let id = Uuid::new_v4();

        parts.headers.insert(request_id, id.to_string().parse().unwrap());

        Ok(Self)
    }
}
