use semver::Version;
use serde::Deserialize;
use thiserror::Error;

const LATEST_RELEASE_URL: &str = "https://api.github.com/repos/6space7/end-port/releases/latest";

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct ReleaseMetadata {
    pub tag_name: String,
    pub html_url: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub availability: UpdateAvailability,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateAvailability {
    Current,
    Available { release_url: String },
}

#[derive(Debug, Error)]
pub enum UpdateError {
    #[error("failed to fetch latest release: {0}")]
    Fetch(String),
    #[error("failed to parse latest release: {0}")]
    Parse(String),
    #[error("failed to compare release versions: {0}")]
    Version(String),
}

pub fn check_for_updates() -> std::result::Result<UpdateInfo, UpdateError> {
    let release = fetch_latest_release()?;
    classify_release_update(env!("CARGO_PKG_VERSION"), release)
}

pub fn classify_release_update(
    current_version: &str,
    release: ReleaseMetadata,
) -> std::result::Result<UpdateInfo, UpdateError> {
    let current = parse_version(current_version)?;
    let latest = parse_version(&release.tag_name)?;
    let latest_version = latest.to_string();

    let availability = if latest > current {
        UpdateAvailability::Available {
            release_url: release.html_url,
        }
    } else {
        UpdateAvailability::Current
    };

    Ok(UpdateInfo {
        current_version: current.to_string(),
        latest_version,
        availability,
    })
}

fn fetch_latest_release() -> std::result::Result<ReleaseMetadata, UpdateError> {
    let response = ureq::get(LATEST_RELEASE_URL)
        .set(
            "User-Agent",
            concat!("end-port/", env!("CARGO_PKG_VERSION")),
        )
        .call()
        .map_err(|error| UpdateError::Fetch(error.to_string()))?;

    let body = response
        .into_string()
        .map_err(|error| UpdateError::Fetch(error.to_string()))?;

    serde_json::from_str(&body).map_err(|error| UpdateError::Parse(error.to_string()))
}

fn parse_version(version: &str) -> std::result::Result<Version, UpdateError> {
    Version::parse(version.trim().trim_start_matches('v'))
        .map_err(|error| UpdateError::Version(error.to_string()))
}
