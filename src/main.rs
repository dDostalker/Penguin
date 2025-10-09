use clap::Parser;
use Penguin::gui::create_native_options;
use Penguin::tools_api::FileManager;
use Penguin::cli::Cli;
use std::env;
use windows::Win32::{
    System::Console::{GetConsoleWindow},

};


#[cfg(windows)]
fn hide_console_for_gui() {    
    unsafe {
        let console_window = GetConsoleWindow();
        if !console_window.is_invalid() {
            use windows::Win32::UI::WindowsAndMessaging::{ShowWindow, SW_HIDE};
            ShowWindow(console_window, SW_HIDE);
        }
    }
}

#[cfg(not(windows))]
fn hide_console_for_gui() {
    // 在非 Windows 系统上不需要处理
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        // GUI 模式：隐藏控制台窗口
        
        let native_options: eframe::NativeOptions = create_native_options();
        hide_console_for_gui();
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
