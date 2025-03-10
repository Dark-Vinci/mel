use {
    crate::models::{context::Ctx, error_response::ApiError},
    axum::{extract::FromRequestParts, http::request::Parts},
    sdk::constants::{
        constant::ZERO_UUID, AUTH_TOKEN, CHROME, REFRESH_TOKEN, REQUEST_ID,
        TIME_ZONE, USER_AGENT, USER_ID, UTC,
    },
    uuid::Uuid,
};

pub struct Context(pub Ctx);

impl<B> FromRequestParts<B> for Context
where
    B: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &B,
    ) -> Result<Self, Self::Rejection> {
        let request_id =
            parts.headers.get(REQUEST_ID).unwrap().to_str().unwrap();

        let user_agent = parts
            .headers
            .get(USER_AGENT)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| CHROME.to_string());

        let time_zone = parts
            .headers
            .get(TIME_ZONE)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| UTC.to_string());

        let auth_token = parts
            .headers
            .get(AUTH_TOKEN)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "".to_string());

        let refresh_token = parts
            .headers
            .get(REFRESH_TOKEN)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| "".to_string());

        let user_id = parts
            .headers
            .get(USER_ID)
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| ZERO_UUID.to_string());

        let request_id = Uuid::parse_str(&request_id).unwrap();
        let user_id = Some(Uuid::parse_str(&user_id).unwrap());

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
