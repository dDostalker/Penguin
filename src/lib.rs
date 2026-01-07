#![feature(generic_const_exprs)]
pub mod gui;
pub mod i18n;
pub mod tools_api;
use crate::tools_api::HashInfo;
use crate::tools_api::calc::ThreadPool;
use crate::tools_api::serde_pe::DangerousFunction;
use std::cell::RefCell;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex, RwLock};
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
pub static LAST_TOAST_HEIGHT: LazyLock<RwLock<f32>> = LazyLock::new(|| RwLock::new(0.0));
