pub(crate) mod calc;
pub(crate) mod file_system;
pub(crate) mod read_file;
pub(crate) mod write_file;
pub(crate) mod toml;
use crate::gui::SubWindowManager;
use crate::tools_api::read_file::nt_header::traits::NtHeaders;
use crate::tools_api::read_file::{
    DataDirectory, ExportDir, ExportTable, ImageDosHeader, ImageDosStub, ImageFileHeader, ImageNtHeaders,
    ImageNtHeaders64, ImageSectionHeaders, ImportDescriptor, ImportDll, ImportTable, nt_header,
};
use std::cell::{Ref, RefCell, RefMut};
use std::sync::Arc;
use std::path::PathBuf;
use tokio::fs::File;
use crate::GLOBAL_RT;

#[derive(Debug, PartialEq)]
pub struct HashInfo {
    pub md5: String,
    pub sha1: String,
}


pub struct FileInfo {
    pub file: Option<RefCell<File>>,
    pub file_name: String,
    pub file_path: PathBuf,
    pub file_hash: Option<HashInfo>,
    pub dos_head: Box<ImageDosHeader>,
    pub dos_stub: ImageDosStub,
    is_64_bit: bool,
    is_little_endian: bool,
    pub file_size: u64,
    pub(crate) nt_head: Box<dyn NtHeaders>,
    pub(crate) data_directory: DataDirectory,
    pub(crate) section_headers: ImageSectionHeaders,
    pub(crate) import_dll: ImportTable,
    pub(crate) export: ExportTable,
}

/// 窗口数组及其信息
#[derive(Default)]
pub enum Page {
    #[default]
    DosHead,
    DosStub,
    NtHead,
    SectionHead,
    Import,
    Export,
}

#[derive(Default)]
pub struct FileManager {
    pub files: Vec<Box<FileInfo>>,                        // 文件列表
    pub(crate) current_index: usize,                 // 当前文件索引
    pub(crate) page: Page,                           // 目标页面
    pub(crate) hover_index: usize,                   // 左边栏悬停
    pub(crate) sub_window_manager: SubWindowManager, // 子窗口管理器
}
impl FileManager {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            sub_window_manager: SubWindowManager::new(),
            ..Default::default()
        }
    }
    pub fn get_file(&self) -> &Box<FileInfo> {
        self.files.get(self.current_index).unwrap()
    }
}

impl PartialEq<Self> for FileInfo {
    fn eq(&self, other: &Self) -> bool {
        self.file_path == other.file_path && self.file_hash == other.file_hash
    }
}

impl Eq for FileInfo {}

impl FileInfo {

