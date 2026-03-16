use crate::services::archive::ArchiveService;
use crate::services::github::GitHubApiService;
use crate::services::git::GitService;
use crate::services::storage::StorageService;
use crate::theme;
use crate::types::*;
use crate::widgets::sidebar;

use iced::widget::{container, row, text};
use iced::{Element, Length, Subscription, Task, Theme};
use iced::futures::SinkExt;

use std::collections::HashSet;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::{mpsc, Mutex, Semaphore};

pub struct GoshApp {
    // Navigation
    pub current_page: Page,

    // Auth state
    pub is_authenticated: bool,
    pub is_loading: bool,
    pub auth_status: Option<String>,
    pub token_input: String,

    // User data
    pub user: Option<GitHubUser>,
    pub repos: Vec<GitHubRepo>,
    pub orgs: Vec<GitHubOrg>,
    pub events: Vec<GitHubEvent>,

    // Repository page state
    pub selected_repos: HashSet<i64>,
    pub repo_search: String,
    pub repo_owner_filter: String,
    pub repo_visibility_filter: VisibilityFilter,
    pub repo_language_filter: String,
    pub repo_sort_by: SortOption,
    pub repo_sort_direction: SortDirection,

    // Backup state
    pub backup_options: BackupOptions,
    pub backup_progress: Option<BackupProgress>,
    pub backup_history: Vec<BackupHistoryEntry>,
    pub backup_active_tab: BackupTab,
    pub is_backup_running: bool,
    backup_progress_rx: Option<Arc<Mutex<mpsc::UnboundedReceiver<BackupProgress>>>>,
    backup_cancelled: Arc<AtomicBool>,

    // Archive state
    pub archive_cancelled: Arc<AtomicBool>,

    // Settings
    pub settings: AppSettings,
    pub logout_confirm_visible: bool,

    // Services
    pub github_service: Option<GitHubApiService>,
    pub git_service: Option<GitService>,
    pub storage_service: StorageService,

    // Theme
    pub current_theme: AppTheme,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    NavigateTo(Page),

    // Auth
    TokenInputChanged(String),
    LoginSubmit,
    LoginResult(Result<GitHubUser, String>),
    LogoutRequested,
    LogoutConfirmed,
    LogoutCancelled,

    // Data loading
    InitComplete(Result<InitData, String>),
    ReposLoaded(Result<Vec<GitHubRepo>, String>),
    OrgsLoaded(Result<Vec<GitHubOrg>, String>),
    EventsLoaded(Result<Vec<GitHubEvent>, String>),

    // Repository page
    RepoSearchChanged(String),
    RepoOwnerFilterChanged(String),
    RepoVisibilityFilterChanged(VisibilityFilter),
    RepoLanguageFilterChanged(String),
    RepoSortByChanged(SortOption),
    RepoToggleSortDirection,
    RepoToggleSelection(i64),
    RepoSelectAll,
    RepoDeselectAll,

    // Backup
    BackupTabChanged(BackupTab),
    BackupSelectFolder,
    BackupFolderSelected(Option<String>),
    BackupCloneTypeChanged(String),
    BackupConcurrentChanged(u8),
    BackupIncludeForksChanged(bool),
    BackupIncludeArchivedChanged(bool),
    BackupSplitByChanged(String),
    BackupCreateZipChanged(bool),
    BackupCompressionChanged(u8),
    BackupStart,
    BackupProgressUpdate(BackupProgress),
    BackupComplete,
    BackupCancel,
    BackupHistoryLoaded(Result<Vec<BackupHistoryEntry>, String>),
    BackupOpenFolder(String),
    ClearBackupHistory,

    // Settings
    ThemeChanged(AppTheme),
    NotificationsChanged(bool),
    SettingsSaved(Result<(), String>),
    DefaultFolderSelect,
    DefaultFolderSelected(Option<String>),

    // External
    OpenUrl(String),

    // Tick for backup timer
    Tick,
}

#[derive(Debug, Clone)]
pub struct InitData {
    pub user: GitHubUser,
    pub repos: Vec<GitHubRepo>,
    pub orgs: Vec<GitHubOrg>,
    pub events: Vec<GitHubEvent>,
}

