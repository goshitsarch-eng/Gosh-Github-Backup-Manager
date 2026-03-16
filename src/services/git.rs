use git2::{Cred, FetchOptions, RemoteCallbacks, Repository};
use std::path::Path;

#[derive(Clone)]
pub struct GitService {
    token: String,
}

impl GitService {
    pub fn new(token: &str) -> Self {
        Self {
            token: token.to_string(),
        }
    }

    pub async fn clone_repo(
        &self,
        repo_url: &str,
        destination: &str,
        clone_type: &str,
    ) -> Result<(), String> {
        let token = self.token.clone();
        let repo_url = repo_url.to_string();
        let destination = destination.to_string();
        let clone_type = clone_type.to_string();

        tokio::task::spawn_blocking(move || {
            Self::clone_repo_sync(&token, &repo_url, &destination, &clone_type)
        })
        .await
        .map_err(|e| format!("Task join error: {}", e))?
    }

    fn clone_repo_sync(
        token: &str,
        repo_url: &str,
        destination: &str,
        clone_type: &str,
    ) -> Result<(), String> {
        let dest_path = Path::new(destination);

        if let Some(parent) = dest_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create directory: {}", e))?;
        }

        // Check if repo already exists
        if dest_path.exists() && dest_path.join(".git").exists() {
            return Self::pull_repo_sync(token, destination);
        }

        let mut callbacks = RemoteCallbacks::new();
        let token_owned = token.to_string();
        callbacks.credentials(move |_url, username_from_url, _allowed_types| {
            Cred::userpass_plaintext(
                username_from_url.unwrap_or("git"),
                &token_owned,
            )
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fetch_options);

        if clone_type == "mirror" {
            builder.bare(true);
        }

        builder.clone(repo_url, dest_path)
            .map_err(|e| format!("Clone failed: {}", e))?;

        Ok(())
    }

    fn pull_repo_sync(token: &str, repo_path: &str) -> Result<(), String> {
        let repo = Repository::open(repo_path)
            .map_err(|e| format!("Failed to open repository: {}", e))?;

        let mut remote = repo.find_remote("origin")
            .map_err(|e| format!("Failed to find remote: {}", e))?;

        let token_clone = token.to_string();
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(move |_url, username_from_url, _allowed_types| {
            Cred::userpass_plaintext(
                username_from_url.unwrap_or("git"),
                &token_clone,
            )
        });

        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(callbacks);

        remote.fetch(&["refs/heads/*:refs/heads/*"], Some(&mut fetch_options), None)
            .map_err(|e| format!("Fetch failed: {}", e))?;

        let head = repo.head()
            .map_err(|e| format!("Failed to get HEAD: {}", e))?;

        let branch_name = head.shorthand().unwrap_or("main");

        let fetch_head = repo.find_reference("FETCH_HEAD")
            .map_err(|e| format!("Failed to find FETCH_HEAD: {}", e))?;

        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)
            .map_err(|e| format!("Failed to get annotated commit: {}", e))?;

        let (analysis, _) = repo.merge_analysis(&[&fetch_commit])
            .map_err(|e| format!("Merge analysis failed: {}", e))?;

        if analysis.is_fast_forward() {
            let refname = format!("refs/heads/{}", branch_name);
            let mut reference = repo.find_reference(&refname)
                .map_err(|e| format!("Failed to find reference: {}", e))?;

            reference.set_target(fetch_commit.id(), "Fast-forward")
                .map_err(|e| format!("Failed to set target: {}", e))?;

            repo.set_head(&refname)
                .map_err(|e| format!("Failed to set HEAD: {}", e))?;

            repo.checkout_head(Some(
                git2::build::CheckoutBuilder::new()
                    .force()
            ))
            .map_err(|e| format!("Checkout failed: {}", e))?;
        } else if analysis.is_normal() {
            repo.checkout_head(Some(
                git2::build::CheckoutBuilder::new()
                    .force()
            ))
            .map_err(|e| format!("Checkout failed: {}", e))?;
        }

        Ok(())
    }
}
