use crate::i18n;
use crate::tools_api::read_file::ImageDosHeader;
use std::fs::File;
use std::io::Read;
use std::mem::{size_of, MaybeUninit};
use std::ptr;

impl ImageDosHeader {
    /// 获取nt头文件地址
    pub fn get_nt_addr(&self) -> u16 {
        self.e_lfanew
    }
    /// 读取dos头
    pub(crate) fn new(file: &mut File) -> anyhow::Result<ImageDosHeader> {
        unsafe {
            let mut dos_head = MaybeUninit::<ImageDosHeader>::uninit();
            let bytes = std::slice::from_raw_parts_mut(
                dos_head.as_mut_ptr() as *mut u8,
                size_of::<ImageDosHeader>()
            );
            file.read_exact(bytes)?;
            let dos_head = dos_head.assume_init();
            
            // 验证dos头
            if dos_head.e_magic != 0x5A4D {
                return Err(anyhow::anyhow!("{}", i18n::NOT_VALID_PE_FILE));
            }
            Ok(dos_head)
        }
    }
}
