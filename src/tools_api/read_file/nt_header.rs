use crate::tools_api::read_file::nt_header::traits::NtHeaders;
use crate::tools_api::read_file::{
    DataDirectory, ImageDataDirectory, ImageDosHeader, ImageFileHeader, ImageNtHeaders,
    ImageNtHeaders64,
};
use std::io::SeekFrom;
use std::mem::transmute;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

const DIRECTORY_EXPORT: usize = 0;
const DIRECTORY_IMPORT: usize = 1;
const DIRECTORY_RESOURCE: usize = 2;
pub(crate) const MACHINE_32: [u16; 21] = [
    0x014C, 0x162, 0x166, 0x168, 0x169, 0x184, 0x1a2, 0x1a3, 0x1a4, 0x1a6, 0x1a8, 0x1c0, 0x1c2,
    0x1c4, 0x1d3, 0x1f0, 0x1f1, 0x266, 0x366, 0x466, 0x520,
];
pub(crate) const MACHINE_64: [u16; 4] = [0x8664, 0x0200, 0x0284, 0xAA64];

pub(crate) enum Characteristics {
    ImageFileRelocsStripped = 0x0001,       // 重定位信息被剥离
    ImageFileExecutableImage = 0x0002,      // 文件是可执行的（即没有未解决的外部引用）
    ImageFileLineNumsStripped = 0x0004,     // 行号被剥离
    ImageFileLocalSymsStripped = 0x0008,    // 本地符号被剥离
    ImageFileAggresiveWsTrim = 0x0010,      // 积极地修剪工作集
    ImageFileLargeAddressAware = 0x0020,    // 应用程序可以处理>2gb地址
    ImageFileBytesReversedLo = 0x0080,      // 机器字节是反向的
    ImageFile32bitMachine = 0x0100,         // 32位机器字
    ImageFileDebugStripped = 0x0200,        // 调试信息被剥离
    ImageFileRemovableRunFromSwap = 0x0400, // 如果映像在可移动媒体上，则从交换文件中复制并运行
    ImageFileNetRunFromSwap = 0x0800,       // 如果映像在网络上，则从交换文件中复制并运行
    ImageFileSystem = 0x1000,               // 系统文件
    ImageFileDll = 0x2000,                  // 文件是DLL
    ImageFileUpSystemOnly = 0x4000,         // 文件应该只在UP机器上运行
    ImageFileBytesReversedHi = 0x8000,      // 机器字节是反向的
}
/// 为 64 位 和 32 位nt头特征
pub mod traits {
    pub trait NtHeaders {
        /// 获取数据目录的数量
        fn num_of_rva(&self) -> u32;
        /// 读取段数量
        fn section_number(&self) -> u16;
        /// 读取段开始文件地址
        fn section_start(&self, nt_start: u16) -> u32;
        fn get_signature(&self) -> &str;
        fn get_machine(&self) -> &str;
        fn get_number_of_sections(&self) -> String;
        fn get_time_date_stamp(&self) -> String;
        fn get_pointer_to_symbol_table(&self) -> String;
        fn get_number_of_symbols(&self) -> String;
        fn get_size_of_optional_header(&self) -> String;
        fn get_characteristics(&self) -> String;
        fn get_characteristics_hover(&self) -> String;
        fn get_magic(&self) -> String;
        fn get_major_linker_version(&self) -> String;
        fn get_long_minor_linker_version(&self) -> String;
        fn get_size_of_code(&self) -> String;
        fn get_size_of_initialized_data(&self) -> String;
        fn get_size_of_uninitialized_data(&self) -> String;
        fn get_address_of_entry_point(&self) -> String;
        fn get_base_of_code(&self) -> String;
        fn get_base_of_data(&self) -> String;
        fn get_image_base(&self) -> String;
        fn get_section_alignment(&self) -> String;
        fn get_file_alignment(&self) -> String;
        fn get_major_os_version(&self) -> String;
        fn get_minor_os_version(&self) -> String;
        fn get_major_image_version(&self) -> String;
        fn get_minor_image_version(&self) -> String;
        fn get_major_subsystem_version(&self) -> String;
        fn get_minor_subsystem_version(&self) -> String;
        fn get_win32_version_value(&self) -> String;
        fn get_size_of_image(&self) -> String;
        fn get_size_of_headers(&self) -> String;
        fn get_checksum(&self) -> String;
        fn get_subsystem(&self) -> String;
        fn get_dll_characteristics(&self) -> String;
        fn get_size_of_stack_reserve(&self) -> String;
        fn get_size_of_stack_commit(&self) -> String;
        fn get_size_of_heap_reserve(&self) -> String;
        fn get_size_of_heap_commit(&self) -> String;
        fn get_loader_flags(&self) -> String;
        fn get_number_of_rva_and_sizes(&self) -> u32;
    }
}

