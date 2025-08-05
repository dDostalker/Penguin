pub(crate) mod calc;
pub(crate) mod file_system;
pub(crate) mod read_file;
pub(crate) mod write_file;
use crate::gui::SubWindowManager;
use crate::tools_api::read_file::nt_header::traits::NtHeaders;
use crate::tools_api::read_file::{
    DataDirectory, ExportDir, ExportTable, ImageDosHeader, ImageDosStub, ImageNtHeaders,
    ImageNtHeaders64, ImageSectionHeaders, ImportDescriptor, ImportDll, is_64, nt_header,
};
use anyhow::anyhow;
use std::cell::{Ref, RefCell, RefMut};
use std::path::PathBuf;
use tokio::fs::File;

#[derive(Debug, PartialEq)]
pub struct HashInfo {
    pub md5: String,
    pub sha1: String,
}



pub struct FileInfo {
    pub file: RefCell<File>,
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
    pub(crate) import_dll: Vec<ImportDll>,
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
    pub files: Vec<FileInfo>,                        // 文件列表
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
    pub fn get_file(&self) -> &FileInfo {
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

    pub fn get_mut_file(&self) -> RefMut<'_, File> {
        self.file.borrow_mut()
    }
    pub fn get_file(&self) -> Ref<'_, File> {
        self.file.borrow()
    }

    pub async fn new(file: PathBuf) -> anyhow::Result<Box<Self>> {
        let mut f = File::options().read(true).write(true).open(&file).await?;
        let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
        let file_path = file;
        let dos_head = Box::new(ImageDosHeader::new(&mut f).await?);
        let is_64_bit = is_64(&mut f, &dos_head).await?;
        let (nt_head, data_directory): (Box<dyn NtHeaders>, DataDirectory) =
            if is_64_bit {
                let (nt, data) = nt_header::read_nt_head::<ImageNtHeaders64>(
                    &mut f,
                    dos_head.get_nt_addr().await,
                )
                .await?;
                (Box::new(nt), data)
            } else {
                let (nt, data) = nt_header::read_nt_head::<ImageNtHeaders>(
                    &mut f,
                    dos_head.get_nt_addr().await,
                )
                .await?;
                (Box::new(nt), data)
            };
        let section_headers = ImageSectionHeaders::new(
            &mut f,
            nt_head.section_start(dos_head.get_nt_addr().await),
            nt_head.section_number(),
        )
        .await?;
        let dos_stub = ImageDosStub::new(&mut f, dos_head.get_nt_addr().await).await?;

        //file size
        let file_size = f.metadata().await?.len();
        // file hash

        Ok(Box::new(FileInfo {
            file: RefCell::new(f),
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
            import_dll: vec![],
            export: ExportTable::default(),

        }))
    }

    pub async fn get_export(&self) -> anyhow::Result<ExportTable> {
        let mut f = self.get_mut_file();
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
        Err(anyhow!("获取导出表失败"))
    }
    pub async fn get_imports(&self) -> anyhow::Result<Vec<ImportDll>> {
        let f = &mut self.get_mut_file();
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
        Ok(import_infos)
    }
}
