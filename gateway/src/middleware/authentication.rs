use {
    super::errors::ApiError,
    axum::{
        extract::Extension,
        http::{Request, StatusCode},
        middleware::Next,
        response::Response,
    },
};

async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, ApiError> {
    let id = req.headers().get(REQUEST_ID).unwrap().to_str().unwrap();

    // Extract the `Authorization` header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok());

    if auth_header.is_none() {
        return Err(ApiError::new(
            StatusCode::UNAUTHORIZED,
            "no token provided".into(),
            id,
            "ts".into(),
        ));
    }

    if auth_header.starts_with("Bearer ") {
        let token = &auth_header[7..];

        // Decode and validate the JWT
        match decode_jwt(token) {
            Ok(claims) => {
                // Attach claims to the request extensions
                req.extensions_mut().insert(claims);
                return Ok(next.run(req).await);
            },
            Err(_) => {
                return Err(ApiError::new(
                    StatusCode::BAD_REQUEST,
                    "token invalid".into(),
                    id,
                    "ts".into(),
                ));
            },
        }
    }

    return Err(ApiError::new(
        StatusCode::BAD_REQUEST,
        "token invalid".into(),
        id,
        "ts".into(),
    ));
}
