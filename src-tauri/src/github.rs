//! Public GitHub REST API helpers for repo metadata (stars, forks, etc.).

use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::error::LugosError;
use crate::installer::parse_github_repo;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GitHubRepoStats {
    pub full_name: String,
    pub owner_login: String,
    pub owner_avatar_url: String,
    pub description: Option<String>,
    pub html_url: String,
    pub stars: u32,
    pub forks: u32,
    pub open_issues: u32,
    pub watchers: u32,
    pub default_branch: String,
    pub pushed_at: Option<String>,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub topics: Vec<String>,
    pub license_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GhLicense {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct GhOwner {
    login: String,
    avatar_url: String,
}

#[derive(Debug, Deserialize)]
struct GhRepoResponse {
    full_name: String,
    owner: GhOwner,
    description: Option<String>,
    html_url: String,
    stargazers_count: u32,
    forks_count: u32,
    open_issues_count: u32,
    watchers_count: u32,
    default_branch: String,
    pushed_at: Option<String>,
    homepage: Option<String>,
    language: Option<String>,
    topics: Option<Vec<String>>,
    license: Option<GhLicense>,
}

fn http_client() -> Result<reqwest::Client, LugosError> {
    reqwest::Client::builder()
        .user_agent("lugios-host/0.1 (github api; open source)")
        .timeout(Duration::from_secs(12))
        .build()
        .map_err(Into::into)
}

pub async fn fetch_repo_stats(repo_url: &str) -> Result<GitHubRepoStats, LugosError> {
    let (owner, repo) = parse_github_repo(repo_url)?;
    let client = http_client()?;
    let url = format!("https://api.github.com/repos/{owner}/{repo}");
    let resp = client
        .get(&url)
        .header("Accept", "application/vnd.github+json")
        .header("X-GitHub-Api-Version", "2022-11-28")
        .send()
        .await?
        .error_for_status()?;
    let r: GhRepoResponse = resp.json().await?;

    Ok(GitHubRepoStats {
        full_name: r.full_name,
        owner_login: r.owner.login,
        owner_avatar_url: r.owner.avatar_url,
        description: r.description,
        html_url: r.html_url,
        stars: r.stargazers_count,
        forks: r.forks_count,
        open_issues: r.open_issues_count,
        watchers: r.watchers_count,
        default_branch: r.default_branch,
        pushed_at: r.pushed_at,
        homepage: r.homepage,
        language: r.language,
        topics: r.topics.unwrap_or_default(),
        license_name: r.license.and_then(|l| l.name),
    })
}
