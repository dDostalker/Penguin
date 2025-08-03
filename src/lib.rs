#![feature(generic_const_exprs)]
pub mod gui;
pub mod tools_api;
use std::{cell::RefCell, sync::LazyLock};
use std::sync::Mutex;

pub static GLOBAL_RT: LazyLock<tokio::runtime::Runtime> =
    LazyLock::new(|| tokio::runtime::Runtime::new().unwrap());
