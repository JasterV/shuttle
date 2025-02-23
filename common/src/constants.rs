//! Shared constants used across Shuttle crates

/// Where executables are moved to in order to persist across deploys, relative to workspace root
pub const EXECUTABLE_DIRNAME: &str = ".shuttle-executables";
/// Where general files will persist across deploys, relative to workspace root. Used by plugins.
pub const STORAGE_DIRNAME: &str = ".shuttle-storage";

// URLs
pub const API_URL_LOCAL: &str = "http://localhost:8001";
pub const API_URL_PRODUCTION: &str = "https://api.shuttle.rs";
#[cfg(debug_assertions)]
pub const API_URL_DEFAULT: &str = API_URL_LOCAL;
#[cfg(not(debug_assertions))]
pub const API_URL_DEFAULT: &str = API_URL_PRODUCTION;

pub const SHUTTLE_STATUS_URL: &str = "https://status.shuttle.rs";
pub const SHUTTLE_LOGIN_URL: &str = "https://console.shuttle.rs/new-project";
pub const SHUTTLE_GH_ISSUE_URL: &str = "https://github.com/shuttle-hq/shuttle/issues/new/choose";
pub const SHUTTLE_INSTALL_DOCS_URL: &str = "https://docs.shuttle.rs/getting-started/installation";
pub const SHUTTLE_CLI_DOCS_URL: &str = "https://docs.shuttle.rs/getting-started/shuttle-commands";
pub const SHUTTLE_IDLE_DOCS_URL: &str = "https://docs.shuttle.rs/getting-started/idle-projects";
pub const SHUTTLE_EXAMPLES_README: &str =
    "https://github.com/shuttle-hq/shuttle-examples#how-to-clone-run-and-deploy-an-example";

// Crate name for checking cargo metadata
pub const RUNTIME_NAME: &str = "shuttle-runtime";

/// Current version field in requests to provisioner
pub const RESOURCE_SCHEMA_VERSION: u32 = 1;

/// Timeframe before a project is considered idle
pub const DEFAULT_IDLE_MINUTES: u64 = 30;

/// Function to set [DEFAULT_IDLE_MINUTES] as a serde default
pub const fn default_idle_minutes() -> u64 {
    DEFAULT_IDLE_MINUTES
}

pub mod limits {
    pub const MAX_PROJECTS_DEFAULT: u32 = 3;
    pub const MAX_PROJECTS_EXTRA: u32 = 15;
}
