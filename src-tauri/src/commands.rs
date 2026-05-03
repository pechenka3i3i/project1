--- tauri_acrylic_app/src-tauri/src/commands.rs (原始)


+++ tauri_acrylic_app/src-tauri/src/commands.rs (修改后)
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct GreetArgs {
    pub name: String,
}

#[tauri::command]
pub fn greet(name: String) -> Result<String, String> {
    Ok(format!("Hello, {}! Welcome to Tauri v2 with Acrylic effect!", name))
}
