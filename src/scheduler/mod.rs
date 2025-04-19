mod metrics;

use crate::{config::Config, fetcher};
use anyhow::Result;
use metrics::Metrics;
use tokio::time::Duration;
use tracing::debug;

pub struct Scheduler {
    config: Config,
    metrics: Metrics,
    interval: Duration,
}

impl Scheduler {}
