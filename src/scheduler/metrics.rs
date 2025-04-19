use prometheus_client::{
    encoding::text::encode,
    metrics::{counter::Counter, family::Family},
    registry::Registry,
};

pub struct Metrics {
    registry: Registry,
    requests_total: Family<Labels, Counter>,
}

impl Metrics {
    pub fn new() -> Self {
        let mut registry = Registry::default();
        let requests_total = Family::default();
        registry.register(
            "requests_total",
            "Total number of HTTP requests",
            requests_total.clone(),
        );
        Self {
            registry,
            requests_total,
        }
    }

    pub fn render(&self) -> String {
        let mut buffer = String::new();
        encode(&mut buffer, &self.registry).unwrap();
        buffer
    }

    pub fn record_request(&self, url: &str, success: bool) {
        let labels = Labels::new(url, if success { "ok" } else { "error" });
        self.requests_total.get_or_create(&labels).inc();
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, prometheus_client::encoding::EncodeLabelSet)]
struct Labels {
    url: String,
    status: String,
}

impl Labels {
    fn new(url: &str, status: &str) -> Self {
        Self {
            url: url.to_string(),
            status: status.to_string(),
        }
    }
}
