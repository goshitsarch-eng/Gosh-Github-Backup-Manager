mod types;
mod github;
mod git;
mod archive;
mod storage;

use std::collections::HashSet;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tokio::sync::{Mutex, Semaphore};
use tauri::{Manager, State, AppHandle, Emitter};
use types::*;
use github::GitHubApiService;
use git::GitService;
use archive::ArchiveService;
use storage::StorageService;

// Application state
pub struct AppState {
    github_service: Mutex<Option<GitHubApiService>>,
    git_service: Mutex<Option<GitService>>,
    storage_service: StorageService,
    active_backup: Mutex<Option<ActiveBackup>>,
    archive_cancelled: Arc<AtomicBool>,
}

impl AppState {
    fn new(app_handle: AppHandle) -> Self {
        Self {
            github_service: Mutex::new(None),
            git_service: Mutex::new(None),
            storage_service: StorageService::new(app_handle),
            active_backup: Mutex::new(None),
            archive_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }
}

struct ActiveBackup {
    is_running: bool,
    is_cancelled: bool,
    progress: BackupProgress,
}

// ==================== GitHub Commands ====================

#[tauri::command]
async fn github_validate_token(
    token: String,
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<TokenValidation>, String> {
    let service = GitHubApiService::new(&token);

    match service.validate_token().await {
        Ok(result) => {
            if result.valid {
                // Store token and initialize services
                state.storage_service.set_token(&token).await.ok();
                *state.github_service.lock().await = Some(service);
                *state.git_service.lock().await = Some(GitService::new(&token));
            }
            Ok(ApiResponse::success(result))
        }
        Err(e) => Ok(ApiResponse::error(e.to_string())),
    }
}

#[tauri::command]
async fn github_get_user(
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<GitHubUser>, String> {
    let mut service_guard = state.github_service.lock().await;

    // Try to restore from stored token if not authenticated
    if service_guard.is_none() {
        if let Ok(Some(token)) = state.storage_service.get_token().await {
            *service_guard = Some(GitHubApiService::new(&token));
            *state.git_service.lock().await = Some(GitService::new(&token));
        }
    }

    match &*service_guard {
        Some(service) => match service.get_authenticated_user().await {
            Ok(user) => Ok(ApiResponse::success(user)),
            Err(e) => Ok(ApiResponse::error(e.to_string())),
        },
        None => Ok(ApiResponse::error("Not authenticated".to_string())),
    }
}

#[tauri::command]
async fn github_get_repos(
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<Vec<GitHubRepo>>, String> {
    let service_guard = state.github_service.lock().await;

    match &*service_guard {
        Some(service) => match service.get_all_repos().await {
            Ok(repos) => Ok(ApiResponse::success(repos)),
            Err(e) => Ok(ApiResponse::error(e.to_string())),
        },
        None => Ok(ApiResponse::error("Not authenticated".to_string())),
    }
}

#[tauri::command]
async fn github_get_orgs(
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<Vec<GitHubOrg>>, String> {
    let service_guard = state.github_service.lock().await;

    match &*service_guard {
        Some(service) => match service.get_organizations().await {
            Ok(orgs) => Ok(ApiResponse::success(orgs)),
            Err(e) => Ok(ApiResponse::error(e.to_string())),
        },
        None => Ok(ApiResponse::error("Not authenticated".to_string())),
    }
}

#[tauri::command]
async fn github_get_events(
    limit: Option<u32>,
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<Vec<GitHubEvent>>, String> {
    let service_guard = state.github_service.lock().await;

    match &*service_guard {
        Some(service) => match service.get_user_events(limit.unwrap_or(20)).await {
            Ok(events) => Ok(ApiResponse::success(events)),
            Err(e) => Ok(ApiResponse::error(e.to_string())),
        },
        None => Ok(ApiResponse::error("Not authenticated".to_string())),
    }
}

#[tauri::command]
async fn github_get_stats(
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<GitHubStats>, String> {
    let service_guard = state.github_service.lock().await;

    match &*service_guard {
        Some(service) => match service.get_all_repos().await {
            Ok(repos) => {
                let stats = GitHubStats {
                    stars: repos.iter().map(|r| r.stargazers_count).sum(),
                    forks: repos.iter().map(|r| r.forks_count).sum(),
                };
                Ok(ApiResponse::success(stats))
            }
            Err(e) => Ok(ApiResponse::error(e.to_string())),
        },
        None => Ok(ApiResponse::error("Not authenticated".to_string())),
    }
}

// ==================== Backup Commands ====================

#[tauri::command]
async fn backup_start(
    app: AppHandle,
    repo_ids: Vec<i64>,
    options: BackupOptions,
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<()>, String> {
    // Check if backup already in progress
    {
        let active = state.active_backup.lock().await;
        if active.as_ref().map(|a| a.is_running).unwrap_or(false) {
            return Ok(ApiResponse::error("Backup already in progress".to_string()));
        }
    }

    let repo_id_set: HashSet<i64> = repo_ids.into_iter().collect();

    // Get repos
    let repos = {
        let service_guard = state.github_service.lock().await;
        match &*service_guard {
            Some(service) => {
                match service.get_all_repos().await {
                    Ok(all_repos) => all_repos
                        .into_iter()
                        .filter(|r| repo_id_set.contains(&r.id))
                        .collect::<Vec<_>>(),
                    Err(e) => return Ok(ApiResponse::error(e.to_string())),
                }
            }
            None => return Ok(ApiResponse::error("Not authenticated".to_string())),
        }
    };

    // Initialize backup state
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

    let progress = BackupProgress {
        total_repos: repos.len() as u32,
        completed_repos: 0,
        failed_repos: 0,
        current_repo: None,
        repos: repo_progress,
        start_time: chrono::Utc::now().timestamp_millis(),
        is_running: true,
        is_cancelled: false,
    };

    {
        let mut active = state.active_backup.lock().await;
        *active = Some(ActiveBackup {
            is_running: true,
            is_cancelled: false,
            progress: progress.clone(),
        });
    }

    // Clone state for async task
    let state_clone = state.inner().clone();
    let app_clone = app.clone();

    // Start backup in background
    tokio::spawn(async move {
        perform_backup(app_clone, repos, options, state_clone).await;
    });

    Ok(ApiResponse::success(()))
}

async fn perform_backup(
    app: AppHandle,
    repos: Vec<GitHubRepo>,
    options: BackupOptions,
    state: Arc<AppState>,
) {
    let max_concurrent = options.max_concurrent.unwrap_or(3).max(1) as usize;
    let semaphore = Arc::new(Semaphore::new(max_concurrent));
    let mut handles = Vec::with_capacity(repos.len());

    for repo in repos.iter().cloned() {
        // Check if cancelled
        {
            let active = state.active_backup.lock().await;
            if active.as_ref().map(|a| a.is_cancelled).unwrap_or(false) {
                break;
            }
        }

        let permit = match semaphore.clone().acquire_owned().await {
            Ok(permit) => permit,
            Err(_) => break,
        };

        // Check if cancelled again after waiting for a slot
        {
            let active = state.active_backup.lock().await;
            if active.as_ref().map(|a| a.is_cancelled).unwrap_or(false) {
                drop(permit);
                break;
            }
        }

        // Determine destination path
        let mut dest_path = options.destination.clone();
        if options.split_by == Some("owner".to_string()) {
            let owner = repo.full_name.split('/').next().unwrap_or("unknown");
            dest_path = format!("{}/{}", dest_path, owner);
        }

        let repo_name = repo.full_name.split('/').last().unwrap_or(&repo.name);
        let full_dest = format!("{}/{}", dest_path, repo_name);

        // Update current repo
        {
            let mut active = state.active_backup.lock().await;
            if let Some(ref mut backup) = *active {
                backup.progress.current_repo = Some(repo.full_name.clone());
                if let Some(rp) = backup.progress.repos.iter_mut().find(|r| r.repo_id == repo.id) {
                    rp.status = "cloning".to_string();
                }
                let _ = app.emit("backup:progress", &backup.progress);
            }
        }

        let app_clone = app.clone();
        let state_clone = state.clone();
        let options_clone = options.clone();
        let repo_clone = repo.clone();

        let handle = tokio::spawn(async move {
            // Clone repo
            let result = {
                let git_guard = state_clone.git_service.lock().await;
                if let Some(ref git_service) = *git_guard {
                    git_service.clone_repo(
                        &repo_clone.clone_url,
                        &full_dest,
                        &options_clone.clone_type,
                    ).await
                } else {
                    Err("Git service not initialized".to_string())
                }
            };

            // Update progress
            {
                let mut active = state_clone.active_backup.lock().await;
                if let Some(ref mut backup) = *active {
                    if let Some(rp) = backup.progress.repos.iter_mut().find(|r| r.repo_id == repo_clone.id) {
                        match result {
                            Ok(_) => {
                                rp.status = "complete".to_string();
                                rp.progress = Some(100);
                                backup.progress.completed_repos += 1;
                            }
                            Err(e) => {
                                rp.status = "failed".to_string();
                                rp.error = Some(e);
                                backup.progress.failed_repos += 1;
                            }
                        }
                    }
                    let _ = app_clone.emit("backup:progress", &backup.progress);
                }
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
            let active = state.active_backup.lock().await;
            active.as_ref().map(|a| a.progress.completed_repos).unwrap_or(0)
        };

        if completed_count > 0 {
            // Update status for zip phase
            {
                let mut active = state.active_backup.lock().await;
                if let Some(ref mut backup) = *active {
                    backup.progress.current_repo = Some("Creating zip archive...".to_string());
                    let _ = app.emit("backup:progress", &backup.progress);
                }
            }

            let username = repos.first()
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

            let zip_path = format!("{}/{}-{}-{}.zip", parent_dir, folder_name, username, date_str);

            let compression = options.zip_compression.unwrap_or(6);
            let exclude = options.exclude_patterns.clone().unwrap_or_default();

            match ArchiveService::create_archive(
                &options.destination,
                &zip_path,
                compression as u32,
                &exclude,
                app.clone(),
                None,
            ).await {
                Ok(result) => {
                    let _ = app.emit("archive:complete", &result);
                }
                Err(e) => {
                    log::error!("Failed to create archive: {}", e);
                }
            }
        }
    }

    // Finalize backup
    {
        let mut active = state.active_backup.lock().await;
        if let Some(ref mut backup) = *active {
            backup.is_running = false;
            backup.progress.is_running = false;
            backup.progress.current_repo = None;

            let _ = app.emit("backup:progress", &backup.progress);

            let success = backup.progress.failed_repos == 0;
            let message = format!(
                "Backup complete: {}/{} repos",
                backup.progress.completed_repos,
                backup.progress.total_repos
            );

            // Save to history
            let history_entry = BackupHistoryEntry {
                id: chrono::Utc::now().timestamp_millis().to_string(),
                date: chrono::Utc::now().to_rfc3339(),
                repo_count: backup.progress.total_repos,
                total_size: 0,
                duration: chrono::Utc::now().timestamp_millis() - backup.progress.start_time,
                destination: options.destination.clone(),
                status: if backup.progress.failed_repos == 0 {
                    "complete".to_string()
                } else if backup.progress.completed_repos > 0 {
                    "partial".to_string()
                } else {
                    "failed".to_string()
                },
                failed_repos: backup.progress.repos
                    .iter()
                    .filter(|r| r.status == "failed")
                    .map(|r| r.repo_name.clone())
                    .collect(),
                options: options.clone(),
            };

            let _ = state.storage_service.add_backup_history(history_entry).await;

            let _ = app.emit("backup:complete", serde_json::json!({
                "success": success,
                "message": message
            }));
        }
        *active = None;
    }
}

#[tauri::command]
async fn backup_cancel(
    app: AppHandle,
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<()>, String> {
    let mut active = state.active_backup.lock().await;
    if let Some(ref mut backup) = *active {
        backup.is_cancelled = true;
        backup.progress.is_cancelled = true;
        let _ = app.emit("backup:progress", &backup.progress);
    }
    Ok(ApiResponse::success(()))
}

#[tauri::command]
async fn backup_get_history(
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<Vec<BackupHistoryEntry>>, String> {
    match state.storage_service.get_backup_history().await {
        Ok(history) => Ok(ApiResponse::success(history)),
        Err(e) => Ok(ApiResponse::error(e.to_string())),
    }
}

// ==================== Archive Commands ====================

#[tauri::command]
async fn archive_create(
    app: AppHandle,
    source: String,
    destination: String,
    options: ArchiveOptions,
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<String>, String> {
    state.archive_cancelled.store(false, Ordering::Relaxed);

    match ArchiveService::create_archive(
        &source,
        &destination,
        options.compression_level,
        &options.exclude_patterns,
        app.clone(),
        Some(state.archive_cancelled.clone()),
    ).await {
        Ok(result) => {
            let _ = app.emit("archive:complete", &result);
            Ok(ApiResponse::success(result.path))
        }
        Err(e) => Ok(ApiResponse::error(e.to_string())),
    }
}

#[tauri::command]
async fn archive_cancel(
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<()>, String> {
    state.archive_cancelled.store(true, Ordering::Relaxed);
    Ok(ApiResponse::success(()))
}

// ==================== Settings Commands ====================

#[tauri::command]
async fn settings_get(
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<AppSettings>, String> {
    match state.storage_service.get_settings().await {
        Ok(settings) => Ok(ApiResponse::success(settings)),
        Err(e) => Ok(ApiResponse::error(e.to_string())),
    }
}

#[tauri::command]
async fn settings_set(
    settings: AppSettings,
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<()>, String> {
    match state.storage_service.set_settings(&settings).await {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Ok(ApiResponse::error(e.to_string())),
    }
}

#[tauri::command]
async fn settings_get_token(
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<Option<String>>, String> {
    match state.storage_service.get_token().await {
        Ok(token) => Ok(ApiResponse::success(token)),
        Err(e) => Ok(ApiResponse::error(e.to_string())),
    }
}

#[tauri::command]
async fn settings_set_token(
    token: String,
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<()>, String> {
    // Initialize services
    *state.github_service.lock().await = Some(GitHubApiService::new(&token));
    *state.git_service.lock().await = Some(GitService::new(&token));

    // Store token and wait for it to complete
    match state.storage_service.set_token(&token).await {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Ok(ApiResponse::error(format!("Failed to store token: {}", e))),
    }
}

#[tauri::command]
async fn settings_clear_token(
    state: State<'_, Arc<AppState>>,
) -> Result<ApiResponse<()>, String> {
    *state.github_service.lock().await = None;
    *state.git_service.lock().await = None;

    match state.storage_service.clear_token().await {
        Ok(_) => Ok(ApiResponse::success(())),
        Err(e) => Ok(ApiResponse::error(e.to_string())),
    }
}

// ==================== Application Setup ====================

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logger for debug output
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_os::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let state = Arc::new(AppState::new(app.handle().clone()));
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // GitHub
            github_validate_token,
            github_get_user,
            github_get_repos,
            github_get_orgs,
            github_get_events,
            github_get_stats,
            // Backup
            backup_start,
            backup_cancel,
            backup_get_history,
            // Archive
            archive_create,
            archive_cancel,
            // Settings
            settings_get,
            settings_set,
            settings_get_token,
            settings_set_token,
            settings_clear_token,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
