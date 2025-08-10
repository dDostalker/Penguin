#![feature(generic_const_exprs)]
pub mod gui;
pub mod tools_api;
pub mod i18n;
use std::sync::LazyLock;

pub static GLOBAL_RT: LazyLock<tokio::runtime::Runtime> =
    LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());


