use arboard::Clipboard;
use enigo::{Enigo, Mouse};
use screenshots::Screen;
use tauri::{AppHandle, Emitter, Manager};

#[tauri::command]
async fn enter_picker_mode(app: AppHandle) -> Result<(), String> {
    if let Some(picker_window) = app.get_webview_window("picker") {
        picker_window
            .set_focus()
            .map_err(|e| e.to_string())?;
        return Ok(());
    }

    let main_window = app
        .get_webview_window("main")
        .ok_or("Could not get main window")?;
    main_window.hide().map_err(|e| e.to_string())?;

    let _picker_window =
        tauri::WebviewWindowBuilder::new(&app, "picker", tauri::WebviewUrl::App("dropper.html".into()))
            .fullscreen(true)
            .transparent(true)
            .decorations(false)
            .always_on_top(true)
            .build()
            .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
async fn capture_color_and_close(app: AppHandle) -> Result<(), String> {
    let enigo = Enigo::new(&Default::default()).map_err(|e| e.to_string())?;
    let (x, y) = enigo.location().map_err(|e| e.to_string())?;
    let screen = Screen::from_point(x, y).map_err(|e| e.to_string())?;
    let image = screen
        .capture_area(x, y, 1, 1)
        .map_err(|e| e.to_string())?;

    let main_window = app
        .get_webview_window("main")
        .ok_or("Could not get main window")?;

    if let Some(picker_window) = app.get_webview_window("picker") {
        picker_window.close().map_err(|e| e.to_string())?;
    }
    main_window.show().map_err(|e| e.to_string())?;
    main_window.set_focus().map_err(|e| e.to_string())?;

    let pixels = image.as_flat_samples();
    let rgba = pixels.as_slice();

    if rgba.len() >= 3 {
        let r = rgba[0];
        let g = rgba[1];
        let b = rgba[2];
        let hex_color = format!("#{:02X}{:02X}{:02X}", r, g, b);
        main_window
            .emit("color-picked", hex_color)
            .map_err(|e| e.to_string())?;
    } else {
        main_window
            .emit("color-pick-failed", ())
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[tauri::command]
async fn cancel_picker(app: AppHandle) -> Result<(), String> {
    if let Some(picker_window) = app.get_webview_window("picker") {
        picker_window.close().map_err(|e| e.to_string())?;
    }
    if let Some(main_window) = app.get_webview_window("main") {
        main_window.show().map_err(|e| e.to_string())?;
        main_window.set_focus().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[tauri::command]
fn copy_to_clipboard(text: String) -> Result<(), String> {
    let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
    clipboard.set_text(text).map_err(|e| e.to_string())?;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            enter_picker_mode,
            capture_color_and_close,
            cancel_picker,
            copy_to_clipboard
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
