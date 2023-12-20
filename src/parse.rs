use reqwest::{self, Result};

pub async fn get_site_data(site: &str) -> Result<String> {
    let body = reqwest::get(site).await?.text().await?;
    Ok(body)
}
