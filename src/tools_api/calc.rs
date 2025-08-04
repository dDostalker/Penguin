use std::path::PathBuf;
use md5::{Digest, Md5};
use sha1::Sha1;
use file_hashing::{get_hash_file};

/// 计算文件md-5
pub fn calc_md5(file_path: &PathBuf) -> String {
    let mut hasher = Md5::new();
    match get_hash_file(file_path, &mut hasher) {
        Ok(hash) => hash,
        Err(_e) => 
            "计算md5失败".to_string()
    }
}
pub fn calc_sha1(file_path: &PathBuf) -> String {
    let mut hasher = Sha1::new();
    match get_hash_file(file_path, &mut hasher) {
        Ok(hash) => hash,
        Err(_e) => "计算sha1失败".to_string()
    }
}   