impl GoshApp {
    pub fn new() -> (Self, Task<Message>) {
        let storage_service = StorageService::new();
        let settings = storage_service.get_settings().unwrap_or_default();
        let backup_history = storage_service.get_backup_history().unwrap_or_default();

        let current_theme = match settings.theme.as_str() {
            "light" => AppTheme::Light,
            "dark" => AppTheme::Dark,
            _ => AppTheme::System,
        };

        let backup_options = BackupOptions {
            destination: settings.default_backup_location.clone(),
            clone_type: settings
                .default_backup_options
                .clone_type
                .clone()
                .unwrap_or_else(|| "full".to_string()),
            include_forks: settings.default_backup_options.include_forks,
            include_archived: settings.default_backup_options.include_archived,
            split_by: settings.default_backup_options.split_by.clone(),
            create_zip: settings.default_backup_options.create_zip,
            zip_compression: settings.default_backup_options.zip_compression,
            exclude_patterns: settings.default_backup_options.exclude_patterns.clone(),
            max_concurrent: settings.default_backup_options.max_concurrent,
        };

        let app = Self {
            current_page: Page::Auth,
            is_authenticated: false,
            is_loading: true,
            auth_status: None,
            token_input: String::new(),
            user: None,
            repos: Vec::new(),
            orgs: Vec::new(),
            events: Vec::new(),
            selected_repos: HashSet::new(),
            repo_search: String::new(),
            repo_owner_filter: String::new(),
            repo_visibility_filter: VisibilityFilter::All,
            repo_language_filter: String::new(),
            repo_sort_by: SortOption::Updated,
            repo_sort_direction: SortDirection::Descending,
            backup_options,
            backup_progress: None,
            backup_history,
            backup_active_tab: BackupTab::Options,
            is_backup_running: false,
            backup_progress_rx: None,
            backup_cancelled: Arc::new(AtomicBool::new(false)),
            archive_cancelled: Arc::new(AtomicBool::new(false)),
            settings,
            logout_confirm_visible: false,
            github_service: None,
            git_service: None,
            storage_service: storage_service.clone(),
            current_theme,
        };

        // Try auto-login from stored token
        let startup_task = Task::perform(
            async move {
                let token = storage_service
                    .get_token()
                    .map_err(|e| e.to_string())?;

                if let Some(token) = token {
                    let service = GitHubApiService::new(&token);
                    let user = service
                        .get_authenticated_user()
                        .await
                        .map_err(|e| e.to_string())?;
                    let repos = service
                        .get_all_repos()
                        .await
                        .map_err(|e| e.to_string())?;
                    let orgs = service
                        .get_organizations()
                        .await
                        .map_err(|e| e.to_string())?;
                    let events = service
                        .get_user_events(&user.login, 20)
                        .await
                        .map_err(|e| e.to_string())?;

                    Ok(InitData {
                        user,
                        repos,
                        orgs,
                        events,
                    })
                } else {
                    Err("No stored token".to_string())
                }
            },
            Message::InitComplete,
        );

        (app, startup_task)
    }

    pub fn title(&self) -> String {
        "Gosh GitHub Backup Manager".to_string()
    }

    pub fn theme(&self) -> Theme {
        theme::get_theme(self.current_theme)
    }

