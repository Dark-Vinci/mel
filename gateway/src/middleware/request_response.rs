use {
    axum::{
        body::{Body, Bytes},
        extract::Request,
        http::StatusCode,
        middleware::{self, Next},
        response::{IntoResponse, Response},
    },
    sdk::constants::REQUEST_ID,
};

const REQUEST: &'static str = "request";
const RESPONSE: &'static str = "response";

pub async fn handle_print_request_response(
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    let id = req.headers().get(REQUEST_ID).unwrap().to_str().unwrap();
    let (part, body) = req.into_parts();

    let bytes = get_bytes(REQUEST, body, id).await;
    let req = Request::from_parts(part, Body::from(bytes));

    let res = next.run(req).await;

    let (parts, body) = res.into_parts();
    let bytes = get_bytes(RESPONSE, body, id).await?;
    let res = Response::from_parts(parts, Body::from(bytes));

    Ok(res)
}

async fn get_bytes<B>(
    typ: &'static str,
    body: B,
    id: &str,
) -> Result<Bytes, (StatusCode, String)>
where
    B: axum::body::HttpBody<Data = Bytes>,
    B::Error: std::fmt::Display,
{
    let bytes = match body.collect().await {
        Ok(collected) => collected.to_bytes(),
        Err(err) => {
            return Err((
                StatusCode::BAD_REQUEST,
                format!("failed to read {direction} body: {err}"), //todo: update later
            ));
        },
    };

    if let Ok(body) = std::str::from_utf8(&bytes) {
        tracing::debug!("{direction} body = {body:?}, id={id}");
    }

    Ok(bytes)
}
