use crate::types::*;
use reqwest::header::{HeaderMap, HeaderValue, ACCEPT, AUTHORIZATION, USER_AGENT};
use serde::Deserialize;
use std::time::Duration;

pub struct GitHubApiService {
    client: reqwest::Client,
    token: String,
}

#[derive(Debug, Deserialize)]
struct GitHubApiUser {
    id: i64,
    login: String,
    name: Option<String>,
    avatar_url: String,
    bio: Option<String>,
    location: Option<String>,
    company: Option<String>,
    blog: Option<String>,
    email: Option<String>,
    public_repos: u32,
    total_private_repos: Option<u32>,
    owned_private_repos: Option<u32>,
    followers: u32,
    following: u32,
    disk_usage: Option<u64>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Deserialize)]
struct GitHubApiOwner {
    login: String,
    id: i64,
    avatar_url: String,
    #[serde(rename = "type")]
    owner_type: String,
}

#[derive(Debug, Deserialize)]
struct GitHubApiPermissions {
    admin: Option<bool>,
    push: Option<bool>,
    pull: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct GitHubApiRepo {
    id: i64,
    name: String,
    full_name: String,
    description: Option<String>,
    private: bool,
    fork: bool,
    archived: bool,
    disabled: bool,
    html_url: String,
    clone_url: String,
    ssh_url: String,
    homepage: Option<String>,
    language: Option<String>,
    stargazers_count: u32,
    watchers_count: u32,
    forks_count: u32,
    open_issues_count: u32,
    size: u64,
    default_branch: String,
    visibility: Option<String>,
    pushed_at: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    owner: GitHubApiOwner,
    permissions: Option<GitHubApiPermissions>,
}

#[derive(Debug, Deserialize)]
struct GitHubApiOrg {
    id: i64,
    login: String,
    description: Option<String>,
    avatar_url: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct GitHubApiEventActor {
    login: String,
    avatar_url: String,
}

#[derive(Debug, Deserialize)]
struct GitHubApiEventRepo {
    id: i64,
    name: String,
}

#[derive(Debug, Deserialize)]
struct GitHubApiEvent {
    id: String,
    #[serde(rename = "type")]
    event_type: Option<String>,
    actor: GitHubApiEventActor,
    repo: GitHubApiEventRepo,
    payload: serde_json::Value,
    created_at: Option<String>,
}

impl GitHubApiService {
    pub fn new(token: &str) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", token)).unwrap(),
        );
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/vnd.github+json"),
        );
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static("GitHub-Backup-Manager"),
        );
        headers.insert(
            "X-GitHub-Api-Version",
            HeaderValue::from_static("2022-11-28"),
        );

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            token: token.to_string(),
        }
    }

    pub async fn validate_token(&self) -> Result<TokenValidation, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .get("https://api.github.com/user")
            .send()
            .await?;

        if response.status().is_success() {
            let scopes = response
                .headers()
                .get("x-oauth-scopes")
                .and_then(|v| v.to_str().ok())
                .map(|s| {
                    s.split(", ")
                        .filter(|s| !s.is_empty())
                        .map(String::from)
                        .collect()
                });

            Ok(TokenValidation {
                valid: true,
                scopes,
            })
        } else {
            Ok(TokenValidation {
                valid: false,
                scopes: None,
            })
        }
    }

    pub async fn get_authenticated_user(&self) -> Result<GitHubUser, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .get("https://api.github.com/user")
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("GitHub API error: {} - {}", status, text).into());
        }

        let user: GitHubApiUser = response.json().await?;

        Ok(GitHubUser {
            id: user.id,
            login: user.login,
            name: user.name,
            avatar_url: user.avatar_url,
            bio: user.bio,
            location: user.location,
            company: user.company,
            blog: user.blog,
            email: user.email,
            public_repos: user.public_repos,
            total_private_repos: user.total_private_repos,
            owned_private_repos: user.owned_private_repos,
            followers: user.followers,
            following: user.following,
            disk_usage: user.disk_usage,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    pub async fn get_all_repos(&self) -> Result<Vec<GitHubRepo>, Box<dyn std::error::Error + Send + Sync>> {
        let mut all_repos = Vec::new();
        let mut page = 1;

        loop {
            let response = self
                .client
                .get("https://api.github.com/user/repos")
                .query(&[
                    ("per_page", "100"),
                    ("sort", "updated"),
                    ("visibility", "all"),
                    ("affiliation", "owner,collaborator,organization_member"),
                    ("page", &page.to_string()),
                ])
                .send()
                .await?;

            if !response.status().is_success() {
                let status = response.status();
                let text = response.text().await.unwrap_or_default();
                return Err(format!("GitHub API error: {} - {}", status, text).into());
            }

            let repos: Vec<GitHubApiRepo> = response.json().await?;

            if repos.is_empty() {
                break;
            }

            for repo in repos {
                all_repos.push(self.map_repo(repo));
            }

            page += 1;
        }

        Ok(all_repos)
    }

    pub async fn get_organizations(&self) -> Result<Vec<GitHubOrg>, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .get("https://api.github.com/user/orgs")
            .query(&[("per_page", "100")])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("GitHub API error: {} - {}", status, text).into());
        }

        let orgs: Vec<GitHubApiOrg> = response.json().await?;

        Ok(orgs
            .into_iter()
            .map(|org| GitHubOrg {
                id: org.id,
                login: org.login,
                description: org.description,
                avatar_url: org.avatar_url,
                url: org.url,
            })
            .collect())
    }

    pub async fn get_user_events(&self, limit: u32) -> Result<Vec<GitHubEvent>, Box<dyn std::error::Error + Send + Sync>> {
        let user = self.get_authenticated_user().await?;

        let response = self
            .client
            .get(&format!(
                "https://api.github.com/users/{}/events",
                user.login
            ))
            .query(&[("per_page", &limit.to_string())])
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(format!("GitHub API error: {} - {}", status, text).into());
        }

        let events: Vec<GitHubApiEvent> = response.json().await?;

        Ok(events
            .into_iter()
            .map(|event| GitHubEvent {
                id: event.id,
                event_type: event.event_type.unwrap_or_else(|| "Unknown".to_string()),
                actor: GitHubEventActor {
                    login: event.actor.login,
                    avatar_url: event.actor.avatar_url,
                },
                repo: GitHubEventRepo {
                    id: event.repo.id,
                    name: event.repo.name,
                },
                payload: event.payload,
                created_at: event.created_at.unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            })
            .collect())
    }

    fn map_repo(&self, repo: GitHubApiRepo) -> GitHubRepo {
        GitHubRepo {
            id: repo.id,
            name: repo.name,
            full_name: repo.full_name,
            description: repo.description,
            private: repo.private,
            fork: repo.fork,
            archived: repo.archived,
            disabled: repo.disabled,
            html_url: repo.html_url,
            clone_url: repo.clone_url,
            ssh_url: repo.ssh_url,
            homepage: repo.homepage,
            language: repo.language,
            stargazers_count: repo.stargazers_count,
            watchers_count: repo.watchers_count,
            forks_count: repo.forks_count,
            open_issues_count: repo.open_issues_count,
            size: repo.size,
            default_branch: repo.default_branch,
            visibility: repo.visibility.unwrap_or_else(|| {
                if repo.private {
                    "private".to_string()
                } else {
                    "public".to_string()
                }
            }),
            pushed_at: repo.pushed_at,
            created_at: repo.created_at.unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            updated_at: repo.updated_at.unwrap_or_else(|| chrono::Utc::now().to_rfc3339()),
            owner: GitHubOwner {
                login: repo.owner.login,
                id: repo.owner.id,
                avatar_url: repo.owner.avatar_url,
                owner_type: repo.owner.owner_type,
            },
            permissions: repo.permissions.map(|p| GitHubPermissions {
                admin: p.admin.unwrap_or(false),
                push: p.push.unwrap_or(false),
                pull: p.pull.unwrap_or(false),
            }),
        }
    }

    pub fn get_token(&self) -> &str {
        &self.token
    }
}
