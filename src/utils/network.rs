use anyhow::Result;

pub async fn test_latency(url: &str) -> Result<u64> {
    let start = std::time::Instant::now();
    reqwest::get(url).await?;
    let elapsed = start.elapsed().as_millis() as u64;
    Ok(elapsed)
}