impl ImageFileHeader {
    pub(crate) async fn new(
        file: &mut File,
        image_dos_header: &ImageDosHeader,
    ) -> anyhow::Result<ImageFileHeader> {
        let file_image_addr = image_dos_header.get_nt_addr().await + 4u16;
        file.seek(SeekFrom::Start(file_image_addr as u64)).await?;
        let mut image_file_header: ImageFileHeader = Default::default();
        unsafe {
            let reads: &mut [u8; 64] = transmute(&mut image_file_header);
            file.read(reads).await?;
        }
        Ok(image_file_header)
    }
}

impl DataDirectory {
    /// 添加data_directory内容
    pub(crate) fn add(&mut self, data: ImageDataDirectory) {
        self.0.push(data);
    }
    pub(crate) async fn get_export_directory_address(&self) -> anyhow::Result<u32> {
        Ok(self
            .0
            .get(crate::tools_api::read_file::nt_header::DIRECTORY_EXPORT)
            .unwrap()
            .virtual_address)
    }
    pub(crate) async fn get_import_directory_address(&self) -> anyhow::Result<u32> {
        Ok(self
            .0
            .get(crate::tools_api::read_file::nt_header::DIRECTORY_IMPORT)
            .unwrap()
            .virtual_address)
    }
    pub(crate) async fn _get_resource_directory_address(&self) -> anyhow::Result<u32> {
        Ok(self
            .0
            .get(crate::tools_api::read_file::nt_header::DIRECTORY_RESOURCE)
            .unwrap()
            .size)
    }
    pub(crate) fn get_import_directory_size(&self) -> anyhow::Result<u32> {
        Ok(self.0.get(DIRECTORY_IMPORT).unwrap().size)
    }
    /// 获取导入dll数量
    pub async fn get_import_directory_num(&self) -> anyhow::Result<usize> {
        Ok((self.get_import_directory_size()? / 0x14) as usize)
    }
    pub fn get_data_directory_size(&self, index: u32) -> anyhow::Result<u32> {
        Ok(self.0.get(index as usize).unwrap().size)
    }
    pub fn get_data_directory_virtual_address(&self, index: u32) -> anyhow::Result<u32> {
        Ok(self.0.get(index as usize).unwrap().virtual_address)
    }
}

impl NtHeaders for ImageNtHeaders {
    fn num_of_rva(&self) -> u32 {
        self.optional_header.number_of_rva_and_sizes
    }
    fn section_number(&self) -> u16 {
        self.file_header.number_of_sections
    }
    fn section_start(&self, nt_start: u16) -> u32 {
        self.num_of_rva() * 0x8 + nt_start as u32 + size_of::<ImageNtHeaders>() as u32
    }

    fn get_signature(&self) -> &str {
        "PE\0\0"
    }

    fn get_machine(&self) -> &str {
        match self.file_header.machine {
            0x14c => "32位x86架构",             // 32位
            0x0162 => "MIPS大端",               // 32位
            0x0166 => "MIPS小端",               // 32位
            0x0168 => "MIPS小端",               // 32位
            0x0169 => "MIPS小端",               // 32位
            0x0184 => "Alpha",                  // 32位
            0x01a2 => "SH3小端",                // 32位
            0x01a3 => "SH3小端",                // 32位
            0x01a4 => "SH3E小端",               // 32位
            0x01a6 => "SH4小端",                // 32位
            0x01a8 => "SH5",                    // 32位
            0x01c0 => "ARM小端",                // 32位
            0x01c2 => "ARM Thumb/Thumb-2 小端", // 32位
            0x01c4 => "ARM Thumb/Thumb-2 小端", // 32位
            0x01d3 => "ARM",                    // 32位
            0x01F0 => "IBM",                    // 32位
            0x01f1 => "POWERCFP",               // 32位
            0x0200 => "Intel 64",               // 64位
            0x0266 => "MIPS",                   // 32位
            0x0284 => "ALPHA64",                // 64位
            0x0366 => "MIPS",                   // 32位
            0x0466 => "MIPS",                   // 32位
            0x0520 => "Infineon",               // 32位
            0x8664 => "64位x64架构",            // 64位
            0xAA64 => "ARM64 小端",             // 64位
            _ => "unknown",
        }
    }

