#![cfg_attr(
    target_os = "windows",
    cfg_attr(not(debug_assertions), windows_subsystem = "windows")
)]
use Penguin::gui::create_native_options;
use Penguin::tools_api::FileManager;
use log::debug;

fn main() {
    env_logger::init();
    debug!("Initializing Penguin");

    let native_options: eframe::NativeOptions = create_native_options();
    eframe::run_native(
        "Penguin",
        native_options,
        Box::new(|cc| Ok(Box::new(FileManager::new(cc)))),
    )
    .expect("Failed to run application");
}