    pub fn get_mut_file(&self) -> anyhow::Result<RefMut<'_, File>> {
        if let Some(file) = &self.file {
            Ok(file.borrow_mut())
        } else {
            Err(anyhow::anyhow!("文件句柄已关闭"))
        }
    }
    pub fn get_file(&self) -> anyhow::Result<Ref<'_, File>> {
        if let Some(file) = &self.file {
            Ok(file.borrow())
        } else {
            Err(anyhow::anyhow!("文件句柄已关闭"))
        }
    }
    /// 转换状态，为后续dll调试提供准备
    pub fn lock_file(&mut self)-> anyhow::Result<()> {
        if self.file.is_some() {
            self.file = None;
            Ok(())
        }
        else{
            let file = GLOBAL_RT.block_on(File::options()
                .read(true)
                .write(true)
                .open(&self.file_path))?;
            self.file = Some(RefCell::new(file));
            Ok(())
        }
    }

    pub async fn new(file_path: PathBuf) -> anyhow::Result<Box<Self>> {
        // 1. 打开文件并提取基本信息
        let mut file = File::options()
            .read(true)
            .write(true)
            .open(&file_path)
            .await?;
        
        let file_name = Self::extract_file_name(&file_path)?;
        let file_size = file.metadata().await?.len();

        // 2. 解析DOS头
        let dos_head = Box::new(ImageDosHeader::new(&mut file).await?);
        let nt_addr = dos_head.get_nt_addr().await;

        // 3. 判断架构并解析NT头
        let is_64_bit = is_64(&mut file, &dos_head).await?;
        let (nt_head, data_directory) = Self::parse_nt_headers(&mut file, nt_addr, is_64_bit).await?;

        // 4. 解析其他结构
        let section_headers = ImageSectionHeaders::new(
            &mut file,
            nt_head.section_start(nt_addr),
            nt_head.section_number(),
        ).await?;
        
        let dos_stub = ImageDosStub::new(&mut file, nt_addr).await?;

        // 5. 构建FileInfo结构
        Ok(Box::new(FileInfo {
            file: Some(RefCell::new(file)),
            file_name,
            file_path,
            file_hash: None,
            dos_head,
            dos_stub,
            is_64_bit,
            is_little_endian: false,
            file_size,
            nt_head,
            data_directory,
            section_headers,
            import_dll: ImportTable::default(),
            export: ExportTable::default(),
        }))
    }

    /// 提取文件名
    fn extract_file_name(file_path: &PathBuf) -> anyhow::Result<String> {
        file_path
            .file_name()
            .and_then(|name| name.to_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow::anyhow!("无法提取文件名"))
    }

    /// 解析NT头部信息
    async fn parse_nt_headers(
        file: &mut File,
        nt_addr: u16,
        is_64_bit: bool,
    ) -> anyhow::Result<(Box<dyn NtHeaders>, DataDirectory)> {
        if is_64_bit {
            let (nt_header, data_dir) = nt_header::read_nt_head::<ImageNtHeaders64>(file, nt_addr).await?;
            Ok((Box::new(nt_header), data_dir))
        } else {
            let (nt_header, data_dir) = nt_header::read_nt_head::<ImageNtHeaders>(file, nt_addr).await?;
            Ok((Box::new(nt_header), data_dir))
        }
    }

    pub async fn get_export(&self) -> anyhow::Result<ExportTable> {
        let mut f = self.get_mut_file()?;
        if let Some(export_dir) = ExportDir::new(
            &mut f,
            &*self.nt_head,
            &self.section_headers,
            &self.data_directory,
        )
        .await?
        {
            let export_info = ExportTable::new(&mut f, &*self.nt_head, &self.section_headers, &export_dir)
                .await?;
            return Ok(export_info);
        }
        Ok(ExportTable::default())
    }

    /// 获取导入表
    pub async fn get_imports(&self) -> anyhow::Result<ImportTable> {
        let f = &mut self.get_mut_file()?;
        let mut import_infos = Vec::new();
        let mut index = 0;
        loop {
            let import = ImportDescriptor::new(
                f,
                &*self.nt_head,
                &self.section_headers,
                &self.data_directory,
                index,
            )
            .await?;
            if let Some(import) = import {
                let import_info = ImportDll::new(
                    f,
                    &self.dos_head,
                    import,
                    &*self.nt_head,
                    &self.section_headers,
                )
                .await?;
                import_infos.push(import_info);
            } else {
                break;
            }
            index += 1;
        }
        Ok(ImportTable(Arc::new(RefCell::new(import_infos))))
    }
}

pub(crate) fn load_file_info(path: PathBuf) -> anyhow::Result<Box<FileInfo>> {
    GLOBAL_RT.block_on(FileInfo::new(path))
}


pub async fn is_64(file: &mut File, image_dos_header: &ImageDosHeader) -> anyhow::Result<bool> {
    let image_file_header = ImageFileHeader::new(file, image_dos_header).await?;
    if nt_header::MACHINE_32.contains(&image_file_header.machine) {
        return Ok(false);
    } else if nt_header::MACHINE_64.contains(&image_file_header.machine) {
        return Ok(true);
    }
    Err(anyhow::anyhow!("Not a normal machine image file"))
}
