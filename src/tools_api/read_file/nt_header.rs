use crate::i18n;
use crate::tools_api::read_file::SerializableNtHeaders;
use crate::tools_api::read_file::nt_header::traits::NtHeaders;
use crate::tools_api::read_file::{
    DataDirectory, ImageDataDirectory, ImageDosHeader, ImageFileHeader, ImageNtHeaders,
    ImageNtHeaders64,
};

use std::fs::File;
use std::io::SeekFrom;
use std::io::{Read, Seek};
use std::mem::transmute;

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
pub(crate) enum DllCharacteristics {
    APPCONTAINER = 4096,
    ControlFlowGuard = 16384,
    DynamicBase = 64,
    ForceIntegrity = 128,
    HighEntropyVA = 32,
    NOBIND = 2048,
    NOLSOLATION = 512,
    NOSEH = 1024,
    NXCOMPAT = 256,
    PROCESSINIT = 1,
    PROCESSTERM = 2,
    TERMINALSERVERAWARE = 32768,
    THREADINIT = 4,
    THREADTERM = 8,
    WDMDRIVER = 8192,
}

/// 辅助函数：根据特征值生成描述信息
fn get_characteristics_descriptions(characteristics: u16) -> String {
    const CHARACTERISTICS_DESCRIPTIONS: &[(u16, &str)] = &[
        (
            Characteristics::ImageFileRelocsStripped as u16,
            i18n::CHARACTERISTICS_RELOCS_STRIPPED,
        ),
        (
            Characteristics::ImageFileExecutableImage as u16,
            i18n::CHARACTERISTICS_EXECUTABLE_IMAGE,
        ),
        (
            Characteristics::ImageFileLineNumsStripped as u16,
            i18n::CHARACTERISTICS_LINE_NUMS_STRIPPED,
        ),
        (
            Characteristics::ImageFileLocalSymsStripped as u16,
            i18n::CHARACTERISTICS_LOCAL_SYMS_STRIPPED,
        ),
        (
            Characteristics::ImageFileAggresiveWsTrim as u16,
            i18n::CHARACTERISTICS_AGGRESSIVE_WS_TRIM,
        ),
        (
            Characteristics::ImageFileLargeAddressAware as u16,
            i18n::CHARACTERISTICS_LARGE_ADDRESS_AWARE,
        ),
        (
            Characteristics::ImageFileBytesReversedLo as u16,
            i18n::CHARACTERISTICS_BYTES_REVERSED_LO,
        ),
        (
            Characteristics::ImageFile32bitMachine as u16,
            i18n::CHARACTERISTICS_32BIT_MACHINE,
        ),
        (
            Characteristics::ImageFileDebugStripped as u16,
            i18n::CHARACTERISTICS_DEBUG_STRIPPED,
        ),
        (
            Characteristics::ImageFileRemovableRunFromSwap as u16,
            i18n::CHARACTERISTICS_REMOVABLE_RUN_FROM_SWAP,
        ),
        (
            Characteristics::ImageFileNetRunFromSwap as u16,
            i18n::CHARACTERISTICS_NET_RUN_FROM_SWAP,
        ),
        (
            Characteristics::ImageFileSystem as u16,
            i18n::CHARACTERISTICS_SYSTEM,
        ),
        (
            Characteristics::ImageFileDll as u16,
            i18n::CHARACTERISTICS_DLL,
        ),
        (
            Characteristics::ImageFileUpSystemOnly as u16,
            i18n::CHARACTERISTICS_UP_SYSTEM_ONLY,
        ),
        (
            Characteristics::ImageFileBytesReversedHi as u16,
            i18n::CHARACTERISTICS_BYTES_REVERSED_HI,
        ),
    ];

    CHARACTERISTICS_DESCRIPTIONS
        .iter()
        .filter_map(|(flag, description)| {
            if characteristics & flag != 0 {
                Some(*description)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
fn get_dll_characteristics_description(dll_characteristics: u16) -> String {
    const DLL_CHARACTERISTICS_DESCRIPTIONS: &[(u16, &str)] = &[
        (
            DllCharacteristics::APPCONTAINER as u16,
            i18n::DLL_CHARACTERISTICS_APPCONTAINER,
        ),
        (
            DllCharacteristics::ControlFlowGuard as u16,
            i18n::DLL_CHARACTERISTICS_CONTROL_FLOW_GUARD,
        ),
        (
            DllCharacteristics::DynamicBase as u16,
            i18n::DLL_CHARACTERISTICS_DYNAMIC_BASE,
        ),
        (
            DllCharacteristics::ForceIntegrity as u16,
            i18n::DLL_CHARACTERISTICS_FORCE_INTEGRITY,
        ),
        (
            DllCharacteristics::HighEntropyVA as u16,
            i18n::DLL_CHARACTERISTICS_HIGH_ENTROPY_VA,
        ),
        (
            DllCharacteristics::NOBIND as u16,
            i18n::DLL_CHARACTERISTICS_NOBIND,
        ),
        (
            DllCharacteristics::NOLSOLATION as u16,
            i18n::DLL_CHARACTERISTICS_NOLSOLATION,
        ),
        (
            DllCharacteristics::NOSEH as u16,
            i18n::DLL_CHARACTERISTICS_NOSEH,
        ),
        (
            DllCharacteristics::NXCOMPAT as u16,
            i18n::DLL_CHARACTERISTICS_NXCOMPAT,
        ),
        (
            DllCharacteristics::PROCESSINIT as u16,
            i18n::DLL_CHARACTERISTICS_PROCESSINIT,
        ),
        (
            DllCharacteristics::PROCESSTERM as u16,
            i18n::DLL_CHARACTERISTICS_PROCESSTERM,
        ),
        (
            DllCharacteristics::TERMINALSERVERAWARE as u16,
            i18n::DLL_CHARACTERISTICS_TERMINALSERVERAWARE,
        ),
        (
            DllCharacteristics::THREADINIT as u16,
            i18n::DLL_CHARACTERISTICS_THREADINIT,
        ),
        (
            DllCharacteristics::THREADTERM as u16,
            i18n::DLL_CHARACTERISTICS_THREADTERM,
        ),
        (
            DllCharacteristics::WDMDRIVER as u16,
            i18n::DLL_CHARACTERISTICS_WDMDRIVER,
        ),
    ];
    DLL_CHARACTERISTICS_DESCRIPTIONS
        .iter()
        .filter_map(|(flag, description)| {
            if dll_characteristics & flag != 0 {
                Some(*description)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}
fn get_machine_descriptions(machine: u16) -> &'static str {
    match machine {
        0x14c => i18n::MACHINE_X86_32,
        0x0162 => i18n::MACHINE_MIPS_BIG_ENDIAN,
        0x0166 => i18n::MACHINE_MIPS_LITTLE_ENDIAN,
        0x0168 => i18n::MACHINE_MIPS_LITTLE_ENDIAN,
        0x0169 => i18n::MACHINE_MIPS_LITTLE_ENDIAN,
        0x0184 => i18n::MACHINE_ALPHA,
        0x01a2 => i18n::MACHINE_SH3_LITTLE_ENDIAN,
        0x01a3 => i18n::MACHINE_SH3_LITTLE_ENDIAN,
        0x01a4 => i18n::MACHINE_SH3E_LITTLE_ENDIAN,
        0x01a6 => i18n::MACHINE_SH4_LITTLE_ENDIAN,
        0x01a8 => i18n::MACHINE_SH5,
        0x01c0 => i18n::MACHINE_ARM_LITTLE_ENDIAN,
        0x01c2 => i18n::MACHINE_ARM_THUMB_LITTLE_ENDIAN,
        0x01c4 => i18n::MACHINE_ARM_THUMB_LITTLE_ENDIAN,
        0x01d3 => i18n::MACHINE_ARM,
        0x01F0 => i18n::MACHINE_IBM,
        0x01f1 => i18n::MACHINE_POWERCFP,
        0x0200 => i18n::MACHINE_INTEL_64,
        0x0266 => i18n::MACHINE_MIPS,
        0x0284 => i18n::MACHINE_ALPHA64,
        0x0366 => i18n::MACHINE_MIPS,
        0x0466 => i18n::MACHINE_MIPS,
        0x0520 => i18n::MACHINE_INFINEON,
        0x8664 => i18n::MACHINE_X64_64,
        0xAA64 => i18n::MACHINE_ARM64_LITTLE_ENDIAN,
        _ => i18n::MACHINE_UNKNOWN,
    }
}
/// 为 64 位 和 32 位nt头特征
pub mod traits {
    use crate::tools_api::read_file::SerializableNtHeaders;
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
        fn get_magic(&self) -> u16;
        fn get_magic_hover(&self) -> String;
        fn get_major_linker_version(&self) -> String;
        fn get_long_minor_linker_version(&self) -> String;
        fn get_size_of_code(&self) -> String;
        fn get_size_of_initialized_data(&self) -> String;
        fn get_size_of_uninitialized_data(&self) -> String;
        fn get_address_of_entry_point(&self) -> u32;
        fn get_base_of_code(&self) -> u32;
        fn get_base_of_data(&self) -> u64;
        fn get_image_base(&self) -> u64;
        fn get_section_alignment(&self) -> u32;
        fn get_file_alignment(&self) -> u32;
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
        fn get_dll_characteristics_hover(&self) -> String;
        fn get_size_of_stack_reserve(&self) -> u64;
        fn get_size_of_stack_commit(&self) -> u64;
        fn get_size_of_heap_reserve(&self) -> u64;
        fn get_size_of_heap_commit(&self) -> u64;
        fn get_loader_flags(&self) -> u32;
        fn get_number_of_rva_and_sizes(&self) -> u32;

        // 序列化
        fn serde_serialize(&self) -> SerializableNtHeaders;
    }
}

impl ImageFileHeader {
    pub(crate) fn new(
        file: &mut File,
        image_dos_header: &ImageDosHeader,
    ) -> anyhow::Result<ImageFileHeader> {
        let file_image_addr = image_dos_header.get_nt_addr() + 4u16;
        file.seek(SeekFrom::Start(file_image_addr as u64))?;
        let mut image_file_header: ImageFileHeader = Default::default();
        unsafe {
            let reads: &mut [u8; 64] = transmute(&mut image_file_header);
            file.read(reads)?;
        }
        Ok(image_file_header)
    }
}

impl DataDirectory {
    /// 去除async
    /// 添加data_directory内容
    pub(crate) fn add(&mut self, data: ImageDataDirectory) {
        self.0.push(data);
    }
    pub(crate) fn get_export_directory_address(&self) -> anyhow::Result<u32> {
        Ok(self
            .0
            .get(crate::tools_api::read_file::nt_header::DIRECTORY_EXPORT)
            .unwrap()
            .virtual_address)
    }
    pub(crate) fn get_import_directory_address(&self) -> anyhow::Result<u32> {
        Ok(self
            .0
            .get(crate::tools_api::read_file::nt_header::DIRECTORY_IMPORT)
            .unwrap()
            .virtual_address)
    }
    pub(crate) fn get_resource_directory_address(&self) -> anyhow::Result<u32> {
        Ok(self
            .0
            .get(crate::tools_api::read_file::nt_header::DIRECTORY_RESOURCE)
            .unwrap()
            .virtual_address)
    }
    pub(crate) fn get_import_directory_size(&self) -> anyhow::Result<u32> {
        Ok(self.0.get(DIRECTORY_IMPORT).unwrap().size)
    }
    /// 获取导入dll数量
    pub fn get_import_directory_num(&self) -> anyhow::Result<usize> {
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
        get_machine_descriptions(self.file_header.machine)
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
        format!("0x{:X}", self.file_header.characteristics)
    }

    fn get_characteristics_hover(&self) -> String {
        get_characteristics_descriptions(self.file_header.characteristics)
    }

    fn get_magic(&self) -> u16 {
        self.optional_header.magic
    }

    fn get_magic_hover(&self) -> String {
        match self.optional_header.magic {
            0x10b => String::from("PE32"),
            0x20b => String::from("PE64"),
            _ => String::from("unknown"),
        }
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

    fn get_address_of_entry_point(&self) -> u32 {
        self.optional_header.address_of_entry_point
    }

    fn get_base_of_code(&self) -> u32 {
        self.optional_header.base_of_code
    }

    fn get_base_of_data(&self) -> u64 {
        self.optional_header.image_base as u64
    }

    fn get_image_base(&self) -> u64 {
        self.optional_header.image_base as u64
    }

    fn get_section_alignment(&self) -> u32 {
        self.optional_header.section_alignment
    }

    fn get_file_alignment(&self) -> u32 {
        self.optional_header.file_alignment
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
        format!("0x{:X}", self.optional_header.dll_characteristics)
    }

    fn get_dll_characteristics_hover(&self) -> String {
        get_dll_characteristics_description(self.optional_header.dll_characteristics)
    }

    fn get_size_of_stack_reserve(&self) -> u64 {
        self.optional_header.size_of_stack_reserve as u64
    }

    fn get_size_of_stack_commit(&self) -> u64 {
        self.optional_header.size_of_stack_commit as u64
    }

    fn get_size_of_heap_reserve(&self) -> u64 {
        self.optional_header.size_of_heap_reserve as u64
    }

    fn get_size_of_heap_commit(&self) -> u64 {
        self.optional_header.size_of_heap_commit as u64
    }

    fn get_loader_flags(&self) -> u32 {
        self.optional_header.loader_flags
    }

    fn get_number_of_rva_and_sizes(&self) -> u32 {
        self.optional_header.number_of_rva_and_sizes
    }

    fn serde_serialize(&self) -> SerializableNtHeaders {
        SerializableNtHeaders::ImageNtHeaders32(self.clone())
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
        get_machine_descriptions(self.file_header.machine)
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
        format!("0x{:X}", self.file_header.characteristics)
    }
    fn get_characteristics_hover(&self) -> String {
        get_characteristics_descriptions(self.file_header.characteristics)
    }

    fn get_magic(&self) -> u16 {
        self.optional_header.magic
    }

    fn get_magic_hover(&self) -> String {
        match self.optional_header.magic {
            0x10b => String::from("PE32"),
            0x20b => String::from("PE64"),
            _ => String::from("unknown"),
        }
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

    fn get_address_of_entry_point(&self) -> u32 {
        self.optional_header.address_of_entry_point
    }

    fn get_base_of_code(&self) -> u32 {
        self.optional_header.base_of_code
    }

    fn get_base_of_data(&self) -> u64 {
        self.optional_header.image_base
    }

    fn get_image_base(&self) -> u64 {
        self.optional_header.image_base
    }

    fn get_section_alignment(&self) -> u32 {
        self.optional_header.section_alignment
    }

    fn get_file_alignment(&self) -> u32 {
        self.optional_header.file_alignment
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
        format!("0x{:X}", self.optional_header.dll_characteristics)
    }

    fn get_dll_characteristics_hover(&self) -> String {
        get_dll_characteristics_description(self.optional_header.dll_characteristics)
    }

    fn get_size_of_stack_reserve(&self) -> u64 {
        self.optional_header.size_of_stack_reserve
    }

    fn get_size_of_stack_commit(&self) -> u64 {
        self.optional_header.size_of_stack_commit
    }

    fn get_size_of_heap_reserve(&self) -> u64 {
        self.optional_header.size_of_heap_reserve
    }

    fn get_size_of_heap_commit(&self) -> u64 {
        self.optional_header.size_of_heap_commit
    }

    fn get_loader_flags(&self) -> u32 {
        self.optional_header.loader_flags
    }

    fn get_number_of_rva_and_sizes(&self) -> u32 {
        self.optional_header.number_of_rva_and_sizes
    }
    fn serde_serialize(&self) -> SerializableNtHeaders {
        SerializableNtHeaders::ImageNtHeaders64(self.clone())
    }
}

pub(crate) fn read_nt_head<T>(
    file: &mut File,
    start_addr: u16,
) -> anyhow::Result<(T, DataDirectory)>
where
    T: NtHeaders + Default,
    [(); size_of::<T>()]:,
{
    let mut nt_head: T = Default::default();
    let mut data_dictionary = DataDirectory(Vec::new());
    file.seek(SeekFrom::Start(start_addr as u64))?;
    unsafe {
        let reads: &mut [u8; size_of::<T>()] = transmute(&mut nt_head);
        file.read(reads)?;
    }
    for _ in 0..nt_head.num_of_rva() {
        let mut image_data: ImageDataDirectory = Default::default();
        unsafe {
            let reads: &mut [u8; 8] = transmute(&mut image_data);
            file.read(reads)?;
        }
        data_dictionary.add(image_data);
    }

    Ok((nt_head, data_dictionary))
}
