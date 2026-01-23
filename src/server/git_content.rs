use git2::{build::RepoBuilder, Error, FetchOptions, Repository};
use log::{error, info, warn};
use std::fs;
use std::path::Path;

/// Manages synchronization of a local directory with a remote Git repository.
///
/// This manager ensures the local content directory always mirrors the remote's `main` branch.
/// It treats the local copy as ephemeral/read-only relative to the remote source of truth.
///
/// # Behavior
/// - **Missing Directory**: Clones the repository.
/// - **Existing Valid Repository**: Fetches `origin` and performs a `hard reset` to match `origin/main`.
/// - **Corrupt/Invalid Directory**: Deletes the directory and re-clones the repository.
#[derive(Debug, Clone)]
pub struct GitContentManager {
    repo_url: String,
    local_path: String,
    git_token: Option<String>,
}

impl GitContentManager {
    /// Creates a new manager instance.
    pub fn new(repo_url: String, local_path: String, git_token: Option<String>) -> Self {
        Self {
            repo_url,
            local_path,
            git_token,
        }
    }

    /// Synchronizes the local content with the remote repository.
    ///
    /// This is a blocking operation and should be run in a blocking thread pool
    /// if called during request handling.
    pub fn sync(&self) -> Result<(), Error> {
        let path = Path::new(&self.local_path);

        if path.exists() {
            self.handle_existing_directory(path)
        } else {
            self.clone_repo(path)
        }
    }

    /// Handles the case where the target directory already exists.
    /// It attempts to open it as a git repo. If that fails (e.g. empty dir or corrupt),
    /// it cleans the directory and re-clones.
    fn handle_existing_directory(&self, path: &Path) -> Result<(), Error> {
        match Repository::open(path) {
            Ok(repo) => {
                info!("Found existing repository at {:?}", path);
                // Try to update. If it fails (e.g. corruption during fetch), treat as invalid.
                if let Err(e) = self.update_repo(&repo) {
                    warn!("Repository exists but update failed: {}. Skipping auto-repair to avoid loops.", e);
                    // Return the error instead of looping clean_and_clone
                    Err(e)
                } else {
                    Ok(())
                }
            }
            Err(e) => {
                warn!(
                    "Directory exists but is not a valid git repository: {}. Attempting to repair...",
                    e
                );
                self.clean_and_clone(path, e)
            }
        }
    }

    /// Performs a fetch and hard reset to the latest remote state.
    fn update_repo(&self, repo: &Repository) -> Result<(), Error> {
        info!("Fetching latest changes from remote...");

        // We assume 'origin' is the remote we care about
        let mut remote = repo.find_remote("origin")?;

        // Configure fetch options
        let mut fetch_options = FetchOptions::new();

        let git_token = self.git_token.clone();

        // This is crucial for handling authentication if the user provides https credentials
        // in the URL (e.g., https://user:token@github.com/repo.git).
        // Without this callback, libgit2 fails when it encounters an auth challenge.
        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |_url, username_from_url, _allowed_types| {
            if let Some(token) = &git_token {
                git2::Cred::userpass_plaintext(username_from_url.unwrap_or("git"), token)
            } else {
                git2::Cred::userpass_plaintext(username_from_url.unwrap_or("git"), "")
            }
        });
        fetch_options.remote_callbacks(callbacks);

        // Fetch the main branch
        // Refspec: map remote main to local main. This is more robust than relying on FETCH_HEAD.
        // We force update (+) to ensure local main always matches remote main, even if history was rewritten.
        // We also fetch main just in case
        if let Err(e) = remote.fetch(
            &[
                "+refs/heads/master:refs/heads/master",
                "+refs/heads/main:refs/heads/main",
            ],
            Some(&mut fetch_options),
            None,
        ) {
            // Check if error is related to authentication
            if e.class() == git2::ErrorClass::Http && e.code() == git2::ErrorCode::Auth {
                error!("Authentication failed fetching remote: {}", e);
                return Err(e);
            }

            warn!(
                "Failed to fetch remote: {}. Check if the repo is valid or corrupted.",
                e
            );
            return Err(e);
        }

        // After fetching to refs/heads/main, we find that reference directly
        let head_ref = match repo.find_reference("refs/heads/main") {
            Ok(r) => r,
            Err(_) => {
                // If refs/heads/main is missing, check remote/origin/main
                match repo.find_reference("refs/remotes/origin/master") {
                    Ok(r) => r,
                    Err(e) => {
                        warn!("Failed to find main branch reference: {}", e);
                        return Err(e);
                    }
                }
            }
        };

        let head_commit = repo.reference_to_annotated_commit(&head_ref)?;
        let object = repo.find_object(head_commit.id(), None)?;

        // Hard reset the working directory to this commit
        // This discards any local changes, which is desired for this "read-only mirror" use case
        repo.reset(&object, git2::ResetType::Hard, None)?;

        info!("Content successfully synchronized.");
        Ok(())
    }

    /// Clones the repository to the specified path.
    fn clone_repo(&self, path: &Path) -> Result<(), Error> {
        info!("Cloning {} into {:?}", self.repo_url, path);

        let mut builder = RepoBuilder::new();

        // Add credentials callback for clone
        let mut fetch_options = FetchOptions::new();
        let git_token = self.git_token.clone();

        let mut callbacks = git2::RemoteCallbacks::new();
        callbacks.credentials(move |_url, username_from_url, _allowed_types| {
            if let Some(token) = &git_token {
                git2::Cred::userpass_plaintext(username_from_url.unwrap_or("git"), token)
            } else {
                git2::Cred::userpass_plaintext(username_from_url.unwrap_or("git"), "")
            }
        });
        fetch_options.remote_callbacks(callbacks);
        builder.fetch_options(fetch_options);

        match builder.clone(&self.repo_url, path) {
            Ok(_) => {
                info!("Repository cloned successfully.");
                Ok(())
            }
            Err(e) => {
                error!("Failed to clone repository: {}", e);
                Err(e)
            }
        }
    }

    /// Helper to remove a corrupt directory and start fresh.
    fn clean_and_clone(&self, path: &Path, original_error: Error) -> Result<(), Error> {
        if let Err(fs_err) = fs::remove_dir_all(path) {
            error!(
                "Critical: Failed to remove corrupt directory {:?}: {}",
                path, fs_err
            );
            // Return original git error if we can't clean up, as that's the root cause
            return Err(original_error);
        }

        info!("Directory cleaned. Re-cloning...");
        self.clone_repo(path)
    }
}
