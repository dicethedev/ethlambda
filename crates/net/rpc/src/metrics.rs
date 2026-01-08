use std::net::SocketAddr;

use axum::{Router, routing::get};

pub async fn start_prometheus_metrics_api(address: SocketAddr) -> Result<(), std::io::Error> {
    let app = Router::new()
        .route("/metrics", get(get_metrics))
        .route("/health", get("Service Up"));

    // Start the axum app
    let listener = tokio::net::TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

pub(crate) async fn get_metrics() -> String {
    gather_default_metrics().unwrap()
}

/// Returns all metrics currently registered in Prometheus' default registry.
///
/// Both profiling and RPC metrics register with this default registry, and the
/// metrics API surfaces them by calling this helper.
pub fn gather_default_metrics() -> Result<String, ()> {
    use prometheus::{Encoder, TextEncoder};

    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();

    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();

    let res = String::from_utf8(buffer).unwrap();

    Ok(res)
}
