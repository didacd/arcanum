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
}

impl GitContentManager {
    /// Creates a new manager instance.
    pub fn new(repo_url: String, local_path: String) -> Self {
        Self {
            repo_url,
            local_path,
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
                self.update_repo(&repo)
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

        // Configure fetch options (default is usually fine for public http)
        let mut fetch_options = FetchOptions::new();

        // Fetch the main branch
        remote.fetch(&["main"], Some(&mut fetch_options), None)?;

        // Find the commit we just fetched (FETCH_HEAD)
        let fetch_head = repo.find_reference("FETCH_HEAD")?;
        let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;
        let object = repo.find_object(fetch_commit.id(), None)?;

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
