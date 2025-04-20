use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tokio::fs;

use crate::fetcher::FetchResult;

//磁盘缓存实现
#[derive(Serialize, Deserialize)]
struct CacheEntry {
    result: FetchResult,
    version: Vec<String>,
}

pub struct DiskCache {
    root: PathBuf,
    max_versions: usize,
}

impl DiskCache {
    pub fn new(root: impl AsRef<Path>, max_versions: usize) -> Result<Self> {
        let root = root.as_ref().to_path_buf();
        if !root.exists() {
            std::fs::create_dir_all(&root)?;
        }
        Ok(Self { root, max_versions })
    }

    pub async fn get(&self, url: &str) -> Result<Option<FetchResult>> {
        let path = self.url_to_path(url);
        if !path.exists() {
            return Ok(None);
        }

        let data = fs::read(&path).await?;
        let entry: CacheEntry = bincode::deserialize(&data)?;
        Ok(entry.version.last().map(|v| entry.result.clone()))
    }

    pub async fn store(&self, url: &str, result: &FetchResult) -> Result<()> {
        let path = self.url_to_path(url);
        let mut entry = if path.exists() {
            let data = fs::read(&path).await?;
            bincode::deserialize(&data)?
        } else {
            CacheEntry {
                result: result.clone(),
                version: Vec::with_capacity(self.max_versions),
            }
        };

        //维护版本历史
        entry.version.push(result.hash.clone());
        if entry.version.len() > self.max_versions {
            entry.version.remove(0);
        }

        let data = bincode::serialize(&entry)?;
        fs::write(path, data).await?;
        Ok(())
    }

    fn url_to_path(&self, url: &str) -> PathBuf {
        let filename = format!("{:x}.bin", md5::compute(url));
        self.root.join(filename)
    }
}
