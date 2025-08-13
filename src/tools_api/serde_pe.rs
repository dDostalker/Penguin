use crate::GLOBAL_RT;
use crate::i18n;
use crate::tools_api::read_file::{
    SerializableDataDirectory, SerializableExportTable, SerializableImageSectionHeaders,
    SerializableImportTable, SerializableNtHeaders,
};
use crate::tools_api::{FileInfo, HashInfo};
use anyhow::anyhow;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use tokio::fs::write;
// 为FileInfo创建可序列化的结构体
#[derive(Serialize, Deserialize)]
pub struct SerializableFileInfo {
    pub file_name: String,
    pub file_path: PathBuf,
    pub file_hash: Option<HashInfo>,
    pub is_64_bit: bool,
    pub is_little_endian: bool,
    pub file_size: u64,
    pub dos_head: crate::tools_api::read_file::ImageDosHeader,
    pub dos_stub: crate::tools_api::read_file::ImageDosStub,
    pub nt_headers: SerializableNtHeaders,
    pub data_directory: SerializableDataDirectory,
    pub section_headers: SerializableImageSectionHeaders,
    pub import_dll: SerializableImportTable,
    pub export: SerializableExportTable,
}

#[derive(Serialize, Deserialize)]
pub struct DangerousFunction {
    pub dangerous: Vec<String>,
    pub warning: Vec<String>,
}
impl Default for DangerousFunction {
    fn default() -> Self {
        Self {
            dangerous: vec![
                "VirtualAlloc".to_string(),
                "VirtualFree".to_string(),
                "VirtualProtect".to_string(),
                "VirtualQuery".to_string(),
                "VirtualProtectEx".to_string(),
                "VirtualQueryEx".to_string(),
                //todo!添加危险函数
            ],
            warning: vec![
                "CreateProcessA".to_string(),
                "CreateProcessW".to_string(),
                "CreateProcess".to_string(),
                "CreateProcessAsUserA".to_string(),
                "CreateProcessAsUserW".to_string(),
                "CreateProcessWithTokenW".to_string(),
                "CreateProcessWithLogonW".to_string(),
                "CreateProcessWithTokenW".to_string(),
            ],
        }
    }
}

impl SerializableFileInfo {
    pub fn from_file_info(file_info: &mut FileInfo) -> anyhow::Result<Self> {
        let import_dll_geted = file_info.import_dll.0.borrow().is_empty();
        if import_dll_geted {
            file_info.import_dll = GLOBAL_RT.block_on(file_info.get_imports())?;
        }
        let export_geted = file_info.export.0.borrow().is_empty();
        if export_geted {
            file_info.export = GLOBAL_RT.block_on(file_info.get_export())?;
        }
        Ok(Self {
            file_name: file_info.file_name.clone(),
            file_path: file_info.file_path.clone(),
            file_hash: file_info.file_hash.clone(),
            is_64_bit: file_info.is_64_bit,
            is_little_endian: false, // 暂时使用默认值，因为字段是私有的
            dos_head: *file_info.dos_head.clone(),
            dos_stub: file_info.dos_stub.clone(),
            nt_headers: file_info.nt_head.serde_serialize(),
            file_size: file_info.file_size,
            data_directory: file_info.data_directory.to_serializable(),
            section_headers: file_info.section_headers.to_serializable(),
            import_dll: file_info.import_dll.to_serializable(),
            export: file_info.export.to_serializable(),
        })
    }
}

impl DangerousFunction {
    pub fn from_file_info(file_path: &PathBuf) -> anyhow::Result<Self> {
        let file = File::open(file_path)?;
        let mut reader_str = String::new();
        BufReader::new(file).read_to_string(&mut reader_str)?;
        let dangerous_function: DangerousFunction = toml::from_str(&reader_str)?;
        Ok(dangerous_function)
    }
}

pub fn pe_info_to_toml(file_info: &mut FileInfo) -> anyhow::Result<String> {
    let serializable_info = SerializableFileInfo::from_file_info(file_info)?;
    let toml_string = toml::to_string_pretty(&serializable_info).map_err(|e| {
        anyhow!(
            "{}",
            i18n::SERIALIZE_TOML_FAILED.replace("{}", &e.to_string())
        )
    })?;
    Ok(toml_string)
}
pub fn pe_info_to_json(file_info: &mut FileInfo) -> anyhow::Result<String> {
    let serializable_info = SerializableFileInfo::from_file_info(file_info)?;
    let json_string = serde_json::to_string_pretty(&serializable_info).map_err(|e| {
        anyhow!(
            "{}",
            i18n::SERIALIZE_JSON_FAILED.replace("{}", &e.to_string())
        )
    })?;
    Ok(json_string)
}

pub async fn save_to_file(
    file_info: &mut FileInfo,
    file_path: &PathBuf,
    file_type: &str,
) -> anyhow::Result<()> {
    let serde_string = match file_type {
        "toml" => pe_info_to_toml(file_info),
        "json" => pe_info_to_json(file_info),
        _ => {
            return Err(anyhow!(
                "{}",
                i18n::UNSUPPORTED_FILE_TYPE.replace("{}", file_type)
            ));
        }
    };
    let serde_string = match serde_string {
        Ok(string) => string,
        Err(e) => {
            return Err(anyhow!(
                "{}",
                i18n::SERIALIZE_FAILED.replace("{}", &e.to_string())
            ));
        }
    };
    write(file_path, serde_string)
        .await
        .map_err(|e| anyhow!("{}", i18n::SAVE_FAILED_ERROR.replace("{}", &e.to_string())))?;
    Ok(())
}
