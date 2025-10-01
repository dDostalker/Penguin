use crate::i18n;
use crate::tools_api::read_file::ImageDosHeader;
use std::fs::File;
use std::io::Read;
use std::mem::transmute;

impl ImageDosHeader {
    /// 获取nt头文件地址
    pub fn get_nt_addr(&self) -> u16 {
        self.e_lfanew
    }
    /// 读取dos头
    pub(crate) fn new(file: &mut File) -> anyhow::Result<ImageDosHeader> {
        let mut dos_head: ImageDosHeader = Default::default();
        unsafe {
            let reads: &mut [u8; 64] = transmute(&mut dos_head);
            file.read(reads)?;
        }
        // 验证dos头
        if dos_head.e_magic != 0x5A4D {
            return Err(anyhow::anyhow!("{}", i18n::NOT_VALID_PE_FILE));
        }
        Ok(dos_head)
    }
}