    fn get_number_of_sections(&self) -> String {
        format!("{}", self.file_header.number_of_sections)
    }

    fn get_time_date_stamp(&self) -> String {
        format!("{}", self.file_header.time_date_stamp)
    }

    fn get_pointer_to_symbol_table(&self) -> String {
        format!("{}", self.file_header.pointer_to_symbol_table)
    }

    fn get_number_of_symbols(&self) -> String {
        format!("{}", self.file_header.number_of_symbols)
    }

    fn get_size_of_optional_header(&self) -> String {
        format!("{}", self.file_header.size_of_optional_header)
    }

    fn get_characteristics(&self) -> String {
        format!("{}", self.file_header.characteristics)
    }

    fn get_characteristics_hover(&self) -> String {
        let mut characteristics_info = String::new();
        let characteristics = self.file_header.characteristics;
        if characteristics & Characteristics::ImageFileRelocsStripped as u16 != 0 {
            characteristics_info.push_str("重定位信息被剥离\n");
        }
        if characteristics & Characteristics::ImageFileExecutableImage as u16 != 0 {
            characteristics_info.push_str("文件是可执行的（即没有未解决的外部引用）\n");
        }
        if characteristics & Characteristics::ImageFileLineNumsStripped as u16 != 0 {
            characteristics_info.push_str("行号被剥离\n");
        }
        if characteristics & Characteristics::ImageFileLocalSymsStripped as u16 != 0 {
            characteristics_info.push_str("本地符号被剥离\n");
        }
        if characteristics & Characteristics::ImageFileAggresiveWsTrim as u16 != 0 {
            characteristics_info.push_str("积极地修剪工作集\n");
        }
        if characteristics & Characteristics::ImageFileLargeAddressAware as u16 != 0 {
            characteristics_info.push_str("应用程序可以处理>2gb地址\n");
        }
        if characteristics & Characteristics::ImageFileBytesReversedLo as u16 != 0 {
            characteristics_info.push_str("机器字节是反向的\n");
        }
        if characteristics & Characteristics::ImageFile32bitMachine as u16 != 0 {
            characteristics_info.push_str("32位机器字\n");
        }
        if characteristics & Characteristics::ImageFileDebugStripped as u16 != 0 {
            characteristics_info.push_str("调试信息被剥离\n");
        }
        if characteristics & Characteristics::ImageFileRemovableRunFromSwap as u16 != 0 {
            characteristics_info.push_str("如果映像在可移动媒体上，则从交换文件中复制并运行\n");
        }
        if characteristics & Characteristics::ImageFileNetRunFromSwap as u16 != 0 {
            characteristics_info.push_str("如果映像在网络上，则从交换文件中复制并运行\n");
        }
        if characteristics & Characteristics::ImageFileSystem as u16 != 0 {
            characteristics_info.push_str("系统文件\n");
        }
        if characteristics & Characteristics::ImageFileDll as u16 != 0 {
            characteristics_info.push_str("文件是DLL\n");
        }
        if characteristics & Characteristics::ImageFileUpSystemOnly as u16 != 0 {
            characteristics_info.push_str("文件应该只在UP机器上运行\n");
        }
        if characteristics & Characteristics::ImageFileBytesReversedHi as u16 != 0 {
            characteristics_info.push_str("机器字节是反向的\n");
        }
        characteristics_info
    }

    fn get_magic(&self) -> String {
        format!("{}", self.optional_header.magic)
    }

    fn get_major_linker_version(&self) -> String {
        format!("{}", self.optional_header.major_linker_version)
    }

