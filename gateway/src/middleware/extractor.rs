use {
    super::errors::ApiError,
    axum::{
        async_trait,
        extract::{FromRequestParts, Query},
        http::{request::Parts, StatusCode},
        response::Json as Rson,
        Json,
    },
    sdk::utils::error::collect_error,
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
        let id = req.headers().get(REQUEST_ID).unwrap().to_str().unwrap();
        let id = Uuid::from_str(id).unwrap();

        let query_res = Query::<T>::from_request_parts(parts, state).await;

        if let Err(e) = query_res {
            let message = e.to_string().as_str().parse().unwrap();
            let api_error = ApiError::new(
                StatusCode::BAD_REQUEST,
                message,
                id,
                "2025".into(),
            ); //todo: update
            return Err(apiError);
        }

        let Query(query_res) = query_res.unwrap();

        if let Err(err) = query_res.validate() {
            let message = collect_error(err);
            let api_error = ApiError::new(
                StatusCode::BAD_REQUEST,
                message,
                id,
                "2025".into(),
            ); //todo: same here
            return Err(apiError);
        }

        Ok(QueryValidator(query_res))
    }
}

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
        let id = req.headers().get(REQUEST_ID).unwrap().to_str().unwrap();
        let id = Uuid::from_str(id).unwrap();

        let param_res = Path::<T>::from_request_parts(parts, state).await;

        if let Err(e) = param_res {
            let message = e.to_string();
            let api_error = ApiError::new(
                StatusCode::BAD_REQUEST,
                message,
                id,
                "2025".into(),
            ); //todo: same here
            return Err(apiError);
        }

        let Path(param_res) = param_res.unwrap();

        if let Err(e) = param_res.validate() {
            let message = collect_error(e);
            let api_error = ApiError::new(
                StatusCode::BAD_REQUEST,
                message,
                id,
                "2025".into(),
            ); //todo: same here
            return Err(apiError);
        }

        Ok(ParamValidator(param_res))
    }
}

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
                e.to_string(),
                id,
                "2025".into(),
            ); //todo: same here
            return Err(api_error);
        }

        let Json(custom) = b.unwrap();

        if let Err(e) = custom.validate() {
            let message = collect_error(e);
            let api_error = ApiError::new(
                StatusCode::BAD_REQUEST,
                e.to_string(),
                id,
                "2025".into(),
            ); //todo: same here

            return Err(api_error);
        }

        Ok(BodyValidator(custom))
    }
}
