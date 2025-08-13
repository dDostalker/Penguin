// 包含构建时生成的常量
include!(concat!(env!("OUT_DIR"), "/language_constants.rs"));

/// 宏，用于获取当前语言的文本
/// 使用方式: i18n!(APP_TITLE)
#[macro_export]
macro_rules! i18n {
    ($constant:ident) => {
        $crate::i18n::$constant
    };
}

/// 获取当前语言
pub fn get_current_language() -> &'static str {
    CURRENT_LANGUAGE
}

/// 检查当前语言是否为指定语言
pub fn is_language(lang: &str) -> bool {
    CURRENT_LANGUAGE == lang
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_detection() {
        let current = get_current_language();
        assert!(["chinese", "english"].contains(&current));
    }

    #[test]
    fn test_constants_exist() {
        // 验证关键常量存在
        assert!(!APP_TITLE.is_empty());
        assert!(!ABOUT_TITLE.is_empty());
        assert!(!SETTINGS_TITLE.is_empty());
    }
}
