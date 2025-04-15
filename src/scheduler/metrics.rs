use metrics::{Counter, Gauge};

pub struct Metrics {
    request_total: Counter,
    bytes_total: Counter,
    uptime_gauge: Gauge,
}
