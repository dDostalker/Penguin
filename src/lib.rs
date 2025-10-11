#![feature(generic_const_exprs)]
pub mod cli;
pub mod gui;
pub mod i18n;
pub mod tools_api;
use crate::tools_api::HashInfo;
use crate::tools_api::calc::ThreadPool;
use crate::tools_api::serde_pe::DangerousFunction;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};
use windows::Win32::System::Console::GetConsoleWindow;
/// 全局配置文件
pub static DANGEROUS_FUNCTION_TOML_PATH: LazyLock<DangerousFunction> = LazyLock::new(|| {
    let mut path = PathBuf::from("./");
    path.push("DangerFunc.toml");
    DangerousFunction::from_file_info(&path).unwrap_or_default()
});

// Global thread pool
pub static GLOBAL_THREAD_POOL: LazyLock<ThreadPool> = LazyLock::new(|| ThreadPool::new(3));

pub static GLOBAL_HASH_INFO: LazyLock<Mutex<Vec<HashInfo>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));

#[cfg(windows)]
pub fn hide_console_for_gui() {
    unsafe {
        let console_window = GetConsoleWindow();
        if !console_window.is_invalid() {
            use windows::Win32::UI::WindowsAndMessaging::{SW_HIDE, ShowWindow};
            let _ = ShowWindow(console_window, SW_HIDE);
        }
    }
}

#[cfg(not(windows))]
pub fn hide_console_for_gui() {
    // 在非 Windows 系统上不需要处理
}
