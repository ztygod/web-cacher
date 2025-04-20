use crate::{cache, config::MonitorTask, fetcher};
use anyhow::Result;
use metrics::Metrics;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod metrics;

pub struct Scheduler {
    client: Arc<fetcher::CustomClient>,
    cache: Arc<cache::disk::DiskCache>,
    metrics: Arc<Metrics>,
    tasks: Mutex<Vec<MonitorTask>>,
}

impl Scheduler {
    pub fn new(
        client: Arc<fetcher::CustomClient>,
        cache: Arc<cache::disk::DiskCache>,
        metrics: Arc<Metrics>,
    ) -> Self {
        Self {
            client,
            cache,
            metrics,
            tasks: Mutex::new(Vec::new()),
        }
    }

    pub async fn add_tasks(&mut self, task: MonitorTask) -> Result<()> {
        let mut tasks = self.tasks.lock().await;
        tasks.push(task);
        Ok(())
    }

    pub async fn run(&self) -> Result<()> {
        let tasks = self.tasks.lock().await;
        let mut join_handles = Vec::new();

        for task in tasks.iter() {
            let task = task.clone();
            let client = self.client.clone();
            let cache = self.cache.clone();
            let metrics = self.metrics.clone();

            let handle = tokio::spawn(async move {
                let mut interval = tokio::time::interval(task.interval);
                loop {
                    interval.tick().await;

                    //执行监控检查
                    let result = fetcher::check_url(&client, url, timeout).await;

                    //处理检查结果
                    metrics.record_request(&task.url, result.is_ok());

                    //缓存比较和报警逻辑
                    if let Ok(status) = result {
                        if let Some(pre) = cache.get(&task.url).await? {
                            if pre.hash != status.hash{
                                
                            }
                        }
                    }
                }
            })
        }
    }
}
