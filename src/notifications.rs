use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

fn escape_markup(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// Send a native OS notification.
pub fn send_notification(app: &AppHandle, title: &str, body: &str) {
    let _ = app
        .notification()
        .builder()
        .title(title)
        .body(escape_markup(body))
        .show();
}

/// Check if the OS currently allows notifications from this app.
pub fn is_permission_granted(app: &AppHandle) -> bool {
    app.notification()
        .permission_state()
        .map(|state| state == tauri_plugin_notification::PermissionState::Granted)
        .unwrap_or(false)
}
