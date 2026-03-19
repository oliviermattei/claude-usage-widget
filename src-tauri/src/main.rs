// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
mod store_helpers;

use tauri::{
    menu::{Menu, MenuItem},
    tray::TrayIconBuilder,
    Emitter, Manager,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            // Build tray menu
            let show_i = MenuItem::with_id(app, "show", "Show Widget", true, None::<&str>)?;
            let refresh_i = MenuItem::with_id(app, "refresh", "Refresh", true, None::<&str>)?;
            let sep1 = tauri::menu::PredefinedMenuItem::separator(app)?;
            let logout_i = MenuItem::with_id(app, "logout", "Log Out", true, None::<&str>)?;
            let sep2 = tauri::menu::PredefinedMenuItem::separator(app)?;
            let exit_i = MenuItem::with_id(app, "exit", "Exit", true, None::<&str>)?;

            let menu = Menu::with_items(app, &[&show_i, &refresh_i, &sep1, &logout_i, &sep2, &exit_i])?;

            let _tray = TrayIconBuilder::new()
                .menu(&menu)
                .tooltip("Claude Usage Widget")
                .icon(app.default_window_icon().unwrap().clone())
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                    "refresh" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("refresh-usage", ());
                        }
                    }
                    "logout" => {
                        // Clear stored credentials
                        let _ = store_helpers::delete_credentials(app);
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.emit("session-expired", ());
                        }
                    }
                    "exit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let tauri::tray::TrayIconEvent::Click { button: tauri::tray::MouseButton::Left, .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // Restore saved window position
            let handle = app.handle();
            if let Some(pos) = store_helpers::get_window_position(handle) {
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.set_position(tauri::PhysicalPosition::new(pos.0, pos.1));
                }
            }

            // Apply saved always-on-top setting (uses Win32 API on Windows to stay above taskbar)
            if let Some(window) = app.get_webview_window("main") {
                let always_on_top = store_helpers::get_setting_bool(handle, "settings.alwaysOnTop", true);
                commands::apply_always_on_top(&window, always_on_top);
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_credentials,
            commands::save_credentials,
            commands::delete_credentials,
            commands::validate_session_key,
            commands::fetch_usage_data,
            commands::get_usage_history,
            commands::get_settings,
            commands::save_settings,
            commands::get_app_version,
            commands::check_for_update,
            commands::resize_window,
            commands::minimize_window,
            commands::close_window,
            commands::set_window_position,
            commands::get_window_position,
            commands::set_compact_mode,
            commands::show_notification,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
