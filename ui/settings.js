async function loadConfig() {
  if (!window.__TAURI__) return;
  const config = await window.__TAURI__.core.invoke("get_config");
  document.getElementById("close-to-tray").checked = config.close_to_tray !== false;
  document.getElementById("notifications-enabled").checked = config.notifications_enabled !== false;
  document.getElementById("shortcut-toggle").value = config.shortcut_toggle || "Ctrl+Shift+W";
}

async function saveConfig() {
  if (!window.__TAURI__) return;
  const config = {
    close_to_tray: document.getElementById("close-to-tray").checked,
    notifications_enabled: document.getElementById("notifications-enabled").checked,
    shortcut_toggle: document.getElementById("shortcut-toggle").value,
  };
  await window.__TAURI__.core.invoke("save_config", { config });
  document.getElementById("status").textContent = "Saved";
  setTimeout(() => {
    document.getElementById("status").textContent = "";
  }, 2000);
}

document.getElementById("save").addEventListener("click", saveConfig);
loadConfig();
