use std::sync::Arc;

use crate::fetcher;

pub mod metrics;

pub struct Scheduler {
    client: Arc<fetcher::CustomClient>,
}
