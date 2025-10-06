#![feature(generic_const_exprs)]
pub mod gui;
pub mod i18n;
pub mod tools_api;
pub mod cli;
use crate::tools_api::serde_pe::DangerousFunction;
use std::path::PathBuf;
use std::sync::LazyLock;

/// 全局配置文件
pub static DANGEROUS_FUNCTION_TOML_PATH: LazyLock<DangerousFunction> = LazyLock::new(|| {
    let mut path = PathBuf::from("./");
    path.push("DangerFunc.toml");
    DangerousFunction::from_file_info(&path).unwrap_or_default()
});
