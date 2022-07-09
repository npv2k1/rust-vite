#![cfg_attr(
all(not(debug_assertions), target_os = "windows"),
windows_subsystem = "windows"
)]

#[derive(serde::Serialize)]
struct CustomResponse {
    message: String,
    other_val: usize,
}

#[tauri::command]
fn my_custom_command() {
    println!("I was invoked from JS!");
}

#[tauri::command]
fn my_cmd_b(number: usize,) ->  Result<CustomResponse, String> {
    println!("I was BB invoked from JS!");
    let message = format!("I was invoked from JS with number: {}", number);
       Ok(CustomResponse {
            message,
            other_val: 42 + number,
        })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![my_custom_command,my_cmd_b])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
