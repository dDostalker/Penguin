#![feature(generic_const_exprs)]
pub mod gui;
pub mod i18n;
pub mod tools_api;
pub mod cli;
use crate::tools_api::serde_pe::DangerousFunction;
use crate::tools_api::calc::ThreadPool;
use crate::tools_api::HashInfo;
use std::path::PathBuf;
use std::sync::{LazyLock, Mutex};

/// 全局配置文件
pub static DANGEROUS_FUNCTION_TOML_PATH: LazyLock<DangerousFunction> = LazyLock::new(|| {
    let mut path = PathBuf::from("./");
    path.push("DangerFunc.toml");
    DangerousFunction::from_file_info(&path).unwrap_or_default()
});

// Global thread pool
pub static GLOBAL_THREAD_POOL: LazyLock<ThreadPool> = LazyLock::new(|| {
    ThreadPool::new(3)
});

pub static GLOBAL_HASH_INFO: LazyLock<Mutex<Vec<HashInfo>>> = LazyLock::new(|| {
    Mutex::new(Vec::new())
});
