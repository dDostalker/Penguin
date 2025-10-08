use crate::i18n;
use crate::tools_api::read_file::DataDirectory;
use crate::tools_api::read_file::ImageResourceDataEntry;
use crate::tools_api::read_file::ImageSectionHeaders;
use crate::tools_api::read_file::ResourceTree;
use crate::tools_api::read_file::nt_header::traits::NtHeaders;
use crate::tools_api::read_file::rva_2_fo;
use crate::tools_api::read_file::{ImageResourceDirectory, ImageResourceDirectoryEntry};
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem::size_of;
use std::mem::transmute;
use std::path::{Path, PathBuf};

impl ImageResourceDirectory {
    pub fn new(file: &mut File, address: u32) -> anyhow::Result<Self> {
        let mut resource_directory: ImageResourceDirectory = Default::default();

        file.seek(SeekFrom::Start(address as u64))?;
        unsafe {
            let read: &mut [u8; size_of::<ImageResourceDirectory>()] =
                transmute(&mut resource_directory);
            file.read(read)?;
        }
        Ok(resource_directory)
    }
}

impl ImageResourceDirectoryEntry {
    pub fn new(file: &mut File, address: u32) -> anyhow::Result<Self> {
        let mut resource_directory_entry: ImageResourceDirectoryEntry = Default::default();
        file.seek(SeekFrom::Start(address as u64))?;
        unsafe {
            let read: &mut [u8; size_of::<ImageResourceDirectoryEntry>()] =
                transmute(&mut resource_directory_entry);
            file.read(read)?;
        }
        Ok(resource_directory_entry)
    }
}

impl ImageResourceDataEntry {
    pub fn new<T>(
        file: &mut File,
        nt_head: &T,
        image_section_headers: &ImageSectionHeaders,
        address: u32,
    ) -> anyhow::Result<Self>
    where
        T: NtHeaders + ?Sized,
    {
        let mut resource_data_entry: ImageResourceDataEntry = Default::default();
        if let Some(fo) = rva_2_fo(nt_head, image_section_headers, address) {
            file.seek(SeekFrom::Start(fo as u64))?;
            unsafe {
                let read: &mut [u8; size_of::<ImageResourceDataEntry>()] =
                    transmute(&mut resource_data_entry);
                file.read(read)?;
            }
        }
        Ok(resource_data_entry)
    }
}

impl ResourceTree {
    pub fn new(name: String, is_dic: bool, data_address: u32, size: u32) -> Self {
        if is_dic {
            Self {
                name,
                children: Some(vec![]),
                data_address,
                size: 0,
            }
        } else {
            Self {
                name,
                children: None,
                data_address,
                size,
            }
        }
    }
    pub fn add_child(&mut self, child: Self) {
        self.children.as_mut().unwrap().push(child);
    }
    pub fn get_resource_tree<E>(
        file: &mut File,
        address: u32,
        nt_head: &E,
        image_section_headers: &ImageSectionHeaders,
        data_dir: &DataDirectory,
    ) -> anyhow::Result<Self>
    where
        E: NtHeaders + ?Sized,
    {
        let address = rva_2_fo(nt_head, image_section_headers, address)
            .ok_or(anyhow::anyhow!(i18n::ERROR_GET_RVA_OFFSET))?;
        let resource_directory = ImageResourceDirectory::new(file, address)?;
        let mut resource_root = Self::new("Directory".to_string(), true, address, 0);
        let resource_num =
            resource_directory.number_of_named_entries + resource_directory.number_of_id_entries;
        let address = address + size_of::<ImageResourceDirectory>() as u32;
        for i in 0..resource_num as u32 {
            let address = address + i * 8;
            let resource_directory_entry = ImageResourceDirectoryEntry::new(file, address)?;
            if resource_directory_entry.name_offset == 0 {
                continue;
            }
            let mut resource_tree = Self::new(
                resource_directory_entry.name_offset.to_string(),
                true,
                address,
                0,
            );
            if resource_directory_entry.offset_to_data & 0x80000000 == 0 {
                let offset = resource_directory_entry.offset_to_data & 0x7FFFFFFF;
                let resource_data_address = offset + data_dir.get_resource_directory_address()?;
                let resource_data_entry = ImageResourceDataEntry::new(
                    file,
                    nt_head,
                    image_section_headers,
                    resource_data_address,
                )?;
                println!("resource_data_entry: {:?}", resource_data_entry);
                resource_tree.add_child(Self::new(
                    resource_directory_entry.name_offset.to_string(),
                    false,
                    resource_data_entry.data_offset,
                    resource_data_entry.data_size,
                ));
            } else {
                let offset = resource_directory_entry.offset_to_data & 0x7FFFFFFF;
                let resource_data_address = offset + data_dir.get_resource_directory_address()?;
                let resource = Self::get_resource_tree(
                    file,
                    resource_data_address,
                    nt_head,
                    image_section_headers,
                    data_dir,
                )?;
                resource_tree.add_child(resource);
            }
            resource_root.add_child(resource_tree);
        }
        Ok(resource_root)
    }

    /// 提取资源到指定目录
    pub fn extract_resources<T>(
        &self,
        file: &mut File,
        output_dir: &Path,
        nt_head: &T,
        image_section_headers: &ImageSectionHeaders,
        data_dir: &DataDirectory,
    ) -> anyhow::Result<Vec<PathBuf>>
    where
        T: NtHeaders + ?Sized,
    {
        let mut extracted_files = Vec::new();

        // 确保输出目录存在
        fs::create_dir_all(output_dir)?;

        self._extract_resources_recursive(
            file,
            output_dir,
            nt_head,
            image_section_headers,
            data_dir,
            &mut extracted_files,
            "",
        )?;

        Ok(extracted_files)
    }

    /// 递归提取资源的内部方法
    fn _extract_resources_recursive<T>(
        &self,
        file: &mut File,
        output_dir: &Path,
        nt_head: &T,
        image_section_headers: &ImageSectionHeaders,
        data_dir: &DataDirectory,
        extracted_files: &mut Vec<PathBuf>,
        current_path: &str,
    ) -> anyhow::Result<()>
    where
        T: NtHeaders + ?Sized,
    {
        let new_path = if current_path.is_empty() {
            self.name.clone()
        } else {
            format!("{}/{}", current_path, self.name)
        };

        if let Some(children) = &self.children {
            // 这是一个目录节点
            let dir_path = output_dir.join(&new_path);
            fs::create_dir_all(&dir_path)?;

            for child in children {
                child._extract_resources_recursive(
                    file,
                    output_dir,
                    nt_head,
                    image_section_headers,
                    data_dir,
                    extracted_files,
                    &new_path,
                )?;
            }
        } else {
            // 这是一个文件节点
            if let Some(file_offset) = rva_2_fo(nt_head, image_section_headers, self.data_address) {
                let file_path = output_dir.join(&new_path);

                // 确保父目录存在
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                // 读取资源数据
                let mut buffer = vec![0u8; self.size as usize];
                file.seek(SeekFrom::Start(file_offset as u64))?;
                file.read_exact(&mut buffer)?;

                // 写入文件
                fs::write(&file_path, buffer)?;
                extracted_files.push(file_path);
            }
        }

        Ok(())
    }
}

/// 资源信息
#[derive(Debug, Clone)]
pub struct ResourceInfo {
    pub path: String,
    pub size: u32,
    pub data_address: u32,
    pub resource_type: String,
}
