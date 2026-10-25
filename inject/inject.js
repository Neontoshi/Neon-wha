// NeonWhat injected script: notification bridge + unread badge watcher.
(function () {
  if (window.top !== window) return; // main frame only
  if (window.__neonwhat_injected) return;
  window.__neonwhat_injected = true;

  var warned = false;

  // Returns true if the call was dispatched, false if IPC isn't available.
  function call(cmd, args) {
    var invoke = window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke;
    if (!invoke) {
      if (!warned) {
        warned = true;
        console.warn("[NeonWhat] Tauri IPC unavailable: check withGlobalTauri and capabilities");
      }
      return false;
    }
    invoke(cmd, args).catch(function (e) {
      console.warn("[NeonWhat] " + cmd + " failed", e);
    });
    return true;
  }

  // ---- Notification bridge ----
  function NeonWhatNotification(title, options) {
    options = options || {};
    var self = new EventTarget(); // so addEventListener/dispatchEvent/onclick don't throw
    self.title = String(title);
    self.body = String(options.body || "");
    self.tag = options.tag || "";
    self.onclick = null;
    self.close = function () {};
    call("notify_show", { title: self.title, body: self.body });
    return self;
  }
  NeonWhatNotification.permission = "granted";
  NeonWhatNotification.requestPermission = function () {
    return Promise.resolve("granted");
  };
  try {
    window.Notification = NeonWhatNotification;
  } catch (e) {
    // Notification may be non-writable in WebKitGTK
  }

  // ---- Unread count from document.title, e.g. "(3) WhatsApp" ----
  var last = -1;

  function parseUnread(title) {
    var m = /^\((\d+)\)/.exec(title || "");
    return m ? parseInt(m[1], 10) : 0;
  }

  function report() {
    var count = parseUnread(document.title);
    if (count === last) return;
    if (call("unread_set", { count: count })) last = count;
  }

  function observeTitle() {
    new MutationObserver(report).observe(document.head, {
      childList: true,
      subtree: true,
      characterData: true,
    });
    report();
  }

  if (document.readyState === "loading") {
    window.addEventListener("DOMContentLoaded", observeTitle);
  } else {
    observeTitle();
  }
})();
