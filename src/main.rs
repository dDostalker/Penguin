use Penguin::gui::{FileManager, create_native_options};

fn main() -> eframe::Result {
    let native_options: eframe::NativeOptions = create_native_options();
    eframe::run_native(
        "Penguin",
        native_options,
        Box::new(|cc| Ok(Box::new(FileManager::new(cc)))),
    )
}
