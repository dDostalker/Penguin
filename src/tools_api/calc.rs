use file_hashing::get_hash_file;
use md5::{Digest, Md5};
use sha1::Sha1;
use std::path::PathBuf;
use crate::i18n;

/// 计算文件md-5
pub fn calc_md5(file_path: &PathBuf) -> String {
    let mut hasher = Md5::new();
    match get_hash_file(file_path, &mut hasher) {
        Ok(hash) => hash,
        Err(_e) => i18n::CALC_MD5_FAILED.to_string(),
    }
}
pub fn calc_sha1(file_path: &PathBuf) -> String {
    let mut hasher = Sha1::new();
    match get_hash_file(file_path, &mut hasher) {
        Ok(hash) => hash,
        Err(_e) => i18n::CALC_SHA1_FAILED.to_string(),
    }
}

/// 解析16进制或10进制字符串为usize
pub fn parse_address_string(input: &str) -> Result<usize, String> {
    let input = input.trim();
    if input.is_empty() {
        return Ok(0);
    }
    
    // 检查是否为16进制格式 (0x开头或包含字母)
    if input.starts_with("0x") || input.starts_with("0X") {
        usize::from_str_radix(&input[2..], 16)
            .map_err(|e| format!("{}", i18n::HEX_PARSE_ERROR.replace("{}", &e.to_string())))
    } else if input.chars().any(|c| c.is_ascii_alphabetic()) {
        // 包含字母但没有0x前缀，尝试作为16进制解析
        usize::from_str_radix(input, 16)
            .map_err(|e| format!("{}", i18n::HEX_PARSE_ERROR.replace("{}", &e.to_string())))
    } else {
        // 纯数字，作为10进制解析
        input.parse::<usize>()
            .map_err(|e| format!("{}", i18n::DECIMAL_PARSE_ERROR.replace("{}", &e.to_string())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_decimal() {
        assert_eq!(parse_address_string("1234").unwrap(), 1234);
        assert_eq!(parse_address_string("0").unwrap(), 0);
    }

    #[test]
    fn test_parse_hex_with_prefix() {
        assert_eq!(parse_address_string("0x4D2").unwrap(), 1234);
        assert_eq!(parse_address_string("0X4D2").unwrap(), 1234);
    }

    #[test]
    fn test_parse_hex_without_prefix() {
        assert_eq!(parse_address_string("4D2").unwrap(), 1234);
        assert_eq!(parse_address_string("ABCD").unwrap(), 0xABCD);
    }

    #[test]
    fn test_parse_empty() {
        assert_eq!(parse_address_string("").unwrap(), 0);
        assert_eq!(parse_address_string("   ").unwrap(), 0);
    }

    #[test]
    fn test_parse_invalid() {
        assert!(parse_address_string("invalid").is_err());
        assert!(parse_address_string("0xinvalid").is_err());
    }
}
