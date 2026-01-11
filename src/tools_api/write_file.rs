use crate::tools_api::read_file::{ExportInfo, ImageSectionHeader, ImportFunction};
use std::fs::File;
use std::io::SeekFrom;
use std::io::{Read, Seek, Write};
use std::path::PathBuf;

impl ExportInfo {
    pub fn write_func_name(&self, file: &mut File, func_name: &str) -> anyhow::Result<()> {
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

        file.seek(SeekFrom::Start(self.name_string_fo as u64))?;
        file.write_all(func_name.as_bytes())?;
        file.write_all(&vec![0; self.name_max_length as usize - func_name.len()])?;
        Ok(())
    }
    pub fn write_func_address(&self, file: &mut File, func: u32) -> anyhow::Result<()> {
        file.seek(SeekFrom::Start(self.function_address as u64))?;
        file.write_all(&func.to_le_bytes())?;
        Ok(())
    }
}

impl ImportFunction {
    pub fn write_func_name(&self, file: &mut File, func_name: &str) -> anyhow::Result<()> {
        file.seek(SeekFrom::Start(self.name_address as u64))?;
        if self.name_max_length < func_name.len() as u32 {
            return Err(anyhow::anyhow!("func_name too long"));
        }
        if !func_name.is_ascii() {
            return Err(anyhow::anyhow!("func_name not ascii"));
        }
        file.write_all(func_name.as_bytes())?;
        file.write_all(&vec![0; self.name_max_length as usize - func_name.len()])?;
        Ok(())
    }
}

impl ImageSectionHeader {}

pub fn copy_file(file: &mut File, file_path: &PathBuf) -> anyhow::Result<()> {
    let mut file_bak = File::create(file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;
    file_bak.write_all(&buf)?;
    Ok(())
}
