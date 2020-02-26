mod check;
mod diff;
mod issue;
mod types;

use reqwest::Error;
use std::env;

const UA: &str = "Mozilla/5.0 Gecko/20100101 Firefox/73.0";
const BASE_URL: &str = "https://api.github.com";

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("GH_TOKEN").expect("missing GitHub token");

    if let Some(html_url) = check::has_changed().await? {
        let git_diff = diff::fetch_diff().await?;
        issue::create(&token, &html_url, git_diff).await?;
    }

    Ok(())
}
