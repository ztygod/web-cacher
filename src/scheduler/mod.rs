mod metrics;

use metrics::Metrics;
use tokio::time::Duration;

use crate::config::Config;

pub struct Scheduler {
    config: Config,
    metrics: Metrics,
    interval: Duration,
}
