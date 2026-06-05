pub mod commands;
pub mod notification_scheduler;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Manager, WindowEvent,
};
use tauri_plugin_autostart::MacosLauncher;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_notification::init())
    .plugin(tauri_plugin_store::Builder::default().build())
    .plugin(tauri_plugin_autostart::init(
        MacosLauncher::LaunchAgent,
        Some(vec![]),
    ))
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }

      // ── System tray ──────────────────────────────────────────────
      let show_item = MenuItem::with_id(app, "show", "Show OneThing", true, None::<&str>)?;
      let quit_item = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
      let tray_menu = Menu::with_items(app, &[&show_item, &quit_item])?;

      TrayIconBuilder::with_id("main-tray")
          .menu(&tray_menu)
          .tooltip("OneThing")
          .on_menu_event(|app_handle, event| {
              match event.id.as_ref() {
                  "show" => {
                      if let Some(window) = app_handle.get_webview_window("main") {
                          let _ = window.show();
                          let _ = window.set_focus();
                      }
                  }
                  "quit" => {
                      app_handle.exit(0);
                  }
                  _ => {}
              }
          })
          .on_tray_icon_event(|tray, event| {
              if let tauri::tray::TrayIconEvent::DoubleClick { .. } = event {
                  let app_handle = tray.app_handle();
                  if let Some(window) = app_handle.get_webview_window("main") {
                      let _ = window.show();
                      let _ = window.set_focus();
                  }
              }
          })
          .build(app)?;

      // ── Start background notification scheduler ──────────────────
      notification_scheduler::start(app.handle().clone());

      Ok(())
    })
    // ── Hide window on close instead of quitting ─────────────────
    .on_window_event(|window, event| {
        if let WindowEvent::CloseRequested { api, .. } = event {
            // Prevent the window from being destroyed
            api.prevent_close();
            // Hide it instead
            let _ = window.hide();
        }
    })
    .invoke_handler(tauri::generate_handler![
        commands::notifications::schedule_notification,
        commands::notifications::send_now,
        commands::notifications::cancel_notification,
        commands::notifications::set_daily_reminder_time,
        commands::window::restore_main_window,
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
