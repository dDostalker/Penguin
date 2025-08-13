#![feature(generic_const_exprs)]
pub mod gui;
pub mod i18n;
pub mod tools_api;
use crate::tools_api::serde_pe::DangerousFunction;
use std::path::PathBuf;
use std::sync::LazyLock;

pub static GLOBAL_RT: LazyLock<tokio::runtime::Runtime> =
    LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());

/// 全局配置文件
pub static DANGEROUS_FUNCTION_TOML_PATH: LazyLock<DangerousFunction> = LazyLock::new(|| {
    let mut path = PathBuf::from("./");
    path.push("dangerous_function.toml");
    match DangerousFunction::from_file_info(&path) {
        Ok(dangerous_function) => dangerous_function,
        Err(e) => {
            eprintln!("{}", e);
            DangerousFunction::default()
        }
    }
});
