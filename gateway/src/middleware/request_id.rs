use {
    axum::{extract::FromRequestParts, http::request::Parts},
    sdk::constants::{request_id, REQUEST_ID},
    uuid::Uuid,
};

pub struct RequestID;

impl<B> FromRequestParts<B> for RequestID
where
    B: Send + Sync,
{
    type Rejection = (); // todo: this has to be app_error

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &B,
    ) -> Result<Self, Self::Rejection> {
        let id = Uuid::new_v4();

        parts
            .headers
            .insert(REQUEST_ID, id.to_string().parse().unwrap());

        Ok(Self)
    }
}

// GET THE REQUEST ID
pub struct GetRequestID(pub Uuid);

#[async_trait]
impl<B> FromRequest<B> for GetRequestID
where
    B: Send + Sync,
{
    type Rejection = String;

    async fn from_request(
        req: Request,
        _state: &B,
    ) -> Result<Self, Self::Rejection> {
        // we can control this, so no need for error handling, we'll always generate a UUID
        let id = req.headers().get(REQUEST_ID).unwrap().to_str().unwrap();

        // we can also still control here too;
        let k = Uuid::from_str(id).unwrap();

        Ok(Self(k))
    }
}

pub async fn append_request_id(
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let uuid = Uuid::new_v4().to_string();

    req.headers_mut()
        .append(REQUEST_ID, (&uuid).parse().unwrap());

    let res = next.run(req).await;

    Ok(res)
}
