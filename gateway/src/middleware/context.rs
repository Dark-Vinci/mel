use {
    crate::models::context::Ctx,
    axum::{
        async_trait,
        extract::{FromRequest, Request},
        http::HeaderValue,
    },
    sdk::constants::{
        AUTH_TOKEN, CHROME, REFRESH_TOKEN, REQUEST_ID, TIME_ZONE, USER_AGENT,
        USER_ID, UTC,
    },
    uuid::Uuid,
};

pub struct Context(pub Ctx);

#[async_trait]
impl<B> FromRequest<B> for Context
where
    B: Send + Sync,
{
    type Rejection = String;

    async fn from_request(
        req: Request,
        _state: &B,
    ) -> Result<Self, Self::Rejection> {
        let request_id =
            req.headers().get(REQUEST_ID).unwrap().to_str().unwrap();

        let user_agent = req
            .headers()
            .get(USER_AGENT)
            .unwrap_or(&HeaderValue::from_static(CHROME))
            .to_str()
            .unwrap();

        let time_zone = req
            .headers()
            .get(TIME_ZONE)
            .unwrap_or(&HeaderValue::from_static(UTC))
            .to_str()
            .unwrap();

        let auth_token = req
            .headers()
            .get(AUTH_TOKEN)
            .unwrap_or(&HeaderValue::from_static(""))
            .to_str()
            .unwrap();

        let refresh_token = req
            .headers()
            .get(REFRESH_TOKEN)
            .unwrap_or(&HeaderValue::from_static(""))
            .to_str()
            .unwrap();

        let user_id = req
            .headers()
            .get(USER_ID)
            .unwrap_or(&HeaderValue::from_static(&Uuid::nil().to_string()))
            .to_str()
            .unwrap();

        let request_id = Uuid::from(request_id);
        let user_id = Some(Uuid::from(user_id));

        let mut ctx = Ctx::new(
            user_agent.to_string(),
            request_id,
            time_zone.to_string(),
            None,
            None,
            user_id,
        );

        if auth_token.len() > 0 {
            ctx.auth_token = Some(auth_token.to_string());
        }

        if refresh_token.len() > 0 {
            ctx.refresh_token = Some(refresh_token.to_string());
        }

        Ok(Self(ctx))
    }
}
