use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::i18n;

const SYSTEM_PATH: [&str; 2] = [r"C:\Windows\System32", r"C:\Windows\SysWOW64"];
const WINDOWS_PATH: [&str; 2] = [r"C:\Windows", r"C:\Program Files"];

/// 打开指定路径的资源管理器
pub fn open_explorer(path: &str) -> anyhow::Result<()> {
    #[cfg(target_os = "windows")]
    {
        Command::new("explorer").arg(path).spawn()?;
    }
    Ok(())
}

/// 打开文件所在的文件夹
pub fn open_file_location(file_path: &PathBuf) -> anyhow::Result<()> {
    let path_obj = file_path;

    if let Some(parent) = path_obj.parent()
        && let Some(parent_str) = parent.to_str()
    {
        return open_explorer(parent_str);
    }
    Err(anyhow::anyhow!(
        "{}",
        i18n::CANNOT_GET_FILE_DIRECTORY.replace("{}", &file_path.display().to_string())
    ))
}

/// 获取对应连接dll的文件夹目录
pub fn get_dll_folder(exe_path: PathBuf, dll_name: &str) -> anyhow::Result<PathBuf> {
    let exe_path = exe_path.parent().unwrap();
    let dll_path = exe_path.join(dll_name);
    if dll_path.exists() {
        return Ok(dll_path);
    }
    for path in SYSTEM_PATH {
        let path = Path::new(path);
        let dll_path = path.join(dll_name);
        if dll_path.exists() {
            return Ok(dll_path);
        }
    }
    for path in WINDOWS_PATH {
        let path = Path::new(path);
        let dll_path = path.join(dll_name);
        if dll_path.exists() {
            return Ok(dll_path);
        }
    }
    for path in get_system_path() {
        let dll_path = path.join(dll_name);
        if dll_path.exists() {
            return Ok(dll_path);
        }
    }
    Ok(PathBuf::new())
}

fn get_system_path() -> Vec<PathBuf> {
    let path = env::var("PATH").unwrap_or_default();
    path.split(";").map(|p| PathBuf::from(p)).collect()
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_get_system_path() {
        let path = get_system_path();
        println!("{:?}", path);
    }
}
