use tauri::Manager;
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

pub fn setup_shortcuts(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    let toggle = Shortcut::new(Some(Modifiers::CONTROL | Modifiers::SHIFT), Code::KeyW);
    let app_handle = app.handle().clone();

    app.global_shortcut()
        .on_shortcut(toggle, move |_app, _shortcut, event| {
            if event.state() == ShortcutState::Pressed {
                if let Some(w) = app_handle.get_webview_window("main") {
                    if w.is_visible().unwrap_or(false) {
                        let _ = w.hide();
                    } else {
                        let _ = w.show();
                        let _ = w.set_focus();
                    }
                }
            }
        })
        .unwrap_or_else(|e| eprintln!("global shortcut not registered: {e}"));
    Ok(())
}
