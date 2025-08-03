pub(crate) mod read_file;
pub(crate) mod write_file;
pub mod dll_debug;

pub mod structure {
    use crate::tools_api::read_file::nt_header::traits::NtHeaders;
    use crate::tools_api::read_file::{
        DataDirectory, ExportDir, ExportTable, ImageDosHeader, ImageDosStub,
        ImageNtHeaders, ImageNtHeaders64, ImageSectionHeaders, ImportDescriptor, ImportDll, is_64,
        nt_header,
    };
    use anyhow::anyhow;
    use std::cell::{RefCell, RefMut};
    use std::path::PathBuf;
    use tokio::fs::File;

    pub struct FileInfo {
        pub file: RefCell<File>,
        pub file_name: String,
        pub file_path: String,
        pub file_hash: String,
        pub dos_head: Box<ImageDosHeader>,
        pub dos_stub: Box<ImageDosStub>,
        is_64_bit: bool,
        is_little_endian: bool,
        pub file_size: u64,
        pub(crate) nt_head: Box<dyn NtHeaders>,
        pub(crate) data_directory: Box<DataDirectory>,
        pub(crate) section_headers: Box<ImageSectionHeaders>,
        pub(crate) import_dll: Vec<ImportDll>,
        pub(crate) export: Box<ExportTable>,
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
        pub async fn new(file: PathBuf) -> anyhow::Result<Box<Self>> {
            let mut f = File::options().read(true).write(true).open(&file).await?;
            let file_name = file.file_name().unwrap().to_str().unwrap().to_string();
            let file_path = file.to_str().unwrap().to_string();
            let file_dos_head = Box::new(ImageDosHeader::new(&mut f).await?);
            let file_is_64 = is_64(&mut f, &file_dos_head).await?;
            let (file_nt_head, file_data_directory): (Box<dyn NtHeaders>, Box<DataDirectory>) =
                if file_is_64 {
                    let (nt, data) = nt_header::read_nt_head::<ImageNtHeaders64>(
                        &mut f,
                        file_dos_head.get_nt_addr().await,
                    )
                    .await?;
                    (Box::new(nt), data)
                } else {
                    let (nt, data) = nt_header::read_nt_head::<ImageNtHeaders>(
                        &mut f,
                        file_dos_head.get_nt_addr().await,
                    )
                    .await?;
                    (Box::new(nt), data)
                };
            let image_section_headers = ImageSectionHeaders::new(
                &mut f,
                file_nt_head.section_start(file_dos_head.get_nt_addr().await),
                file_nt_head.section_number(),
            )
            .await?;
            let file_dos_stub =
                Box::new(ImageDosStub::new(&mut f, file_dos_head.get_nt_addr().await).await?);

            //file size
            let file_size = f.metadata().await?.len();
            
            Ok(Box::new(FileInfo {
                file: RefCell::new(f),
                file_name,
                file_path,
                file_hash: "".to_string(),
                dos_head: file_dos_head,
                dos_stub: file_dos_stub,
                is_64_bit: file_is_64,
                is_little_endian: false,
                file_size,
                nt_head: file_nt_head,
                data_directory: file_data_directory,
                section_headers: image_section_headers,
                import_dll: vec![],
                export: Box::new(ExportTable::default()),
            }))
        }
        pub async fn get_export(&self) -> anyhow::Result<Box<ExportTable>> {
            let mut f = self.get_mut_file();
            if let Some(export_dir) = ExportDir::new(
                &mut f,
                &*self.nt_head,
                &self.section_headers,
                &self.data_directory,
            )
            .await?
            {
                let export_info = Box::new(
                    ExportTable::new(&mut f, &*self.nt_head, &self.section_headers, &export_dir)
                        .await?,
                );
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
}

/// 文件系统操作模块
pub mod file_system {
    use std::process::Command;
    use std::path::Path;
    
    /// 打开指定路径的资源管理器
    pub fn open_explorer(path: &str) -> anyhow::Result<()> {
        #[cfg(target_os = "windows")]
        {
            Command::new("explorer")
                .arg(path)
                .spawn()?;
        }
        
        #[cfg(target_os = "macos")]
        {
            Command::new("open")
                .arg(path)
                .spawn()?;
        }
        
        #[cfg(target_os = "linux")]
        {
            let file_managers = ["xdg-open", "nautilus", "dolphin", "thunar"];
            
            for manager in &file_managers {
                if Command::new("which").arg(manager).output().is_ok() {
                    Command::new(manager).arg(path).spawn()?;
                    return Ok(());
                }
            }
            
            return Err(anyhow::anyhow!("未找到可用的文件管理器"));
        }
        
        Ok(())
    }
    
    /// 打开文件所在的文件夹
    pub fn open_file_location(file_path: &str) -> anyhow::Result<()> {
        let path_obj = Path::new(file_path);
        
        if let Some(parent) = path_obj.parent() && let Some(parent_str) = parent.to_str(){
                return open_explorer(parent_str);
        }
        
        Err(anyhow::anyhow!("无法获取文件所在目录"))
    }
    
    /// 打开当前工作目录
    pub fn open_current_directory() -> anyhow::Result<()> {
        let current_dir = std::env::current_dir()?;
        let path_str = current_dir.to_string_lossy();
        open_explorer(&path_str)
    }
}
