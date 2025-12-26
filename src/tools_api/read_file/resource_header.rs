use crate::i18n;
use crate::tools_api::read_file::DataDirectory;
use crate::tools_api::read_file::ImageResourceDataEntry;
use crate::tools_api::read_file::ImageSectionHeaders;
use crate::tools_api::read_file::ResourceTree;
use crate::tools_api::read_file::nt_header::traits::NtHeaders;
use crate::tools_api::read_file::rva_2_fo;
use crate::tools_api::read_file::{ImageResourceDirectory, ImageResourceDirectoryEntry};
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::mem::{MaybeUninit, size_of};
use std::path::{Path, PathBuf};

/// ICO/CUR 文件头（6字节）
#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct IconFileHeader {
    reserved: u16,    // 保留，必须为0
    icon_type: u16,   // 1=ICO, 2=CUR
    image_count: u16, // 图像数量
}

/// ICO 目录条目（16字节）
#[allow(dead_code)]
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct IconDirEntry {
    width: u8,         // 宽度（0表示256）
    height: u8,        // 高度（0表示256）
    color_count: u8,   // 颜色数（0表示256色以上）
    reserved: u8,      // 保留，必须为0
    planes: u16,       // 颜色平面数（ICO）或热点X（CUR）
    bit_count: u16,    // 每像素位数（ICO）或热点Y（CUR）
    bytes_in_res: u32, // 图像数据大小
    image_offset: u32, // 图像数据偏移（在ICO文件中）
}

/// RT_GROUP_ICON/RT_GROUP_CURSOR 资源中的条目
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct GroupIconDirEntry {
    width: u8,
    height: u8,
    color_count: u8,
    reserved: u8,
    planes: u16,
    bit_count: u16,
    bytes_in_res: u32,
    icon_id: u16, // 引用的RT_ICON资源的ID
}

impl ImageResourceDirectory {
    pub fn new(file: &mut File, address: u32) -> anyhow::Result<Self> {
        file.seek(SeekFrom::Start(address as u64))?;
        unsafe {
            let mut resource_directory = MaybeUninit::<ImageResourceDirectory>::uninit();
            let bytes = std::slice::from_raw_parts_mut(
                resource_directory.as_mut_ptr() as *mut u8,
                size_of::<ImageResourceDirectory>(),
            );
            file.read_exact(bytes)?;
            Ok(resource_directory.assume_init())
        }
    }
}

impl ImageResourceDirectoryEntry {
    pub fn new(file: &mut File, address: u32) -> anyhow::Result<Self> {
        file.seek(SeekFrom::Start(address as u64))?;
        unsafe {
            let mut resource_directory_entry = MaybeUninit::<ImageResourceDirectoryEntry>::uninit();
            let bytes = std::slice::from_raw_parts_mut(
                resource_directory_entry.as_mut_ptr() as *mut u8,
                size_of::<ImageResourceDirectoryEntry>(),
            );
            file.read_exact(bytes)?;
            Ok(resource_directory_entry.assume_init())
        }
    }
}

impl ImageResourceDataEntry {
    #[allow(dead_code)]
    pub fn new<T>(
        file: &mut File,
        nt_head: &T,
        image_section_headers: &ImageSectionHeaders,
        address: u32,
    ) -> anyhow::Result<Self>
    where
        T: NtHeaders + ?Sized,
    {
        if let Some(fo) = rva_2_fo(nt_head, image_section_headers, address) {
            file.seek(SeekFrom::Start(fo as u64))?;
            unsafe {
                let mut resource_data_entry = MaybeUninit::<ImageResourceDataEntry>::uninit();
                let bytes = std::slice::from_raw_parts_mut(
                    resource_data_entry.as_mut_ptr() as *mut u8,
                    size_of::<ImageResourceDataEntry>(),
                );
                file.read_exact(bytes)?;
                return Ok(resource_data_entry.assume_init());
            }
        }
        Ok(Default::default())
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
        if let Some(ref mut children) = self.children {
            children.push(child);
        }
    }

