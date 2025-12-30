use crate::types::{ArchiveProgress, ArchiveResult};
use std::fs::File;
use std::io;
use std::path::Path;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tauri::{AppHandle, Emitter};
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

pub struct ArchiveService;

impl ArchiveService {
    pub async fn create_archive(
        source_dir: &str,
        destination_path: &str,
        compression_level: u32,
        exclude_patterns: &[String],
        app: AppHandle,
        cancel_flag: Option<Arc<AtomicBool>>,
    ) -> Result<ArchiveResult, String> {
        let source = source_dir.to_string();
        let dest = destination_path.to_string();
        let compression = compression_level;
        let patterns = exclude_patterns.to_vec();
        let cancel = cancel_flag.clone();

        tokio::task::spawn_blocking(move || {
            Self::create_archive_sync(&source, &dest, compression, &patterns, app, cancel)
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }

    fn create_archive_sync(
        source_dir: &str,
        destination_path: &str,
        compression_level: u32,
        exclude_patterns: &[String],
        app: AppHandle,
        cancel_flag: Option<Arc<AtomicBool>>,
    ) -> Result<ArchiveResult, String> {
        let source_path = Path::new(source_dir);
        let dest_path = Path::new(destination_path);

        if Self::is_cancelled(&cancel_flag) {
            return Err("Archive cancelled".to_string());
        }

        // Create parent directory if needed
        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        // Count total files first
        let total_files = Self::count_files(source_path, exclude_patterns);

        // Create the zip file
        let file = File::create(dest_path)
            .map_err(|e| format!("Failed to create archive file: {}", e))?;

        let mut zip = ZipWriter::new(file);

        let compression = match compression_level {
            0 => zip::CompressionMethod::Stored,
            _ => zip::CompressionMethod::Deflated,
        };

        let mut processed_files = 0u32;

        // Walk the directory
        let walker = WalkDir::new(source_path)
            .into_iter()
            .filter_entry(|e| !Self::should_skip_dir(e, exclude_patterns));

        for entry in walker {
            if Self::is_cancelled(&cancel_flag) {
                drop(zip);
                let _ = std::fs::remove_file(dest_path);
                return Err("Archive cancelled".to_string());
            }

            let entry = match entry {
                Ok(e) => e,
                Err(_) => continue,
            };

            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // Check exclusions
            if Self::should_exclude(&name, exclude_patterns) {
                continue;
            }

            if entry.file_type().is_symlink() {
                continue;
            }

            // Get relative path
            let relative_path = match path.strip_prefix(source_path) {
                Ok(p) => p,
                Err(_) => continue,
            };

            let path_str = relative_path.to_string_lossy().to_string();

            if entry.file_type().is_file() {
                let file_options = SimpleFileOptions::default()
                    .compression_method(compression)
                    .unix_permissions(0o644);

                // Add file to archive
                zip.start_file(&path_str, file_options)
                    .map_err(|e| format!("Failed to start file in archive: {}", e))?;

                let mut file = File::open(path)
                    .map_err(|e| format!("Failed to open file: {}", e))?;

                io::copy(&mut file, &mut zip)
                    .map_err(|e| format!("Failed to write to archive: {}", e))?;

                processed_files += 1;

                let progress_percent = if total_files == 0 {
                    100
                } else {
                    (((processed_files as f64 / total_files as f64) * 100.0).round() as u8).min(100)
                };

                // Emit progress
                let progress = ArchiveProgress {
                    is_running: true,
                    progress: progress_percent,
                    current_file: Some(path_str),
                    total_files,
                    processed_files,
                };
                let _ = app.emit("archive:progress", &progress);
            } else if entry.file_type().is_dir() && !path_str.is_empty() {
                let dir_options = SimpleFileOptions::default()
                    .compression_method(compression)
                    .unix_permissions(0o755);

                // Add directory
                zip.add_directory(&format!("{}/", path_str), dir_options)
                    .map_err(|e| format!("Failed to add directory: {}", e))?;
            }
        }

        // Finish the archive
        zip.finish()
            .map_err(|e| format!("Failed to finish archive: {}", e))?;

        // Get the final size
        let size = std::fs::metadata(dest_path)
            .map(|m| m.len())
            .unwrap_or(0);

        Ok(ArchiveResult {
            path: destination_path.to_string(),
            size,
        })
    }

    fn count_files(dir: &Path, exclude_patterns: &[String]) -> u32 {
        let mut count = 0u32;

        let walker = WalkDir::new(dir)
            .into_iter()
            .filter_entry(|e| !Self::should_skip_dir(e, exclude_patterns));

        for entry in walker {
            let entry = match entry {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let name = entry.file_name().to_string_lossy().to_string();
            if Self::should_exclude(&name, exclude_patterns) {
                continue;
            }

            if entry.file_type().is_file() && !entry.file_type().is_symlink() {
                count += 1;
            }
        }

        count
    }

    fn should_skip_dir(entry: &walkdir::DirEntry, patterns: &[String]) -> bool {
        if entry.depth() == 0 || !entry.file_type().is_dir() {
            return false;
        }

        let name = entry.file_name().to_string_lossy().to_string();
        Self::should_exclude(&name, patterns)
    }

    fn should_exclude(name: &str, patterns: &[String]) -> bool {
        for pattern in patterns {
            if pattern.starts_with('*') {
                // Wildcard at start (e.g., *.log)
                let suffix = &pattern[1..];
                if name.ends_with(suffix) {
                    return true;
                }
            } else if pattern.ends_with('*') {
                // Wildcard at end (e.g., temp*)
                let prefix = &pattern[..pattern.len() - 1];
                if name.starts_with(prefix) {
                    return true;
                }
            } else {
                // Exact match
                if name == pattern {
                    return true;
                }
            }
        }
        false
    }

    fn is_cancelled(cancel_flag: &Option<Arc<AtomicBool>>) -> bool {
        cancel_flag
            .as_ref()
            .map(|flag| flag.load(Ordering::Relaxed))
            .unwrap_or(false)
    }
}
