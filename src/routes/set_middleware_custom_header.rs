use axum::{
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use serde::de::value::MapAccessDeserializer;

use super::read_middleware_custom_header::HeaderMessage;

pub async fn set_middleware_custom_header<B>(
    mut request: Request<B>,
    next: Next<B>,
) -> Result<Response, StatusCode> {
    let headers = request.headers();
    let message = headers
        .get("message")
        .ok_or_else(|| StatusCode::BAD_REQUEST)?;
    let message = message
        .to_str()
        .map_err(|_err| StatusCode::BAD_REQUEST)?
        .to_owned();
    let extenstion = request.extensions_mut();
    extenstion.insert(HeaderMessage(message));

    Ok(next.run(request).await)
}
