use tauri::{LogicalSize, Size, Window};

#[tauri::command]
pub async fn restore_main_window(window: Window) -> Result<(), String> {
    window.set_decorations(true).map_err(|e| e.to_string())?;
    window.set_resizable(true).map_err(|e| e.to_string())?;
    window.set_size(Size::Logical(LogicalSize { width: 1400.0, height: 800.0 })).map_err(|e| e.to_string())?;
    window.center().map_err(|e| e.to_string())?;
    Ok(())
}
