use {
    crate::models::error_response::ApiError,
    axum::{
        async_trait,
        extract::{FromRequest, FromRequestParts, Path, Query, Request},
        http::{request::Parts, StatusCode},
        Json,
    },
    sdk::{constants::REQUEST_ID, utils::error::collect_error},
    serde::de::DeserializeOwned,
    std::str::FromStr,
    uuid::Uuid,
    validator::Validate,
};

#[derive(Debug, Clone, Copy, Default)]
pub struct QueryValidator<T: Validate>(pub T);

#[async_trait]
impl<K, T> FromRequestParts<K> for QueryValidator<T>
where
    K: Send + Sync,
    T: DeserializeOwned + Validate + Clone + Send + Sync + Sized + 'static,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &K,
    ) -> Result<Self, Self::Rejection> {
        let id = parts.headers.get(REQUEST_ID).unwrap().to_str().unwrap();
        let id = Uuid::from_str(id).unwrap();

        let query_res = Query::<T>::from_request_parts(parts, state).await;

        if let Err(e) = query_res {
            let message = e.to_string().as_str().parse().unwrap();

            let api_error =
                ApiError::new(StatusCode::BAD_REQUEST, id, message, "".into());

            return Err(api_error);
        }

        let Query(query_res) = query_res.unwrap();

        if let Err(err) = query_res.validate() {
            let message = collect_error(err);

            let api_error =
                ApiError::new(StatusCode::BAD_REQUEST, id, message, "".into());

            return Err(api_error);
        }

        Ok(QueryValidator(query_res))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct ParamValidator<T: Validate>(pub T);

#[async_trait]
impl<K, T> FromRequestParts<K> for ParamValidator<T>
where
    K: Send + Sync,
    T: DeserializeOwned + Validate + Clone + Send + Sync + Sized + 'static,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &K,
    ) -> Result<Self, Self::Rejection> {
        let id = parts.headers.get(REQUEST_ID).unwrap().to_str().unwrap();
        let id = Uuid::from_str(id).unwrap();

        let param_res = Path::<T>::from_request_parts(parts, state).await;

        if let Err(e) = param_res {
            let message = e.to_string();

            let api_error =
                ApiError::new(StatusCode::BAD_REQUEST, id, message, "".into());

            return Err(api_error);
        }

        let Path(param_res) = param_res.unwrap();

        if let Err(e) = param_res.validate() {
            let message = collect_error(e);

            let api_error =
                ApiError::new(StatusCode::BAD_REQUEST, id, message, "".into());

            return Err(api_error);
        }

        Ok(ParamValidator(param_res))
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct BodyValidator<T: Validate>(pub T);

#[async_trait]
impl<B, T> FromRequest<B> for BodyValidator<T>
where
    B: Send + Sync,
    T: DeserializeOwned + Validate + Clone + Send + Sync + Sized + 'static,
{
    type Rejection = ApiError;

    async fn from_request(
        req: Request,
        state: &B,
    ) -> Result<Self, Self::Rejection> {
        let id = req.headers().get(REQUEST_ID).unwrap().to_str().unwrap();
        let id = Uuid::from_str(id).unwrap();

        let b = Json::<T>::from_request(req, state).await;

        if let Err(e) = b {
            let api_error = ApiError::new(
                StatusCode::BAD_REQUEST,
                id,
                e.to_string(),
                "".into(),
            );
            return Err(api_error);
        }

        let Json(custom) = b.unwrap();

        if let Err(e) = custom.validate() {
            let message = collect_error(e);
            let api_error =
                ApiError::new(StatusCode::BAD_REQUEST, id, message, "".into());

            return Err(api_error);
        }

        Ok(BodyValidator(custom))
    }
}
