use metrics::{counter, histogram};
use std::time::Instant;

// ---------- HTTP METRICS ----------

pub fn record_http_request(method: &str, path: &str, status: &str) {
    counter!(
        "http_requests_total",
        "method" => method.to_string(),
        "path" => path.to_string(),
        "status" => status.to_string()
    )
    .increment(1);
}

pub fn record_http_latency(method: &str, path: &str, status: &str, start: Instant) {
    let duration = start.elapsed().as_secs_f64();

    histogram!(
        "http_request_duration_seconds",
        "method" => method.to_string(),
        "path" => path.to_string(),
        "status" => status.to_string()
    )
    .record(duration);
}

// ---------- DB METRICS ----------

pub fn record_db_query(query_name: &'static str, start: Instant) {
    let duration = start.elapsed().as_secs_f64();

    histogram!(
        "db_query_duration_seconds",
        "query" => query_name
    )
    .record(duration);
}

pub fn record_db_error(query_name: &'static str) {
    counter!(
        "db_query_errors_total",
        "query" => query_name
    )
    .increment(1);
}