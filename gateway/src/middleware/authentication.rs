use {
    super::errors::ApiError, axum::{
        http::{Request, StatusCode},
        middleware::Next,
        response::Response,
    }, sdk::constants::REQUEST_ID, std::str::FromStr, uuid::Uuid
};

async fn auth_middleware<B>(
    mut req: Request<B>,
    next: Next,
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
            Uuid::from_str(id).unwrap(),
            "ts".into(),
        ));
    }
    
    let auth_header = auth_header.unwrap().to_string();

    if !auth_header.starts_with("Bearer ") {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "token invalid".into(),
            Uuid::from_str(id).unwrap(),
            "ts".into(),
        ));
    }
    
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

fn decode_jwt(token: &str) -> Result<String, ApiError> {
    todo!()
}
