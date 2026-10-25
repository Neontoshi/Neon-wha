use std::path::PathBuf;
use tauri::Manager;

/// Get the data directory for the webview.
/// If a profile name is provided, it creates a subfolder for it (e.g., "profile-work").
pub fn get_session_dir(app: &tauri::AppHandle, profile: Option<&str>) -> Option<PathBuf> {
    let mut path = app.path().app_data_dir().ok()?;

    if let Some(p) = profile {
        path.push(format!("profile-{}", p));
    }

    // Create the directory if it doesn't exist
    if !path.exists() {
        let _ = std::fs::create_dir_all(&path);
    }

    Some(path)
}
