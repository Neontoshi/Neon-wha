mod badge;
mod commands;
mod config;
mod notifications;
mod session;
mod shortcuts;
mod tray;
mod window;

use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            if let Some(win) = app.get_webview_window("main") {
                let _ = win.show();
                let _ = win.set_focus();
            }
        }))
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            commands::notify_show,
            commands::unread_set,
            commands::open_settings,
            commands::get_config,
            commands::save_config,
        ])
        .setup(|app| {
            window::build_main_window(app)?;
            tray::setup_tray(app)?;
            shortcuts::setup_shortcuts(app)?;
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                if window.label() == "main"
                    && config::load(window.app_handle())["close_to_tray"]
                        .as_bool()
                        .unwrap_or(true)
                {
                    api.prevent_close();
                    let _ = window.hide();
                }
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running NeonWhat");
}