    pub fn subscription(&self) -> Subscription<Message> {
        let mut subs = vec![];

        if self.is_backup_running {
            // Poll for backup progress
            if let Some(ref rx) = self.backup_progress_rx {
                let rx = rx.clone();
                subs.push(Subscription::run_with_id(
                    "backup-progress",
                    iced::stream::channel(100, move |mut output| async move {
                        let mut receiver = rx.lock().await;
                        loop {
                            match receiver.recv().await {
                                Some(progress) => {
                                    let is_done = !progress.is_running;
                                    let _ = output
                                        .send(Message::BackupProgressUpdate(progress))
                                        .await;
                                    if is_done {
                                        let _ = output.send(Message::BackupComplete).await;
                                        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                                        break;
                                    }
                                }
                                None => {
                                    let _ = output.send(Message::BackupComplete).await;
                                    break;
                                }
                            }
                        }
                        std::future::pending::<()>().await;
                    }),
                ));
            }

            // Tick every second for elapsed time
            subs.push(
                iced::time::every(std::time::Duration::from_secs(1)).map(|_| Message::Tick),
            );
        }

        Subscription::batch(subs)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            // Navigation
            Message::NavigateTo(page) => {
                self.current_page = page;
                if page == Page::Backup {
                    let storage = self.storage_service.clone();
                    return Task::perform(
                        async move {
                            storage
                                .get_backup_history()
                                .map_err(|e| e.to_string())
                        },
                        Message::BackupHistoryLoaded,
                    );
                }
                Task::none()
            }

            // Auth
            Message::TokenInputChanged(value) => {
                self.token_input = value;
                Task::none()
            }
            Message::LoginSubmit => {
                if self.token_input.is_empty() {
                    return Task::none();
                }
                self.is_loading = true;
                self.auth_status = None;
                let token = self.token_input.clone();
                let storage = self.storage_service.clone();

                Task::perform(
                    async move {
                        let service = GitHubApiService::new(&token);
                        let validation = service
                            .validate_token()
                            .await
                            .map_err(|e| e.to_string())?;

                        if !validation.valid {
                            return Err("Invalid token. Please check and try again.".to_string());
                        }

                        storage.set_token(&token).map_err(|e| e.to_string())?;

                        let user = service
                            .get_authenticated_user()
                            .await
                            .map_err(|e| e.to_string())?;
                        Ok(user)
                    },
                    Message::LoginResult,
                )
            }
            Message::LoginResult(result) => {
                self.is_loading = false;
                match result {
                    Ok(user) => {
                        let token = self.token_input.clone();
                        self.github_service = Some(GitHubApiService::new(&token));
                        self.git_service = Some(GitService::new(&token));
                        self.user = Some(user.clone());
                        self.is_authenticated = true;
                        self.current_page = Page::Dashboard;
                        self.token_input.clear();

                        let service = self.github_service.clone().unwrap();
                        let username = user.login.clone();

                        Task::batch([
                            Task::perform(
                                async move {
                                    service.get_all_repos().await.map_err(|e| e.to_string())
                                },
                                Message::ReposLoaded,
                            ),
                            {
                                let service = self.github_service.clone().unwrap();
                                Task::perform(
                                    async move {
                                        service.get_organizations().await.map_err(|e| e.to_string())
                                    },
                                    Message::OrgsLoaded,
                                )
                            },
                            {
                                let service = self.github_service.clone().unwrap();
                                Task::perform(
                                    async move {
                                        service
                                            .get_user_events(&username, 20)
                                            .await
                                            .map_err(|e| e.to_string())
                                    },
                                    Message::EventsLoaded,
                                )
                            },
                        ])
                    }
                    Err(e) => {
                        self.auth_status = Some(e);
                        Task::none()
                    }
                }
            }

            Message::InitComplete(result) => {
                self.is_loading = false;
                match result {
                    Ok(data) => {
                        // Restore services from stored token
                        if let Ok(Some(token)) = self.storage_service.get_token() {
                            self.github_service = Some(GitHubApiService::new(&token));
                            self.git_service = Some(GitService::new(&token));
                        }
                        self.user = Some(data.user);
                        self.repos = data.repos;
                        self.orgs = data.orgs;
                        self.events = data.events;
                        self.is_authenticated = true;
                        self.current_page = Page::Dashboard;
                    }
                    Err(_) => {
                        // No stored token or token expired, show auth page
                        self.current_page = Page::Auth;
                    }
                }
                Task::none()
            }

            Message::ReposLoaded(result) => {
                if let Ok(repos) = result {
                    self.repos = repos;
                }
                Task::none()
            }
            Message::OrgsLoaded(result) => {
                if let Ok(orgs) = result {
                    self.orgs = orgs;
                }
                Task::none()
            }
            Message::EventsLoaded(result) => {
                if let Ok(events) = result {
                    self.events = events;
                }
                Task::none()
            }

            // Logout
            Message::LogoutRequested => {
                self.logout_confirm_visible = true;
                Task::none()
            }
            Message::LogoutCancelled => {
                self.logout_confirm_visible = false;
                Task::none()
            }
            Message::LogoutConfirmed => {
                self.logout_confirm_visible = false;
                self.is_authenticated = false;
                self.current_page = Page::Auth;
                self.user = None;
                self.repos.clear();
                self.orgs.clear();
                self.events.clear();
                self.selected_repos.clear();
                self.github_service = None;
                self.git_service = None;
                let _ = self.storage_service.clear_token();
                Task::none()
            }

            // Repository filters
            Message::RepoSearchChanged(val) => {
                self.repo_search = val;
                Task::none()
            }
            Message::RepoOwnerFilterChanged(val) => {
                self.repo_owner_filter = if val == "All Owners" {
                    String::new()
                } else {
                    val
                };
                Task::none()
            }
            Message::RepoVisibilityFilterChanged(val) => {
                self.repo_visibility_filter = val;
                Task::none()
            }
            Message::RepoLanguageFilterChanged(val) => {
                self.repo_language_filter = if val == "All Languages" {
                    String::new()
                } else {
                    val
                };
                Task::none()
            }
            Message::RepoSortByChanged(val) => {
                self.repo_sort_by = val;
                Task::none()
            }
            Message::RepoToggleSortDirection => {
                self.repo_sort_direction = match self.repo_sort_direction {
                    SortDirection::Ascending => SortDirection::Descending,
                    SortDirection::Descending => SortDirection::Ascending,
                };
                Task::none()
            }
            Message::RepoToggleSelection(id) => {
                if self.selected_repos.contains(&id) {
                    self.selected_repos.remove(&id);
                } else {
                    self.selected_repos.insert(id);
                }
                Task::none()
            }
            Message::RepoSelectAll => {
                let ids: Vec<i64> = self.get_filtered_repos().iter().map(|r| r.id).collect();
                for id in ids {
                    self.selected_repos.insert(id);
                }
                Task::none()
            }
            Message::RepoDeselectAll => {
                self.selected_repos.clear();
                Task::none()
            }

            // Backup options
            Message::BackupTabChanged(tab) => {
                self.backup_active_tab = tab;
                Task::none()
            }
            Message::BackupSelectFolder => {
                Task::perform(
                    async {
                        let handle = rfd::AsyncFileDialog::new()
                            .set_title("Select Backup Destination")
                            .pick_folder()
                            .await;
                        handle.map(|h| h.path().to_string_lossy().to_string())
                    },
                    Message::BackupFolderSelected,
                )
            }
            Message::BackupFolderSelected(path) => {
                if let Some(path) = path {
                    self.backup_options.destination = path;
                }
                Task::none()
            }
            Message::BackupCloneTypeChanged(val) => {
                self.backup_options.clone_type = val;
                Task::none()
            }
            Message::BackupConcurrentChanged(val) => {
                self.backup_options.max_concurrent = Some(val);
                Task::none()
            }
            Message::BackupIncludeForksChanged(val) => {
                self.backup_options.include_forks = Some(val);
                Task::none()
            }
            Message::BackupIncludeArchivedChanged(val) => {
                self.backup_options.include_archived = Some(val);
                Task::none()
            }
            Message::BackupSplitByChanged(val) => {
                self.backup_options.split_by = Some(val);
                Task::none()
            }
            Message::BackupCreateZipChanged(val) => {
                self.backup_options.create_zip = Some(val);
                Task::none()
            }
            Message::BackupCompressionChanged(val) => {
                self.backup_options.zip_compression = Some(val);
                Task::none()
            }

            // Backup execution
            Message::BackupStart => {
                if self.is_backup_running {
                    return Task::none();
                }

                let git_service = match self.git_service.clone() {
                    Some(s) => s,
                    None => return Task::none(),
                };

                let selected_ids = self.selected_repos.clone();
                let repos: Vec<GitHubRepo> = self
                    .repos
                    .iter()
                    .filter(|r| selected_ids.contains(&r.id))
                    .cloned()
                    .collect();

                if repos.is_empty() {
                    return Task::none();
                }

                let options = self.backup_options.clone();
                let storage = self.storage_service.clone();

                // Create progress channel
                let (tx, rx) = mpsc::unbounded_channel::<BackupProgress>();
                self.backup_progress_rx = Some(Arc::new(Mutex::new(rx)));
                self.is_backup_running = true;
                self.backup_active_tab = BackupTab::Progress;
                self.backup_cancelled.store(false, Ordering::Relaxed);

                let cancel_flag = self.backup_cancelled.clone();
                let archive_cancel = self.archive_cancelled.clone();

                // Initialize progress
                let repo_progress: Vec<RepoBackupProgress> = repos
                    .iter()
                    .map(|r| RepoBackupProgress {
                        repo_id: r.id,
                        repo_name: r.full_name.clone(),
                        status: "pending".to_string(),
                        progress: None,
                        error: None,
                    })
                    .collect();

                let initial_progress = BackupProgress {
                    total_repos: repos.len() as u32,
                    completed_repos: 0,
                    failed_repos: 0,
                    current_repo: None,
                    repos: repo_progress,
                    start_time: chrono::Utc::now().timestamp_millis(),
                    is_running: true,
                    is_cancelled: false,
                };
                self.backup_progress = Some(initial_progress.clone());
                let _ = tx.send(initial_progress);

                // Spawn backup task
                tokio::spawn(async move {
                    perform_backup(repos, options, git_service, storage, tx, cancel_flag, archive_cancel).await;
                });

                Task::none()
            }

            Message::BackupProgressUpdate(progress) => {
                self.backup_progress = Some(progress);
                Task::none()
            }

            Message::BackupComplete => {
                self.is_backup_running = false;
                self.backup_progress_rx = None;
                // Reload history
                let storage = self.storage_service.clone();
                Task::perform(
                    async move {
                        storage.get_backup_history().map_err(|e| e.to_string())
                    },
                    Message::BackupHistoryLoaded,
                )
            }

            Message::BackupCancel => {
                self.backup_cancelled.store(true, Ordering::Relaxed);
                Task::none()
            }

            Message::BackupHistoryLoaded(result) => {
                if let Ok(history) = result {
                    self.backup_history = history;
                }
                Task::none()
            }

            Message::BackupOpenFolder(path) => {
                let _ = open::that(&path);
                Task::none()
            }

            Message::ClearBackupHistory => {
                let _ = self.storage_service.clear_backup_history();
                self.backup_history.clear();
                Task::none()
            }

            // Settings
            Message::ThemeChanged(theme) => {
                self.current_theme = theme;
                self.settings.theme = match theme {
                    AppTheme::Light => "light".to_string(),
                    AppTheme::Dark => "dark".to_string(),
                    AppTheme::System => "system".to_string(),
                };
                let settings = self.settings.clone();
                let storage = self.storage_service.clone();
                Task::perform(
                    async move { storage.set_settings(&settings).map_err(|e| e.to_string()) },
                    Message::SettingsSaved,
                )
            }
            Message::NotificationsChanged(val) => {
                self.settings.notifications = val;
                let settings = self.settings.clone();
                let storage = self.storage_service.clone();
                Task::perform(
                    async move { storage.set_settings(&settings).map_err(|e| e.to_string()) },
                    Message::SettingsSaved,
                )
            }
            Message::SettingsSaved(_) => Task::none(),
            Message::DefaultFolderSelect => {
                Task::perform(
                    async {
                        let handle = rfd::AsyncFileDialog::new()
                            .set_title("Select Default Backup Location")
                            .pick_folder()
                            .await;
                        handle.map(|h| h.path().to_string_lossy().to_string())
                    },
                    Message::DefaultFolderSelected,
                )
            }
            Message::DefaultFolderSelected(path) => {
                if let Some(path) = path {
                    self.settings.default_backup_location = path;
                    let settings = self.settings.clone();
                    let storage = self.storage_service.clone();
                    return Task::perform(
                        async move { storage.set_settings(&settings).map_err(|e| e.to_string()) },
                        Message::SettingsSaved,
                    );
                }
                Task::none()
            }

            // External
            Message::OpenUrl(url) => {
                let _ = open::that(&url);
                Task::none()
            }

            Message::Tick => {
                // Just triggers a re-render for elapsed time display
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if self.is_loading && !self.is_authenticated {
            return container(
                text("Loading...").size(16)
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into();
        }

        if !self.is_authenticated {
            return self.view_auth();
        }

        let sidebar = sidebar::view(self.current_page);
        let content = match self.current_page {
            Page::Dashboard => self.view_dashboard(),
            Page::Repositories => self.view_repositories(),
            Page::Backup => self.view_backup(),
            Page::Settings => self.view_settings(),
            Page::About => self.view_about(),
            Page::Auth => unreachable!(),
        };

        row![sidebar, content].into()
    }
}

async fn perform_backup(
    repos: Vec<GitHubRepo>,
    options: BackupOptions,
    git_service: GitService,
    storage: StorageService,
    progress_tx: mpsc::UnboundedSender<BackupProgress>,
    cancel_flag: Arc<AtomicBool>,
    archive_cancel: Arc<AtomicBool>,
) {
    let max_concurrent = options.max_concurrent.unwrap_or(3).max(1) as usize;
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let progress = Arc::new(Mutex::new(BackupProgress {
        total_repos: repos.len() as u32,
        completed_repos: 0,
        failed_repos: 0,
        current_repo: None,
        repos: repos
            .iter()
            .map(|r| RepoBackupProgress {
                repo_id: r.id,
                repo_name: r.full_name.clone(),
                status: "pending".to_string(),
                progress: None,
                error: None,
            })
            .collect(),
        start_time: chrono::Utc::now().timestamp_millis(),
        is_running: true,
        is_cancelled: false,
    }));

    let mut handles = Vec::with_capacity(repos.len());

    for repo in repos.iter().cloned() {
        if cancel_flag.load(Ordering::Relaxed) {
            break;
        }

        let permit = match semaphore.clone().acquire_owned().await {
            Ok(permit) => permit,
            Err(_) => break,
        };

        if cancel_flag.load(Ordering::Relaxed) {
            drop(permit);
            break;
        }

        let mut dest_path = options.destination.clone();
        if options.split_by.as_deref() == Some("owner") {
            let owner = repo.full_name.split('/').next().unwrap_or("unknown");
            dest_path = format!("{}/{}", dest_path, owner);
        }

        let repo_name = repo.full_name.split('/').last().unwrap_or(&repo.name);
        let full_dest = format!("{}/{}", dest_path, repo_name);

        // Update current repo
        {
            let mut p = progress.lock().await;
            p.current_repo = Some(repo.full_name.clone());
            if let Some(rp) = p.repos.iter_mut().find(|r| r.repo_id == repo.id) {
                rp.status = "cloning".to_string();
            }
            let _ = progress_tx.send(p.clone());
        }

        let git = git_service.clone();
        let progress_clone = progress.clone();
        let tx_clone = progress_tx.clone();
        let repo_clone = repo.clone();
        let clone_type = options.clone_type.clone();

        let handle = tokio::spawn(async move {
            let result = git.clone_repo(&repo_clone.clone_url, &full_dest, &clone_type).await;

            {
                let mut p = progress_clone.lock().await;
                if let Some(rp) = p.repos.iter_mut().find(|r| r.repo_id == repo_clone.id) {
                    match result {
                        Ok(_) => {
                            rp.status = "complete".to_string();
                            rp.progress = Some(100);
                            p.completed_repos += 1;
                        }
                        Err(e) => {
                            rp.status = "failed".to_string();
                            rp.error = Some(e);
                            p.failed_repos += 1;
                        }
                    }
                }
                let _ = tx_clone.send(p.clone());
            }

            drop(permit);
        });

        handles.push(handle);
    }

    for handle in handles {
        let _ = handle.await;
    }

    // Handle zip creation if enabled
    if options.create_zip.unwrap_or(false) {
        let completed_count = {
            let p = progress.lock().await;
            p.completed_repos
        };

        if completed_count > 0 {
            {
                let mut p = progress.lock().await;
                p.current_repo = Some("Creating zip archive...".to_string());
                let _ = progress_tx.send(p.clone());
            }

            let username = repos
                .first()
                .map(|r| r.full_name.split('/').next().unwrap_or("backup"))
                .unwrap_or("backup");
            let date_str = chrono::Utc::now().format("%Y-%m-%d").to_string();

            let parent_dir = std::path::Path::new(&options.destination)
                .parent()
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|| options.destination.clone());
            let folder_name = std::path::Path::new(&options.destination)
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "backup".to_string());

            let zip_path = format!(
                "{}/{}-{}-{}.zip",
                parent_dir, folder_name, username, date_str
            );

            let compression = options.zip_compression.unwrap_or(6);
            let exclude = options.exclude_patterns.clone().unwrap_or_default();
            let (archive_tx, _archive_rx) = mpsc::unbounded_channel();

            archive_cancel.store(false, Ordering::Relaxed);

            let _ = ArchiveService::create_archive(
                &options.destination,
                &zip_path,
                compression as u32,
                &exclude,
                archive_tx,
                Some(archive_cancel),
            )
            .await;
        }
    }

    // Finalize
    {
        let mut p = progress.lock().await;
        p.is_running = false;
        p.current_repo = None;

        let history_entry = BackupHistoryEntry {
            id: chrono::Utc::now().timestamp_millis().to_string(),
            date: chrono::Utc::now().to_rfc3339(),
            repo_count: p.total_repos,
            total_size: 0,
            duration: chrono::Utc::now().timestamp_millis() - p.start_time,
            destination: options.destination.clone(),
            status: if p.failed_repos == 0 {
                "complete".to_string()
            } else if p.completed_repos > 0 {
                "partial".to_string()
            } else {
                "failed".to_string()
            },
            failed_repos: p
                .repos
                .iter()
                .filter(|r| r.status == "failed")
                .map(|r| r.repo_name.clone())
                .collect(),
            options: options.clone(),
        };

        let _ = storage.add_backup_history(history_entry);
        let _ = progress_tx.send(p.clone());
    }
}
