use crate::UA;
use reqwest::{ClientBuilder, Error};

pub async fn fetch_diff(token: &str) -> Result<String, Error> {
    let builder = ClientBuilder::new();
    let client = builder.user_agent(UA).build()?;
    client
        .get("https://github.com/laravel/laravel/commit/HEAD.diff")
        .header("Authorization", format!("token {}", token))
        .send()
        .await?
        .text()
        .await
}
