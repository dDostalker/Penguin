use crate::tools_api::read_file::{ExportInfo, ImportFunction};
use std::io::SeekFrom;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt};
use std::path::PathBuf;

impl ExportInfo {
    pub async fn write_func_name(&self, file: &mut File, func_name: &str) -> anyhow::Result<()> {
        if self.name_max_length < func_name.len() as u32 {
            return Err(anyhow::anyhow!("func_name too long"));
        }
        if !func_name.is_ascii() {
            return Err(anyhow::anyhow!("func_name not ascii"));
        }
        let first_char = func_name.chars().next().unwrap();
        if first_char.is_ascii_digit() {
            return Err(anyhow::anyhow!("func_name first can't be digit"));
        }
        // 添加头过滤/

        file.seek(SeekFrom::Start(self.name_string_fo as u64))
            .await?;
        file.write(func_name.as_bytes()).await?;
        file.write(&vec![0; self.name_max_length as usize - func_name.len()])
            .await?;
        Ok(())
    }
    pub async fn write_func_address(&self, file: &mut File, func: u32) -> anyhow::Result<()> {
        file.seek(SeekFrom::Start(self.function_address as u64))
            .await?;
        file.write_u32_le(func).await?;
        Ok(())
    }
}

impl ImportFunction {
    pub async fn write_func_name(&self, file: &mut File, func_name: &str) -> anyhow::Result<()> {
        file.seek(SeekFrom::Start(self.name_address as u64)).await?;
        if self.name_max_length < func_name.len() as u32 {
            return Err(anyhow::anyhow!("func_name too long"));
        }
        if !func_name.is_ascii() {
            return Err(anyhow::anyhow!("func_name not ascii"));
        }
        file.write(func_name.as_bytes()).await?;
        file.write(&vec![0; self.name_max_length as usize - func_name.len()])
            .await?;
        Ok(())
    }
}

pub async fn copy_file(file: &mut File, file_path: &PathBuf) -> anyhow::Result<()> {
    let mut file_bak = File::create(file_path).await?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).await?;
    file_bak.write_all(&buf).await?;
    Ok(())
}