    fn get_long_minor_linker_version(&self) -> String {
        format!("{}", self.optional_header.minor_linker_version)
    }

    fn get_size_of_code(&self) -> String {
        format!("{}", self.optional_header.size_of_code)
    }

    fn get_size_of_initialized_data(&self) -> String {
        format!("{}", self.optional_header.size_of_initialized_data)
    }

    fn get_size_of_uninitialized_data(&self) -> String {
        format!("{}", self.optional_header.size_of_uninitialized_data)
    }

    fn get_address_of_entry_point(&self) -> String {
        format!("{}", self.optional_header.address_of_entry_point)
    }

    fn get_base_of_code(&self) -> String {
        format!("{}", self.optional_header.base_of_code)
    }

    fn get_base_of_data(&self) -> String {
        format!("{}", self.optional_header.base_of_data)
    }

    fn get_image_base(&self) -> String {
        format!("{}", self.optional_header.image_base)
    }

    fn get_section_alignment(&self) -> String {
        format!("{}", self.optional_header.section_alignment)
    }

    fn get_file_alignment(&self) -> String {
        format!("{}", self.optional_header.file_alignment)
    }

    fn get_major_os_version(&self) -> String {
        format!("{}", self.optional_header.major_operating_system_version)
    }

    fn get_minor_os_version(&self) -> String {
        format!("{}", self.optional_header.minor_operating_system_version)
    }

    fn get_major_image_version(&self) -> String {
        format!("{}", self.optional_header.major_image_version)
    }

    fn get_minor_image_version(&self) -> String {
        format!("{}", self.optional_header.minor_image_version)
    }

    fn get_major_subsystem_version(&self) -> String {
        format!("{}", self.optional_header.major_subsystem_version)
    }

    fn get_minor_subsystem_version(&self) -> String {
        format!("{}", self.optional_header.minor_subsystem_version)
    }

    fn get_win32_version_value(&self) -> String {
        format!("{}", self.optional_header.win32version_value)
    }

    fn get_size_of_image(&self) -> String {
        format!("{}", self.optional_header.size_of_image)
    }

    fn get_size_of_headers(&self) -> String {
        format!("{}", self.optional_header.size_of_headers)
    }

    fn get_checksum(&self) -> String {
        format!("{}", self.optional_header.check_sum)
    }

    fn get_subsystem(&self) -> String {
        format!("{}", self.optional_header.subsystem)
    }

    fn get_dll_characteristics(&self) -> String {
        format!("{}", self.optional_header.dll_characteristics)
    }

    fn get_size_of_stack_reserve(&self) -> String {
        format!("{}", self.optional_header.size_of_stack_reserve)
    }

    fn get_size_of_stack_commit(&self) -> String {
        format!("{}", self.optional_header.size_of_stack_commit)
    }

    fn get_size_of_heap_reserve(&self) -> String {
        format!("{}", self.optional_header.size_of_heap_reserve)
    }

    fn get_size_of_heap_commit(&self) -> String {
        format!("{}", self.optional_header.size_of_heap_commit)
    }

    fn get_loader_flags(&self) -> String {
        format!("{}", self.optional_header.loader_flags)
    }

    fn get_number_of_rva_and_sizes(&self) -> u32 {
        self.optional_header.number_of_rva_and_sizes
    }
}
impl NtHeaders for ImageNtHeaders64 {
    fn num_of_rva(&self) -> u32 {
        self.optional_header.number_of_rva_and_sizes
    }
    fn section_number(&self) -> u16 {
        self.file_header.number_of_sections
    }
    fn section_start(&self, nt_start: u16) -> u32 {
        self.num_of_rva() * 0x8 + nt_start as u32 + size_of::<ImageNtHeaders64>() as u32
    }
    fn get_signature(&self) -> &str {
        "PE\0\0"
    }

