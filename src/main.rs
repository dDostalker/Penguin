use clap::Parser;
use Penguin::gui::create_native_options;
use Penguin::tools_api::FileManager;
use Penguin::cli::Cli;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        let native_options: eframe::NativeOptions = create_native_options();
        eframe::run_native(
            "Penguin",
            native_options,
            Box::new(|cc| Ok(Box::new(FileManager::new(cc)))),
        )
        .expect("Failed to run application");
    }
    else{
        let cli = Cli::parse();
        cli.execute();
    }
}
