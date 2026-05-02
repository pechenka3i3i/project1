#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod commands;

use commands::greet;
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent},
    Manager,
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            #[cfg(target_os = "windows")]
            {
                use tauri::Manager;
                let window = app.get_webview_window("main").unwrap();
                
                // Apply acrylic effect on Windows
                apply_acrylic_effect(&window);
            }
            
            let exit_menu_item = MenuItem::with_id(
                app,
                "exit",
                "Exit",
                true,
                None::<&str>,
            )?;

            let menu = Menu::with_items(app, &[&exit_menu_item])?;

            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "exit" => {
                        app.exit(0);
                    }
                    _ => {}
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(target_os = "windows")]
fn apply_acrylic_effect(window: &tauri::WebviewWindow) {
    use windows::Win32::Graphics::Dwm::{DwmEnableBlurBehindWindow, DWM_BB_ENABLE, DWM_BLURBEHIND};
    use windows::Win32::Foundation::HWND;

    unsafe {
        let hwnd = HWND(window.hwnd().unwrap().0);
        
        // Enable blur behind for acrylic-like effect
        let blur_behind = DWM_BLURBEHIND {
            dwFlags: DWM_BB_ENABLE,
            fEnable: 1,
            hRgnBlur: std::ptr::null_mut(),
            fTransitionOnMaximized: 0,
        };

        let result = DwmEnableBlurBehindWindow(hwnd, &blur_behind);
        
        if result.is_err() {
            eprintln!("Failed to enable blur behind: {:?}", result.err());
        }
    }
}
