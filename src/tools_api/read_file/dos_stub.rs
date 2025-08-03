use crate::tools_api::read_file::ImageDosStub;
use std::io::SeekFrom;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

impl ImageDosStub {
    pub async fn new(file: &mut File, dos_stub_end: u16) -> anyhow::Result<ImageDosStub> {
        file.seek(SeekFrom::Start(64)).await?;
        // dos_stub_end - 64 为存根长度
        let mut buffer = vec![0u8; dos_stub_end as usize - 64];
        file.read(&mut buffer).await?;
        Ok(ImageDosStub { buffer })
    }
}
