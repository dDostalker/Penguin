use crate::tools_api::read_file::ImageDosStub;
use std::fs::File;
use std::io::SeekFrom;
use std::io::{Read, Seek};

impl ImageDosStub {
    pub fn new(file: &mut File, dos_stub_end: u16) -> anyhow::Result<ImageDosStub> {
        file.seek(SeekFrom::Start(64))?;
        // dos_stub_end - 64 为存根长度
        let mut buffer = vec![0u8; dos_stub_end as usize - 64];
        file.read(&mut buffer)?;
        Ok(ImageDosStub { buffer })
    }
}
