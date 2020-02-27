use crate::{
    types::{Issue, IssueResponse},
    BASE_URL, UA,
};
use reqwest::{ClientBuilder, Error};

const ISSUE_TITLE: &str = "Laravel Config Has Changed.";

pub async fn create(token: &str, html_url: &str, diff: String) -> Result<(), Error> {
    let content = format!(
        "### View More\n\n[{}]({})\n\n### Diff\n\n```diff\n{}\n```\n",
        html_url, html_url, diff
    );

    let builder = ClientBuilder::new();
    let client = builder.user_agent(UA).build()?;
    let resp = client
        .post(&format!(
            "{}/repos/bs-community/blessing-skin-server/issues",
            BASE_URL
        ))
        .header("Authorization", format!("token {}", token))
        .json(&Issue {
            title: ISSUE_TITLE.to_string(),
            body: content,
        })
        .send()
        .await?
        .json::<IssueResponse>()
        .await?;

    if let IssueResponse::Err { message } = resp {
        eprintln!("Failed to create issue: {}", message);
    }

    Ok(())
}
