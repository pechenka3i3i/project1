--- tauri_acrylic_app/src-tauri/src/main.rs (原始)


+++ tauri_acrylic_app/src-tauri/src/main.rs (修改后)
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
    use windows::Win32::Graphics::Dwm::{
        DwmEnableBlurBehindWindow, DwmSetWindowAttribute,
        DWM_BB_ENABLE, DWM_BLURBEHIND,
        DWMWA_USE_IMMERSIVE_DARK_MODE, DWMWA_SYSTEMBACKDROP_TYPE,
        DWM_SYSTEMBACKDROP_TYPE_DSMBT_TRANSIENTWINDOW
    };
    use windows::Win32::Foundation::HWND;
    use windows::Win32::UI::WindowsAndMessaging::{
        SetWindowLongA, GWL_EXSTYLE, WS_EX_NOREDIRECTIONBITMAP
    };

    unsafe {
        let hwnd = HWND(window.hwnd().unwrap().0);

        // Remove redirection bitmap for better acrylic effect
        SetWindowLongA(hwnd, GWL_EXSTYLE, WS_EX_NOREDIRECTIONBITMAP.0 as i32);

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

        // Enable immersive dark mode for better acrylic appearance
        let dark_mode: u32 = 1;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_USE_IMMERSIVE_DARK_MODE,
            &dark_mode as *const u32 as _,
            std::mem::size_of::<u32>() as u32,
        );

        // Set system backdrop type for acrylic material (Windows 11)
        let backdrop_type: u32 = DWM_SYSTEMBACKDROP_TYPE_DSMBT_TRANSIENTWINDOW.0 as u32;
        let _ = DwmSetWindowAttribute(
            hwnd,
            DWMWA_SYSTEMBACKDROP_TYPE,
            &backdrop_type as *const u32 as _,
            std::mem::size_of::<u32>() as u32,
        );

        println!("Acrylic effect applied successfully!");
    }
}
