use serde::{Deserialize, Serialize};

// Token validation result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenValidation {
    pub valid: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

// GitHub User
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubUser {
    pub id: i64,
    pub login: String,
    pub name: Option<String>,
    pub avatar_url: String,
    pub bio: Option<String>,
    pub location: Option<String>,
    pub company: Option<String>,
    pub blog: Option<String>,
    pub email: Option<String>,
    pub public_repos: u32,
    pub total_private_repos: Option<u32>,
    pub owned_private_repos: Option<u32>,
    pub followers: u32,
    pub following: u32,
    pub disk_usage: Option<u64>,
    pub created_at: String,
    pub updated_at: String,
}

// GitHub Repository Owner
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubOwner {
    pub login: String,
    pub id: i64,
    pub avatar_url: String,
    #[serde(rename = "type")]
    pub owner_type: String,
}

// GitHub Repository Permissions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubPermissions {
    pub admin: bool,
    pub push: bool,
    pub pull: bool,
}

// GitHub Repository
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepo {
    pub id: i64,
    pub name: String,
    pub full_name: String,
    pub description: Option<String>,
    pub private: bool,
    pub fork: bool,
    pub archived: bool,
    pub disabled: bool,
    pub html_url: String,
    pub clone_url: String,
    pub ssh_url: String,
    pub homepage: Option<String>,
    pub language: Option<String>,
    pub stargazers_count: u32,
    pub watchers_count: u32,
    pub forks_count: u32,
    pub open_issues_count: u32,
    pub size: u64,
    pub default_branch: String,
    pub visibility: String,
    pub pushed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub owner: GitHubOwner,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<GitHubPermissions>,
}

// GitHub Organization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubOrg {
    pub id: i64,
    pub login: String,
    pub description: Option<String>,
    pub avatar_url: String,
    pub url: String,
}

// GitHub Event Actor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubEventActor {
    pub login: String,
    pub avatar_url: String,
}

// GitHub Event Repo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubEventRepo {
    pub id: i64,
    pub name: String,
}

// GitHub Event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubEvent {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub actor: GitHubEventActor,
    pub repo: GitHubEventRepo,
    pub payload: serde_json::Value,
    pub created_at: String,
}

// GitHub Stats
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubStats {
    pub stars: u32,
    pub forks: u32,
}

// Backup Options
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupOptions {
    pub destination: String,
    pub clone_type: String,
    pub include_forks: Option<bool>,
    pub include_archived: Option<bool>,
    pub split_by: Option<String>,
    pub create_zip: Option<bool>,
    pub zip_compression: Option<u8>,
    pub exclude_patterns: Option<Vec<String>>,
    pub max_concurrent: Option<u8>,
}

impl Default for BackupOptions {
    fn default() -> Self {
        Self {
            destination: String::new(),
            clone_type: "full".to_string(),
            include_forks: Some(true),
            include_archived: Some(false),
            split_by: Some("owner".to_string()),
            create_zip: Some(false),
            zip_compression: Some(6),
            exclude_patterns: Some(vec![
                "node_modules".to_string(),
                ".DS_Store".to_string(),
                "Thumbs.db".to_string(),
            ]),
            max_concurrent: Some(3),
        }
    }
}

// Repository Backup Progress
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepoBackupProgress {
    pub repo_id: i64,
    pub repo_name: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// Backup Progress
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupProgress {
    pub total_repos: u32,
    pub completed_repos: u32,
    pub failed_repos: u32,
    pub current_repo: Option<String>,
    pub repos: Vec<RepoBackupProgress>,
    pub start_time: i64,
    pub is_running: bool,
    pub is_cancelled: bool,
}

// Backup History Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackupHistoryEntry {
    pub id: String,
    pub date: String,
    pub repo_count: u32,
    pub total_size: u64,
    pub duration: i64,
    pub destination: String,
    pub status: String,
    pub failed_repos: Vec<String>,
    pub options: BackupOptions,
}

// Archive Progress
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveProgress {
    pub is_running: bool,
    pub progress: u8,
    pub current_file: Option<String>,
    pub total_files: u32,
    pub processed_files: u32,
}

// Archive Result
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchiveResult {
    pub path: String,
    pub size: u64,
}

// Auto Backup Settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoBackupSettings {
    pub enabled: bool,
    pub schedule: String,
    pub time: String,
    pub last_run: Option<String>,
}

// Default Backup Options for Settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DefaultBackupOptions {
    pub clone_type: Option<String>,
    pub include_forks: Option<bool>,
    pub include_archived: Option<bool>,
    pub split_by: Option<String>,
    pub create_zip: Option<bool>,
    pub zip_compression: Option<u8>,
    pub exclude_patterns: Option<Vec<String>>,
    pub max_concurrent: Option<u8>,
}

// App Settings
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub theme: String,
    pub default_backup_location: String,
    pub default_backup_options: DefaultBackupOptions,
    pub notifications: bool,
    pub auto_backup: AutoBackupSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            theme: "system".to_string(),
            default_backup_location: String::new(),
            default_backup_options: DefaultBackupOptions {
                clone_type: Some("full".to_string()),
                include_forks: Some(true),
                include_archived: Some(false),
                split_by: Some("owner".to_string()),
                create_zip: Some(false),
                zip_compression: Some(6),
                exclude_patterns: Some(vec![
                    "node_modules".to_string(),
                    ".DS_Store".to_string(),
                    "Thumbs.db".to_string(),
                ]),
                max_concurrent: Some(3),
            },
            notifications: true,
            auto_backup: AutoBackupSettings {
                enabled: false,
                schedule: "weekly".to_string(),
                time: "03:00".to_string(),
                last_run: None,
            },
        }
    }
}

// UI-specific types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Auth,
    Dashboard,
    Repositories,
    Backup,
    Settings,
    About,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupTab {
    Options,
    Progress,
    History,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VisibilityFilter {
    All,
    Public,
    Private,
}

impl std::fmt::Display for VisibilityFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            VisibilityFilter::All => write!(f, "All"),
            VisibilityFilter::Public => write!(f, "Public"),
            VisibilityFilter::Private => write!(f, "Private"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortOption {
    Name,
    Stars,
    Updated,
    Size,
}

impl std::fmt::Display for SortOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortOption::Name => write!(f, "Name"),
            SortOption::Stars => write!(f, "Stars"),
            SortOption::Updated => write!(f, "Updated"),
            SortOption::Size => write!(f, "Size"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortDirection {
    Ascending,
    Descending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTheme {
    Light,
    Dark,
    System,
}

impl std::fmt::Display for AppTheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppTheme::Light => write!(f, "Light"),
            AppTheme::Dark => write!(f, "Dark"),
            AppTheme::System => write!(f, "System"),
        }
    }
}
