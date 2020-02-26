use crate::UA;
use reqwest::{ClientBuilder, Error};

pub async fn fetch_diff() -> Result<String, Error> {
    let builder = ClientBuilder::new();
    let client = builder.user_agent(UA).build()?;
    client
        .get("https://github.com/laravel/laravel/commit/HEAD.diff")
        .send()
        .await?
        .text()
        .await
}
