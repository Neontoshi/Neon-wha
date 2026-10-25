pub fn update_badge(app: &tauri::AppHandle, count: u32) {
    if let Some(tray) = app.tray_by_id("main-tray") {
        let bytes: &[u8] = if count > 0 {
            include_bytes!("../icons/tray/unread.png")
        } else {
            include_bytes!("../icons/tray/idle.png")
        };
        if let Ok(icon) = tauri::image::Image::from_bytes(bytes) {
            let _ = tray.set_icon(Some(icon));
        }
        let tooltip = if count > 0 {
            format!("NeonWhat — {} unread", count)
        } else {
            "NeonWhat".to_string()
        };
        let _ = tray.set_tooltip(Some(&tooltip));
    }
}
