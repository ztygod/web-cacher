use anyhow::Ok;
use reqwest::Client;

pub async fn fetch_url(url: &str) -> Result<String> {
    let client = Client::new();
    let res = client.get(url).send().await?;
    let body = res.text().await?;
    Ok(body)
}
