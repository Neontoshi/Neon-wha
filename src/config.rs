use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;
use tauri::Manager;

fn config_path(app: &tauri::AppHandle) -> PathBuf {
    app.path()
        .app_config_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("neonwhat.json")
}

pub fn load(app: &tauri::AppHandle) -> Value {
    let path = config_path(app);
    if let Ok(data) = fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_else(|_| default_config())
    } else {
        default_config()
    }
}

pub fn save(app: &tauri::AppHandle, config: &Value) -> Result<(), Box<dyn std::error::Error>> {
    let path = config_path(app);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&path, serde_json::to_string_pretty(config)?)?;
    Ok(())
}

// UPDATED: Safely load the custom theme CSS from the app's config directory
pub fn load_custom_css(app: &tauri::AppHandle) -> String {
    let mut path = app
        .path()
        .app_config_dir()
        .unwrap_or_else(|_| PathBuf::from("."));
    path.push("assets");
    path.push("themes");
    path.push("example.css");

    // If the file doesn't exist, return an empty string instead of panicking
    fs::read_to_string(path).unwrap_or_default()
}

fn default_config() -> Value {
    json!({
        "theme": "default",
        "close_to_tray": true,
        "notifications_enabled": true,
        "shortcut_toggle": "Ctrl+Shift+W"
    })
}
