use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct File {
    pub filename: String,
}

#[derive(Deserialize)]
pub struct Commit {
    pub html_url: String,
    pub files: Vec<File>,
}

#[derive(Serialize)]
pub struct Issue {
    pub title: String,
    pub body: String,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum IssueResponse {
    Ok { id: i32 },
    Err { message: String },
}