    fn get_machine(&self) -> &str {
        match self.file_header.machine {
            0x14c => "32位x86架构",
            0x0162 => "MIPS大端",
            0x0166 => "MIPS小端",
            0x0168 => "MIPS小端",
            0x0169 => "MIPS小端",
            0x0184 => "Alpha",
            0x01a2 => "SH3小端",
            0x01a3 => "SH3小端",
            0x01a4 => "SH3E小端",
            0x01a6 => "SH4小端",
            0x01a8 => "SH5",
            0x01c0 => "ARM小端",
            0x01c2 => "ARM Thumb/Thumb-2 小端",
            0x01c4 => "ARM Thumb/Thumb-2 小端",
            0x01d3 => "ARM",
            0x01F0 => "IBM",
            0x01f1 => "POWERCFP",
            0x0200 => "Intel 64",
            0x0266 => "MIPS",
            0x0284 => "ALPHA64",
            0x0366 => "MIPS",
            0x0466 => "MIPS",
            0x0520 => "Infineon",
            0x8664 => "64位x64架构",
            0xAA64 => "ARM64 小端",
            _ => "unknown",
        }
    }

    fn get_number_of_sections(&self) -> String {
        format!("{}", self.file_header.number_of_sections)
    }

    fn get_time_date_stamp(&self) -> String {
        format!("{}", self.file_header.time_date_stamp)
    }

    fn get_pointer_to_symbol_table(&self) -> String {
        format!("{}", self.file_header.pointer_to_symbol_table)
    }

    fn get_number_of_symbols(&self) -> String {
        format!("{}", self.file_header.number_of_symbols)
    }

    fn get_size_of_optional_header(&self) -> String {
        format!("{}", self.file_header.size_of_optional_header)
    }

    fn get_characteristics(&self) -> String {
        format!("{}", self.file_header.characteristics)
    }
    fn get_characteristics_hover(&self) -> String {
        let mut characteristics_info = String::new();
        let characteristics = self.file_header.characteristics;
        if characteristics & Characteristics::ImageFileRelocsStripped as u16 != 0 {
            characteristics_info.push_str("重定位信息被剥离\n");
        }
        if characteristics & Characteristics::ImageFileExecutableImage as u16 != 0 {
            characteristics_info.push_str("文件是可执行的（即没有未解决的外部引用）\n");
        }
        if characteristics & Characteristics::ImageFileLineNumsStripped as u16 != 0 {
            characteristics_info.push_str("行号被剥离\n");
        }
        if characteristics & Characteristics::ImageFileLocalSymsStripped as u16 != 0 {
            characteristics_info.push_str("本地符号被剥离\n");
        }
        if characteristics & Characteristics::ImageFileAggresiveWsTrim as u16 != 0 {
            characteristics_info.push_str("积极地修剪工作集\n");
        }
        if characteristics & Characteristics::ImageFileLargeAddressAware as u16 != 0 {
            characteristics_info.push_str("应用程序可以处理>2gb地址\n");
        }
        if characteristics & Characteristics::ImageFileBytesReversedLo as u16 != 0 {
            characteristics_info.push_str("机器字节是反向的\n");
        }
        if characteristics & Characteristics::ImageFile32bitMachine as u16 != 0 {
            characteristics_info.push_str("32位机器字\n");
        }
        if characteristics & Characteristics::ImageFileDebugStripped as u16 != 0 {
            characteristics_info.push_str("调试信息被剥离\n");
        }
        if characteristics & Characteristics::ImageFileRemovableRunFromSwap as u16 != 0 {
            characteristics_info.push_str("如果映像在可移动媒体上，则从交换文件中复制并运行\n");
        }
        if characteristics & Characteristics::ImageFileNetRunFromSwap as u16 != 0 {
            characteristics_info.push_str("如果映像在网络上，则从交换文件中复制并运行\n");
        }
        if characteristics & Characteristics::ImageFileSystem as u16 != 0 {
            characteristics_info.push_str("系统文件\n");
        }
        if characteristics & Characteristics::ImageFileDll as u16 != 0 {
            characteristics_info.push_str("文件是DLL\n");
        }
        if characteristics & Characteristics::ImageFileUpSystemOnly as u16 != 0 {
            characteristics_info.push_str("文件应该只在UP机器上运行\n");
        }
        if characteristics & Characteristics::ImageFileBytesReversedHi as u16 != 0 {
            characteristics_info.push_str("机器字节是反向的\n");
        }
        characteristics_info
    }

    fn get_magic(&self) -> String {
        format!("{}", self.optional_header.magic)
    }

    fn get_major_linker_version(&self) -> String {
        format!("{}", self.optional_header.major_linker_version)
    }