    fn get_default_extension(resource_type: &str) -> &'static str {
        match resource_type {
            "RT_ICON" | "RT_GROUP_ICON" => ".ico",
            "RT_BITMAP" => ".bmp",
            "RT_CURSOR" | "RT_GROUP_CURSOR" => ".cur",
            "RT_ANICURSOR" => ".ani",
            "RT_ANIICON" => ".ani",
            "RT_MANIFEST" => ".xml",
            "RT_HTML" => ".html",
            "RT_VERSION" => ".rc",
            "RT_STRING" | "RT_MESSAGETABLE" => ".txt",
            "RT_FONT" => ".fnt",
            "RT_FONTDIR" => ".fon",
            "RT_MENU" | "RT_DIALOG" | "RT_ACCELERATOR" => ".rc",
            _ => ".bin",
        }
    }

    /// 根据文件内容的魔数推断文件类型
    fn detect_file_type(data: &[u8]) -> Option<&'static str> {
        if data.len() < 4 {
            return None;
        }

        // PNG: 89 50 4E 47 0D 0A 1A 0A
        if data.len() >= 8
            && data[0] == 0x89
            && data[1] == 0x50
            && data[2] == 0x4E
            && data[3] == 0x47
        {
            return Some(".png");
        }

        // JPEG: FF D8 FF
        if data.len() >= 3 && data[0] == 0xFF && data[1] == 0xD8 && data[2] == 0xFF {
            return Some(".jpg");
        }

        // BMP: 42 4D (BM)
        if data[0] == 0x42 && data[1] == 0x4D {
            return Some(".bmp");
        }

        // ICO: 00 00 01 00
        if data.len() >= 4
            && data[0] == 0x00
            && data[1] == 0x00
            && data[2] == 0x01
            && data[3] == 0x00
        {
            return Some(".ico");
        }

        // CUR: 00 00 02 00
        if data.len() >= 4
            && data[0] == 0x00
            && data[1] == 0x00
            && data[2] == 0x02
            && data[3] == 0x00
        {
            return Some(".cur");
        }

        // GIF: 47 49 46 38 (GIF8)
        if data.len() >= 4
            && data[0] == 0x47
            && data[1] == 0x49
            && data[2] == 0x46
            && data[3] == 0x38
        {
            return Some(".gif");
        }

        // XML/HTML: 3C 3F 78 6D (<?xml) or 3C 68 74 6D (<htm)
        if data.len() >= 4 && data[0] == 0x3C {
            if data[1] == 0x3F && data[2] == 0x78 && data[3] == 0x6D {
                return Some(".xml");
            }
            if data.len() >= 5
                && data[1] == 0x68
                && data[2] == 0x74
                && data[3] == 0x6D
                && (data[4] == 0x6C || data[4] == 0x3E)
            {
                return Some(".html");
            }
            // 其他XML格式
            if data.len() >= 5 && data[1] == 0x3F {
                return Some(".xml");
            }
        }

        // PE/DLL: 4D 5A (MZ)
        if data[0] == 0x4D && data[1] == 0x5A {
            return Some(".dll");
        }

        // ZIP: 50 4B 03 04
        if data.len() >= 4
            && data[0] == 0x50
            && data[1] == 0x4B
            && data[2] == 0x03
            && data[3] == 0x04
        {
            return Some(".zip");
        }

        // 尝试检测是否是文本文件（UTF-8 BOM: EF BB BF）
        if data.len() >= 3 && data[0] == 0xEF && data[1] == 0xBB && data[2] == 0xBF {
            return Some(".txt");
        }

        // 检测是否是纯ASCII文本
        if data
            .iter()
            .take(100)
            .all(|&b| b == 0x09 || b == 0x0A || b == 0x0D || (b >= 0x20 && b <= 0x7E) || b >= 0x80)
        {
            return Some(".txt");
        }

        None
    }

    /// 为资源文件生成合适的文件名（带扩展名）
    fn generate_filename(
        resource_path: &str,
        file_data: &[u8],
        parent_type: Option<&str>,
    ) -> String {
        // 如果文件名已经有扩展名，直接使用
        if resource_path.contains('.') {
            return resource_path.to_string();
        }

        // 首先尝试根据文件内容推断类型
        if let Some(ext) = Self::detect_file_type(file_data) {
            return format!("{}{}", resource_path, ext);
        }

        // 如果有父级资源类型信息，使用默认扩展名
        if let Some(res_type) = parent_type {
            let ext = Self::get_default_extension(res_type);
            return format!("{}{}", resource_path, ext);
        }

        // 默认使用.bin扩展名
        format!("{}.bin", resource_path)
    }

    /// 将资源类型ID转换为可读的类型名称
    fn get_resource_type_name(id: u32) -> String {
        match id {
            1 => "RT_CURSOR".to_string(),
            2 => "RT_BITMAP".to_string(),
            3 => "RT_ICON".to_string(),
            4 => "RT_MENU".to_string(),
            5 => "RT_DIALOG".to_string(),
            6 => "RT_STRING".to_string(),
            7 => "RT_FONTDIR".to_string(),
            8 => "RT_FONT".to_string(),
            9 => "RT_ACCELERATOR".to_string(),
            10 => "RT_RCDATA".to_string(),
            11 => "RT_MESSAGETABLE".to_string(),
            12 => "RT_GROUP_CURSOR".to_string(),
            14 => "RT_GROUP_ICON".to_string(),
            16 => "RT_VERSION".to_string(),
            17 => "RT_DLGINCLUDE".to_string(),
            19 => "RT_PLUGPLAY".to_string(),
            20 => "RT_VXD".to_string(),
            21 => "RT_ANICURSOR".to_string(),
            22 => "RT_ANIICON".to_string(),
            23 => "RT_HTML".to_string(),
            24 => "RT_MANIFEST".to_string(),
            _ => format!("RT_UNKNOWN_{}", id),
        }
    }

    /// 读取资源名称字符串（Unicode）
    fn read_resource_name(
        file: &mut File,
        base_offset: u32,
        name_offset: u32,
    ) -> anyhow::Result<String> {
        // name_offset的低31位是相对于资源段基址的偏移
        let offset = base_offset + (name_offset & 0x7FFFFFFF);
        file.seek(SeekFrom::Start(offset as u64))?;

        // 前两个字节是字符串长度
        let mut len_buf = [0u8; 2];
        file.read_exact(&mut len_buf)?;
        let len = u16::from_le_bytes(len_buf) as usize;

        // 读取Unicode字符串
        let mut name_buf = vec![0u16; len];
        for i in 0..len {
            let mut char_buf = [0u8; 2];
            file.read_exact(&mut char_buf)?;
            name_buf[i] = u16::from_le_bytes(char_buf);
        }

        Ok(String::from_utf16_lossy(&name_buf))
    }

    /// 获取条目名称（可能是ID或字符串）
    fn get_entry_name(
        file: &mut File,
        base_offset: u32,
        name_offset: u32,
        is_type_level: bool,
    ) -> anyhow::Result<String> {
        if name_offset & 0x80000000 != 0 {
            // 高位为1，表示是命名资源
            Self::read_resource_name(file, base_offset, name_offset)
        } else {
            // 高位为0，表示是ID资源
            let id = name_offset & 0x7FFFFFFF;
            if is_type_level {
                // 如果是类型层级，转换为类型名称
                Ok(Self::get_resource_type_name(id))
            } else {
                Ok(format!("ID_{}", id))
            }
        }
    }

    /// 递归解析资源目录树
    fn parse_resource_directory<E>(
        file: &mut File,
        base_offset: u32,
        relative_offset: u32,
        nt_head: &E,
        image_section_headers: &ImageSectionHeaders,
        resource_base_rva: u32,
        is_type_level: bool,
    ) -> anyhow::Result<Self>
    where
        E: NtHeaders + ?Sized,
    {
        let current_offset = base_offset + relative_offset;
        let resource_directory = ImageResourceDirectory::new(file, current_offset)?;

        let mut resource_root = Self::new("Directory".to_string(), true, current_offset, 0);
        let total_entries =
            resource_directory.number_of_named_entries + resource_directory.number_of_id_entries;

        let entries_offset = current_offset + size_of::<ImageResourceDirectory>() as u32;

        for i in 0..total_entries {
            let entry_offset =
                entries_offset + (i as u32 * size_of::<ImageResourceDirectoryEntry>() as u32);
            let entry = ImageResourceDirectoryEntry::new(file, entry_offset)?;

            let entry_name =
                Self::get_entry_name(file, base_offset, entry.name_offset, is_type_level)?;

            // 检查是否是子目录
            if entry.offset_to_data & 0x80000000 != 0 {
                let subdir_offset = entry.offset_to_data & 0x7FFFFFFF;
                let subdirectory = Self::parse_resource_directory(
                    file,
                    base_offset,
                    subdir_offset,
                    nt_head,
                    image_section_headers,
                    resource_base_rva,
                    false,
                )?;

                let mut subdir_node = Self::new(entry_name, true, entry_offset, 0);
                for child in subdirectory.children.unwrap_or_default() {
                    subdir_node.add_child(child);
                }
                resource_root.add_child(subdir_node);
            } else {
                let data_entry_offset = entry.offset_to_data & 0x7FFFFFFF;
                let data_entry_file_offset = base_offset + data_entry_offset;

                file.seek(SeekFrom::Start(data_entry_file_offset as u64))?;
                let data_entry = unsafe {
                    let mut data_entry = MaybeUninit::<ImageResourceDataEntry>::uninit();
                    let bytes = std::slice::from_raw_parts_mut(
                        data_entry.as_mut_ptr() as *mut u8,
                        size_of::<ImageResourceDataEntry>(),
                    );
                    file.read_exact(bytes)?;
                    data_entry.assume_init()
                };
                let data_file_offset =
                    rva_2_fo(nt_head, image_section_headers, data_entry.data_offset).unwrap_or(0);

                let data_node =
                    Self::new(entry_name, false, data_file_offset, data_entry.data_size);
                resource_root.add_child(data_node);
            }
        }

        Ok(resource_root)
    }

    pub fn get_resource_tree<E>(
        file: &mut File,
        address: u32,
        nt_head: &E,
        image_section_headers: &ImageSectionHeaders,
        _data_dir: &DataDirectory,
    ) -> anyhow::Result<Self>
    where
        E: NtHeaders + ?Sized,
    {
        let base_offset = rva_2_fo(nt_head, image_section_headers, address)
            .ok_or(anyhow::anyhow!(i18n::ERROR_GET_RVA_OFFSET))?;

        Self::parse_resource_directory(
            file,
            base_offset,
            0,
            nt_head,
            image_section_headers,
            address,
            true,
        )
    }

    /// 从RT_GROUP_ICON或RT_GROUP_CURSOR数据构建完整的ICO/CUR文件
    fn build_icon_file(
        group_data: &[u8],
        icon_data_map: &HashMap<u16, Vec<u8>>,
        is_cursor: bool,
    ) -> anyhow::Result<Vec<u8>> {
        if group_data.len() < 6 {
            return Err(anyhow::anyhow!("Group icon data too small"));
        }

        // 读取文件头（前6字节）
        let reserved = u16::from_le_bytes([group_data[0], group_data[1]]);
        let _icon_type = u16::from_le_bytes([group_data[2], group_data[3]]);
        let image_count = u16::from_le_bytes([group_data[4], group_data[5]]);

        let mut result = Vec::new();

        // 写入ICO/CUR文件头
        result.extend_from_slice(&reserved.to_le_bytes());
        result.extend_from_slice(&if is_cursor { 2u16 } else { 1u16 }.to_le_bytes());
        result.extend_from_slice(&image_count.to_le_bytes());

        // 解析每个图标目录条目
        let mut icon_entries = Vec::new();
        let entry_size = size_of::<GroupIconDirEntry>();

        for i in 0..image_count as usize {
            let offset = 6 + i * entry_size;
            if offset + entry_size > group_data.len() {
                break;
            }

            let entry_data = &group_data[offset..offset + entry_size];
            let width = entry_data[0];
            let height = entry_data[1];
            let color_count = entry_data[2];
            let reserved = entry_data[3];
            let planes = u16::from_le_bytes([entry_data[4], entry_data[5]]);
            let bit_count = u16::from_le_bytes([entry_data[6], entry_data[7]]);
            let bytes_in_res =
                u32::from_le_bytes([entry_data[8], entry_data[9], entry_data[10], entry_data[11]]);
            let icon_id = u16::from_le_bytes([entry_data[12], entry_data[13]]);

            icon_entries.push((
                width,
                height,
                color_count,
                reserved,
                planes,
                bit_count,
                bytes_in_res,
                icon_id,
            ));
        }

        let header_size = 6 + icon_entries.len() * 16;
        let mut current_offset = header_size as u32;

        // 写入目录条目并收集图标数据
        let mut icon_data_list = Vec::new();

        for (width, height, color_count, reserved, planes, bit_count, _bytes_in_res, icon_id) in
            icon_entries
        {
            // 获取对应的图标数据
            if let Some(data) = icon_data_map.get(&icon_id) {
                // 写入目录条目
                result.push(width);
                result.push(height);
                result.push(color_count);
                result.push(reserved);
                result.extend_from_slice(&planes.to_le_bytes());
                result.extend_from_slice(&bit_count.to_le_bytes());
                result.extend_from_slice(&(data.len() as u32).to_le_bytes());
                result.extend_from_slice(&current_offset.to_le_bytes());

                current_offset += data.len() as u32;
                icon_data_list.push(data.clone());
            }
        }

        // 写入所有图标数据
        for data in icon_data_list {
            result.extend_from_slice(&data);
        }

        Ok(result)
    }

    /// 收集所有RT_ICON或RT_CURSOR资源
    fn collect_icon_resources(&self) -> HashMap<u16, (u32, u32)> {
        let mut icon_map = HashMap::new();

        if let Some(children) = &self.children {
            for child in children {
                // 查找RT_ICON或RT_CURSOR节点
                if child.name == "RT_ICON" || child.name == "RT_CURSOR" {
                    if let Some(icon_items) = &child.children {
                        for item in icon_items {
                            if let Some(id_str) = item.name.strip_prefix("ID_") {
                                if let Ok(id) = id_str.parse::<u16>() {
                                    // 获取实际的数据节点
                                    if let Some(data_children) = &item.children {
                                        for data_node in data_children {
                                            if data_node.children.is_none() {
                                                icon_map.insert(
                                                    id,
                                                    (data_node.data_address, data_node.size),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        icon_map
    }

    /// 提取资源到指定目录
    pub fn extract_resources<T>(
        &self,
        file: &mut File,
        output_dir: &Path,
        nt_head: &T,
        image_section_headers: &ImageSectionHeaders,
        _data_dir: &DataDirectory,
    ) -> anyhow::Result<Vec<PathBuf>>
    where
        T: NtHeaders + ?Sized,
    {
        let mut extracted_files = Vec::new();

        fs::create_dir_all(output_dir)?;

        // 首先收集所有的RT_ICON和RT_CURSOR资源
        let icon_resources = self.collect_icon_resources();

        self._extract_resources_recursive(
            file,
            output_dir,
            nt_head,
            image_section_headers,
            &mut extracted_files,
            "",
            None, // 顶层没有资源类型
            0,    // 深度为0
            &icon_resources,
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
        extracted_files: &mut Vec<PathBuf>,
        current_path: &str,
        parent_type: Option<&str>,
        depth: u32,
        icon_resources: &HashMap<u16, (u32, u32)>,
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

            // 判断当前节点是否是资源类型节点（第一层）
            let resource_type = if depth == 1 && self.name.starts_with("RT_") {
                Some(self.name.as_str())
            } else {
                parent_type
            };

            // 跳过单独的RT_ICON和RT_CURSOR节点（它们由GROUP_ICON/GROUP_CURSOR处理）
            if depth == 1 && (self.name == "RT_ICON" || self.name == "RT_CURSOR") {
                return Ok(());
            }

            let dir_path = output_dir.join(&new_path);
            fs::create_dir_all(&dir_path)?;

            for child in children {
                child._extract_resources_recursive(
                    file,
                    output_dir,
                    nt_head,
                    image_section_headers,
                    extracted_files,
                    &new_path,
                    resource_type,
                    depth + 1,
                    icon_resources,
                )?;
            }
        } else {
            // 这是一个文件节点，实际提取数据
            // data_address 已经是文件偏移了（在解析时已转换）
            if self.data_address > 0 && self.size > 0 {
                // 读取资源数据
                let mut buffer = vec![0u8; self.size as usize];
                file.seek(SeekFrom::Start(self.data_address as u64))?;
                file.read_exact(&mut buffer)?;

                // 检查是否是RT_GROUP_ICON或RT_GROUP_CURSOR
                let is_group_icon = parent_type == Some("RT_GROUP_ICON");
                let is_group_cursor = parent_type == Some("RT_GROUP_CURSOR");

                if is_group_icon || is_group_cursor {
                    // 读取所有引用的图标数据
                    let mut icon_data_map = HashMap::new();

                    // 解析group数据获取所有图标ID
                    if buffer.len() >= 6 {
                        let image_count = u16::from_le_bytes([buffer[4], buffer[5]]);
                        let entry_size = size_of::<GroupIconDirEntry>();

                        for i in 0..image_count as usize {
                            let offset = 6 + i * entry_size;
                            if offset + entry_size <= buffer.len() {
                                let icon_id =
                                    u16::from_le_bytes([buffer[offset + 12], buffer[offset + 13]]);

                                // 从icon_resources中获取图标数据
                                if let Some(&(data_address, data_size)) =
                                    icon_resources.get(&icon_id)
                                {
                                    let mut icon_data = vec![0u8; data_size as usize];
                                    file.seek(SeekFrom::Start(data_address as u64))?;
                                    file.read_exact(&mut icon_data)?;
                                    icon_data_map.insert(icon_id, icon_data);
                                }
                            }
                        }
                    }

                    // 构建完整的ICO/CUR文件
                    match Self::build_icon_file(&buffer, &icon_data_map, is_group_cursor) {
                        Ok(complete_icon) => {
                            buffer = complete_icon;
                        }
                        Err(e) => {
                            eprintln!("警告: 构建图标文件失败: {}", e);
                            // 如果失败，仍然保存原始数据
                        }
                    }
                }

                // 生成智能文件名（带扩展名）
                let filename = Self::generate_filename(&new_path, &buffer, parent_type);
                let file_path = output_dir.join(&filename);

                // 确保父目录存在
                if let Some(parent) = file_path.parent() {
                    fs::create_dir_all(parent)?;
                }

                // 写入文件
                fs::write(&file_path, buffer)?;
                extracted_files.push(file_path);
            }
        }

        Ok(())
    }
}

// /// 资源信息
// #[derive(Debug, Clone)]
// pub struct ResourceInfo {
//     pub path: String,
//     pub size: u32,
//     pub data_address: u32,
//     pub resource_type: String,
// }
