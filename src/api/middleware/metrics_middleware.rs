use std::time::Instant;

use axum::{
    extract::MatchedPath,
    http::Request,
    middleware::Next,
    response::Response,
};

use crate::infrastructure::metrics::{record_http_latency, record_http_request};

pub async fn metrics_middleware(
    req: Request<axum::body::Body>,
    next: Next,
) -> Response {
    let method = req.method().to_string();

    let path = req
        .extensions()
        .get::<MatchedPath>()
        .map(|p| p.as_str())
        .unwrap_or("unknown")
        .to_string();

    let start = Instant::now();

    let response = next.run(req).await;

    let status = response.status().as_u16().to_string();

    record_http_request(&method, &path, &status);
    record_http_latency(&method, &path, &status, start);

    response
}