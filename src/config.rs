use env_logger::{Builder, Env};
use std::sync::Once;
use std::{env, fmt};

/// Runtime configuration loaded from environment.
#[derive(Debug, Clone)]
pub struct SiteConfig {
    pub site_title: String,
    pub site_url: String,
    pub profile_pic: String,
    pub content_dir: String,
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
    pub fn load_config() -> Result<SiteConfig, ConfigError> {
        // Load .env if present (ignore if missing)
        let _ = dotenvy::dotenv();

        // Helpers to reduce boilerplate
        let require_env = |name: &'static str| {
            env::var(name).map_err(|_| ConfigError::MissingVar(name))
        };
        let env_or = |name: &str, default: &str| {
            env::var(name).unwrap_or_else(|_| default.to_string())
        };

        let site_title = require_env("SITE_TITLE")?;
        let site_url = require_env("SITE_URL")?;
        let profile_pic = require_env("PROFILE_PIC")?;
        let content_dir_raw = require_env("CONTENT_DIR")?;

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
            profile_pic,
            content_dir,
            address,
            port,
            logging,
        };
        println!("Current site configuration:\n{:#?}", conf); // Debugging site configuration

        // Loads the current configuration
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            // Honor existing RUST_LOG; otherwise apply default_level
            let env = Env::default().default_filter_or(&conf.logging);
            Builder::from_env(env).init();
        });
        Ok(conf)
    }
}
