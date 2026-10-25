fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "notify_show",
            "unread_set",
            "open_settings",
            "get_config",
            "save_config",
        ]),
    ))
    .expect("failed to run tauri-build");
}
