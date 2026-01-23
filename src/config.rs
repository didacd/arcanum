use env_logger::{Builder, Env};
use log::info;
use std::sync::Once;
use std::{env, fmt};

/// Runtime configuration loaded from environment.
#[derive(Debug, Clone)]
pub struct SiteConfig {
    pub site_title: String,
    pub site_url: String,
    pub username: String,
    pub user_description: String,
    pub user_profile: String,
    pub profile_pic: String,
    pub content_dir: String,
    pub content_repo_url: Option<String>,
    pub git_token: Option<String>,
    pub webhook_secret: Option<String>,
    pub poll_interval: Option<u64>,
    pub address: String,
    pub port: u16,
    pub logging: String,
}

/// Represents errors that can occur during configuration loading.
#[derive(Debug)]
pub enum ConfigError {
    MissingVar(&'static str),
    InvalidPort(std::num::ParseIntError),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingVar(v) => write!(f, "Missing required env var {}", v),
            ConfigError::InvalidPort(e) => write!(f, "Invalid PORT: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

/// Load configuration from environment / .env.
/// Fails only if a required variable is absent (SITE_TITLE, CONTENT_DIR) or PORT is invalid.
impl SiteConfig {
    fn to_log_entries(&self) -> Vec<String> {
        vec![
            format!("site_title: {}", self.site_title),
            format!("site_url: {}", self.site_url),
            format!("username: {}", self.username),
            format!("user_description: {}", self.user_description),
            format!("user_profile: {}", self.user_profile),
            format!("profile_pic: {}", self.profile_pic),
            format!("content_dir: {}", self.content_dir),
            format!("content_repo_url: {:?}", self.content_repo_url),
            format!("git_token: {:?}", self.git_token.as_ref().map(|_| "****")),
            format!(
                "webhook_secret: {:?}",
                self.webhook_secret.as_ref().map(|_| "****")
            ),
            format!("poll_interval: {:?}", self.poll_interval),
            format!("address: {}", self.address),
            format!("port: {}", self.port),
            format!("logging: {}", self.logging),
        ]
    }

    pub fn load_config() -> Result<SiteConfig, ConfigError> {
        // Load .env if present (ignore if missing)
        let _ = dotenvy::dotenv();

        // Helpers to reduce boilerplate
        let sanitize = |s: String| s.trim().trim_matches('"').trim_matches('\'').to_string();

        let require_env = |name: &'static str| {
            env::var(name)
                .map(sanitize)
                .map_err(|_| ConfigError::MissingVar(name))
        };
        let env_or = |name: &str, default: &str| {
            env::var(name)
                .map(sanitize)
                .unwrap_or_else(|_| default.to_string())
        };

        let site_title = require_env("SITE_TITLE")?;
        let site_url = require_env("SITE_URL")?;
        let username = require_env("USERNAME")?;
        let user_description = env_or("USER_DESCRIPTION", "");
        let user_profile = env_or("USER_PROFILE", "https://github.com/didacd");
        let profile_pic = require_env("PROFILE_PIC")?;
        let content_dir_raw = require_env("CONTENT_DIR")?;
        let content_repo_url = env::var("CONTENT_REPO_URL").ok().map(sanitize);

        let git_token = env::var("GIT_TOKEN_FILE")
            .ok()
            .and_then(|path| std::fs::read_to_string(path).ok())
            .map(|s| s.trim().to_string())
            .or_else(|| env::var("GIT_TOKEN").ok().map(sanitize));

        // Support Kubernetes/Docker secrets mounted as files
        // Priority: WEBHOOK_SECRET_FILE (content of file) > WEBHOOK_SECRET (env var value)
        let webhook_secret = env::var("WEBHOOK_SECRET_FILE")
            .ok()
            .and_then(|path| std::fs::read_to_string(path).ok())
            .map(|s| s.trim().to_string())
            .or_else(|| env::var("WEBHOOK_SECRET").ok().map(sanitize));

        let poll_interval = env::var("POLL_INTERVAL")
            .ok()
            .and_then(|v| v.parse::<u64>().ok());

        let address = env_or("ADDRESS", "0.0.0.0");
        let port = env_or("PORT", "8080")
            .parse::<u16>()
            .map_err(ConfigError::InvalidPort)?;
        let logging = env_or("LOGGING", "info");

        // Normalize content dir to always end with a '/'
        let content_dir = if content_dir_raw.ends_with('/') {
            content_dir_raw
        } else {
            format!("{}/", content_dir_raw)
        };
        let conf = SiteConfig {
            site_title,
            site_url,
            username,
            user_description,
            user_profile,
            profile_pic,
            content_dir,
            content_repo_url,
            git_token,
            webhook_secret,
            poll_interval,
            address,
            port,
            logging,
        };

        // Loads the current configuration
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            // Honor existing RUST_LOG; otherwise apply default_level
            let env = Env::default().default_filter_or(&conf.logging);
            Builder::from_env(env).init();
        });

        for entry in conf.to_log_entries() {
            info!("{entry}");
        }
        Ok(conf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn clear_env_vars() {
        unsafe {
            env::remove_var("SITE_TITLE");
            env::remove_var("SITE_URL");
            env::remove_var("USERNAME");
            env::remove_var("PROFILE_PIC");
            env::remove_var("CONTENT_DIR");
            env::remove_var("CONTENT_REPO_URL");
            env::remove_var("GIT_TOKEN");
            env::remove_var("WEBHOOK_SECRET");
        }
    }

    #[test]
    fn test_load_config_sanitizes_quotes() {
        // Setup minimal required env vars
        clear_env_vars();
        unsafe {
            env::set_var("SITE_TITLE", "\"My Blog\"");
            env::set_var("SITE_URL", "'https://example.com'");
            env::set_var("USERNAME", "\"user\"");
            env::set_var("PROFILE_PIC", "pic.jpg");
            env::set_var("CONTENT_DIR", "\"/var/www/content/\"");

            // Test the problematic variable specifically
            env::set_var("CONTENT_REPO_URL", "\"https://github.com/user/repo\"");
            env::set_var("GIT_TOKEN", "'secret_token'");
        }

        let config = SiteConfig::load_config().expect("Failed to load config");

        assert_eq!(config.site_title, "My Blog");
        assert_eq!(config.site_url, "https://example.com");
        assert_eq!(config.content_dir, "/var/www/content/");
        assert_eq!(
            config.content_repo_url,
            Some("https://github.com/user/repo".to_string())
        );
        assert_eq!(config.git_token, Some("secret_token".to_string()));
    }
}
