#![feature(generic_const_exprs)]
pub mod gui;
pub mod i18n;
pub mod tools_api;
use crate::tools_api::HashInfo;
use crate::tools_api::calc::ThreadPool;
use crate::tools_api::serde_pe::DangerousFunction;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex, RwLock};
/// 获取linux系统特定的配置文件路径
#[cfg(target_os = "linux")]
fn get_config_path() -> PathBuf {
    use std::env;

    let mut path = match env::var("HOME") {
        Ok(home_dir) => PathBuf::from(home_dir),
        Err(_) => PathBuf::from("."),
    };

    path.push(".config");
    path.push("penguin");
    path
}

/// 获取windows系统特定的配置文件路径
#[cfg(target_os = "windows")]
fn get_config_path() -> PathBuf {
    use std::env;

    let mut path = match env::var("APPDATA") {
        Ok(appdata_dir) => PathBuf::from(appdata_dir),
        Err(_) => {
            // 如果无法获取APPDATA目录，回退到当前目录
            PathBuf::from(".")
        }
    };

    path.push("penguin");
    path
}

/// 获取操作系统特定的配置文件路径
/// 对于其他不支持的操作系统，回退到当前目录
#[cfg(not(any(target_os = "linux", target_os = "windows")))]
fn get_config_path() -> PathBuf {
    PathBuf::from(".")
}

/// 全局配置文件
pub static DANGEROUS_FUNCTION_TOML_PATH: LazyLock<DangerousFunction> = LazyLock::new(|| {
    let mut path = get_config_path();
    path.push("DangerFunc.toml");
    DangerousFunction::from_file_info(&path).unwrap_or_default()
});

// Global thread pool
pub static GLOBAL_THREAD_POOL: LazyLock<ThreadPool> = LazyLock::new(|| ThreadPool::new(3));

pub static GLOBAL_HASH_INFO: LazyLock<Mutex<Vec<HashInfo>>> =
    LazyLock::new(|| Mutex::new(Vec::new()));
pub static LAST_TOAST_HEIGHT: LazyLock<RwLock<f32>> = LazyLock::new(|| RwLock::new(0.0));