    fn get_long_minor_linker_version(&self) -> String {
        format!("{}", self.optional_header.minor_image_version)
    }

    fn get_size_of_code(&self) -> String {
        format!("{}", self.optional_header.size_of_code)
    }

    fn get_size_of_initialized_data(&self) -> String {
        format!("{}", self.optional_header.size_of_initialized_data)
    }

    fn get_size_of_uninitialized_data(&self) -> String {
        format!("{}", self.optional_header.size_of_uninitialized_data)
    }

    fn get_address_of_entry_point(&self) -> String {
        format!("{}", self.optional_header.address_of_entry_point)
    }

    fn get_base_of_code(&self) -> String {
        format!("{}", self.optional_header.base_of_code)
    }

    fn get_base_of_data(&self) -> String {
        format!("{}", self.optional_header.image_base)
    }

    fn get_image_base(&self) -> String {
        format!("{}", self.optional_header.image_base)
    }

    fn get_section_alignment(&self) -> String {
        format!("{}", self.optional_header.section_alignment)
    }

    fn get_file_alignment(&self) -> String {
        format!("{}", self.optional_header.file_alignment)
    }

    fn get_major_os_version(&self) -> String {
        format!("{}", self.optional_header.major_operating_system_version)
    }

    fn get_minor_os_version(&self) -> String {
        format!("{}", self.optional_header.minor_operating_system_version)
    }

    fn get_major_image_version(&self) -> String {
        format!("{}", self.optional_header.major_image_version)
    }

    fn get_minor_image_version(&self) -> String {
        format!("{}", self.optional_header.minor_image_version)
    }

    fn get_major_subsystem_version(&self) -> String {
        format!("{}", self.optional_header.major_subsystem_version)
    }

    fn get_minor_subsystem_version(&self) -> String {
        format!("{}", self.optional_header.minor_subsystem_version)
    }

    fn get_win32_version_value(&self) -> String {
        format!("{}", self.optional_header.win32_version_value)
    }

    fn get_size_of_image(&self) -> String {
        format!("{}", self.optional_header.size_of_image)
    }

    fn get_size_of_headers(&self) -> String {
        format!("{}", self.optional_header.size_of_headers)
    }

    fn get_checksum(&self) -> String {
        format!("{}", self.optional_header.checksum)
    }

    fn get_subsystem(&self) -> String {
        format!("{}", self.optional_header.subsystem)
    }

    fn get_dll_characteristics(&self) -> String {
        format!("{}", self.optional_header.dll_characteristics)
    }

    fn get_size_of_stack_reserve(&self) -> String {
        format!("{}", self.optional_header.size_of_stack_reserve)
    }

    fn get_size_of_stack_commit(&self) -> String {
        format!("{}", self.optional_header.size_of_stack_commit)
    }

    fn get_size_of_heap_reserve(&self) -> String {
        format!("{}", self.optional_header.size_of_heap_reserve)
    }

    fn get_size_of_heap_commit(&self) -> String {
        format!("{}", self.optional_header.size_of_heap_commit)
    }

    fn get_loader_flags(&self) -> String {
        format!("{}", self.optional_header.loader_flags)
    }

    fn get_number_of_rva_and_sizes(&self) -> u32 {
        self.optional_header.number_of_rva_and_sizes
    }
}

pub(crate) async fn read_nt_head<T>(
    file: &mut File,
    start_addr: u16,
) -> anyhow::Result<(T, Box<DataDirectory>)>
where
    T: NtHeaders + Default,
    [(); size_of::<T>()]:,
{
    let mut nt_head: T = Default::default();
    let mut data_dictionary = Box::new(DataDirectory(Vec::new()));
    file.seek(SeekFrom::Start(start_addr as u64)).await?;
    unsafe {
        let reads: &mut [u8; size_of::<T>()] = transmute(&mut nt_head);
        file.read(reads).await?;
    }
    for _ in 0..nt_head.num_of_rva() {
        let mut image_data: ImageDataDirectory = Default::default();
        unsafe {
            let reads: &mut [u8; 8] = transmute(&mut image_data);
            file.read(reads).await?;
        }
        data_dictionary.add(image_data);
    }

    Ok((nt_head, data_dictionary))
}
