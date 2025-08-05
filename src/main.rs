use Penguin::gui::create_native_options;
use Penguin::tools_api::FileManager;

fn main() -> eframe::Result {
    let native_options: eframe::NativeOptions = create_native_options();
    eframe::run_native(
        "Penguin",
        native_options,
        Box::new(|cc| Ok(Box::new(FileManager::new(cc)))),
    )
}
