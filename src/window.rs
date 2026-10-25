use tauri::{WebviewUrl, WebviewWindowBuilder};

const CHROME_UA: &str = "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/156.0.0.0 Safari/537.36";

pub fn build_main_window(app: &tauri::App) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load the custom CSS and wrap it in a script to inject into the page
    let custom_css = crate::config::load_custom_css(app.handle());
    let css_literal = serde_json::to_string(&custom_css)?;

    let css_inject_script = format!(
        r#"
        (function() {{
            function applyCss() {{
                const style = document.createElement('style');
                style.id = 'neonwhat-custom-css';
                style.textContent = {};

                document.head.appendChild(style);
            }}
            if (document.readyState === 'loading') {{
                window.addEventListener('DOMContentLoaded', applyCss);
            }} else {{
                applyCss();
            }}
        }})();
        "#,
        css_literal
    );

    // 2. Build the window (Removed the custom data_directory here)
    //
    #[allow(unused_variables)]
    let win = WebviewWindowBuilder::new(
        app,
        "main",
        WebviewUrl::External("https://web.whatsapp.com".parse()?),
    )
    .title("NeonWhat")
    .user_agent(CHROME_UA)
    .initialization_script(include_str!("../inject/inject.js"))
    .initialization_script(&css_inject_script)
    .inner_size(1200.0, 800.0)
    .build()?;

    #[cfg(debug_assertions)]
    win.open_devtools();


    // Let status/video audio play without a user gesture (WebKitGTK)
    #[cfg(target_os = "linux")]
    win.with_webview(|wv| {
        use webkit2gtk::{SettingsExt, WebViewExt};
        if let Some(s) = wv.inner().settings() {
            s.set_media_playback_requires_user_gesture(false);
        }
    })?;
    Ok(())
}
