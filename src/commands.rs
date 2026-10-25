use tauri::Manager;

#[tauri::command]
pub fn notify_show(app: tauri::AppHandle, title: String, body: String) {
    let enabled = crate::config::load(&app)["notifications_enabled"]
        .as_bool()
        .unwrap_or(true);
    if enabled && crate::notifications::is_permission_granted(&app) {
        crate::notifications::send_notification(&app, &title, &body);
    }
}

#[tauri::command]
pub fn unread_set(app: tauri::AppHandle, count: u32) {
    if let Some(win) = app.get_webview_window("main") {
        let title = if count > 0 {
            format!("({}) NeonWhat", count)
        } else {
            "NeonWhat".to_string()
        };
        let _ = win.set_title(&title);
    }
    crate::badge::update_badge(&app, count);
}

#[tauri::command]
pub fn open_settings(app: tauri::AppHandle) {
    if let Some(win) = app.get_webview_window("settings") {
        let _ = win.show();
        let _ = win.set_focus();
        return;
    }
    let _ = tauri::WebviewWindowBuilder::new(
        &app,
        "settings",
        tauri::WebviewUrl::App("index.html".into()),
    )
    .title("NeonWhat Settings")
    .inner_size(500.0, 400.0)
    .build();
}

#[tauri::command]
pub fn get_config(app: tauri::AppHandle) -> serde_json::Value {
    crate::config::load(&app)
}

#[tauri::command]
pub fn save_config(app: tauri::AppHandle, config: serde_json::Value) -> Result<(), String> {
    crate::config::save(&app, &config).map_err(|e| e.to_string())
}
