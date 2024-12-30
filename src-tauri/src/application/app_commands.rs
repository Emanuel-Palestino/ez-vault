pub struct AppCommands;

impl AppCommands {
    #[tauri::command]
    pub fn run() {
        println!("AppCommands::run()");
    }
}
