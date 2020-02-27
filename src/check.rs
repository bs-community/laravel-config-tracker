use crate::{types::*, BASE_URL, UA};
use reqwest::{ClientBuilder, Error};

pub async fn has_changed(token: &str) -> Result<Option<String>, Error> {
    let builder = ClientBuilder::new();
    let client = builder.user_agent(UA).build()?;
    let Commit { files, html_url } = client
        .get(&format!("{}/repos/laravel/laravel/commits/HEAD", BASE_URL))
        .header("Authorization", format!("token {}", token))
        .send()
        .await?
        .json()
        .await?;

    let changed = files.iter().any(|file| file.filename.starts_with("config"));
    if changed {
        Ok(Some(html_url))
    } else {
        Ok(None)
    }
}
