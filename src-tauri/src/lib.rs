use enigo::{Enigo, Mouse};
use screenshots::Screen;
use arboard::Clipboard;

#[tauri::command]
fn pick_color() -> Result<String, String> {
    let enigo = Enigo::new(&Default::default()).map_err(|e| e.to_string())?;
    let (x, y) = enigo.mouse_location();

    let screen = Screen::from_point(x, y).map_err(|e| e.to_string())?;

    let image = screen.capture_area(x, y, 1, 1).map_err(|e| e.to_string())?;
    
    let pixels = image.as_flat_samples();
    let rgba = pixels.as_slice();

    if rgba.len() >= 3 {
        let r = rgba[0];
        let g = rgba[1];
        let b = rgba[2];
        Ok(format!("#{:02X}{:02X}{:02X}", r, g, b))
    } else {
        Err("Failed to get pixel color".to_string())
    }
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
        .invoke_handler(tauri::generate_handler![pick_color, copy_to_clipboard])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
