use end_port::{classify_release_update, ReleaseMetadata, UpdateAvailability};

#[test]
fn classify_release_update_detects_newer_version() {
    let update = classify_release_update(
        "0.3.0",
        ReleaseMetadata {
            tag_name: "v0.4.0".to_string(),
            html_url: "https://github.com/6space7/end-port/releases/tag/v0.4.0".to_string(),
        },
    )
    .expect("release metadata should classify");

    assert_eq!(update.current_version, "0.3.0");
    assert_eq!(update.latest_version, "0.4.0");
    assert_eq!(
        update.availability,
        UpdateAvailability::Available {
            release_url: "https://github.com/6space7/end-port/releases/tag/v0.4.0".to_string(),
        }
    );
}

#[test]
fn classify_release_update_treats_same_version_as_current() {
    let update = classify_release_update(
        "0.4.0",
        ReleaseMetadata {
            tag_name: "v0.4.0".to_string(),
            html_url: "https://github.com/6space7/end-port/releases/tag/v0.4.0".to_string(),
        },
    )
    .expect("release metadata should classify");

    assert_eq!(update.availability, UpdateAvailability::Current);
}
