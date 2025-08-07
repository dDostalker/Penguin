use crate::tools_api::read_file::nt_header::traits::NtHeaders;
use std::sync::Arc;
use std::cell::RefCell;

mod dos_header;
mod dos_stub;
mod export;
mod import;
pub mod nt_header;
mod section_headers;

#[repr(C)]
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct ImageDosHeader {
    pub(crate) e_magic: u16,      // MZ标记 0x5A4D
    pub(crate) e_cblp: u16,       // 最后(部分)页中的字节数
    pub(crate) e_cp: u16,         // 文件中的全部和部分页数
    pub(crate) e_crlc: u16,       // 重定位表中的指针数
    pub(crate) e_cparhdr: u16,    // 头部尺寸以段落为单位
    pub(crate) e_minalloc: u16,   // 所需的最小附加段
    pub(crate) e_maxalloc: u16,   // 所需的最大附加段
    pub(crate) e_ss: u16,         // 初始的SS值(相对偏移量)
    pub(crate) e_sp: u16,         // 初始的SP值
    pub(crate) e_csum: u16,       // 补码校验值
    pub(crate) e_ip: u16,         // 初始的IP值
    pub(crate) e_cs: u16,         // 初始的SS值
    pub(crate) e_lfarlc: u16,     // 重定位表的字节偏移量
    pub(crate) e_ovno: u16,       // 覆盖号
    pub(crate) e_res: [u16; 4],   // 保留字
    pub(crate) e_oemid: u16,      // OEM标识符(相对m_oeminfo)
    pub(crate) e_oeminfo: u16,    // OEM信息
    pub(crate) e_res2: [u16; 10], // 保留字
    pub(crate) e_lfanew: u16,     // NT头相对于文件的偏移地址
}

/// 存根内容
#[derive(Debug, Eq, PartialEq)]
pub struct ImageDosStub {
    pub buffer: Vec<u8>,
}

/// image_file_header 位于nt头中
#[repr(C)]
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub(crate) struct ImageFileHeader {
    pub(crate) machine: u16,                 //标记可以程序可以运行在什么样的CPU上
    pub(crate) number_of_sections: u16,      //节区的数量
    pub(crate) time_date_stamp: u32,         //时间戳，可以更改
    pub(crate) pointer_to_symbol_table: u32, //符号表的偏移量，与debug有关，没有则为零
    pub(crate) number_of_symbols: u32,       //符号表中的符号数
    pub(crate) size_of_optional_header: u16, //记录MAGE_OPTIONAL_HEADER的大小
    pub(crate) characteristics: u16,
}
#[repr(C)]
#[derive(Default, Clone, Debug)]
struct MageDataDirectory {
    virtual_address: u32,
    size: u32,
}
#[repr(C)]
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct ImageOptionalHeader64 {
    pub(crate) magic: u16, // 标识PE文件的魔数，例如0x20B表示64位PE文件
    pub(crate) major_linker_version: u8, // 链接器主要版本号
    pub(crate) long_minor_linker_version: u8, // 链接器次要版本号
    /// 所有代码段的总大小
    pub size_of_code: u32,
    /// 所有已初始化数据的总大小
    pub size_of_initialized_data: u32,
    /// 所有未初始化数据（BSS段）的总大小
    pub size_of_uninitialized_data: u32,
    /// 程序入口点的RVA（相对虚拟地址）
    pub address_of_entry_point: u32,
    /// 代码段的起始RVA
    pub base_of_code: u32,
    /// 程序的首选加载基地址
    pub image_base: u64,
    /// 内存中节的对齐方式（字节）
    pub section_alignment: u32,
    /// 文件中节的对齐方式（字节）
    pub file_alignment: u32,
    /// 程序所需的最低操作系统主版本号
    pub major_operating_system_version: u16,
    /// 程序所需的最低操作系统次版本号
    pub minor_operating_system_version: u16,
    /// 程序的主版本号
    pub major_image_version: u16,
    /// 程序的次版本号
    pub minor_image_version: u16,
    /// 程序所需的最低子系统主版本号
    pub major_subsystem_version: u16,
    /// 程序所需的最低子系统次版本号
    pub minor_subsystem_version: u16,
    /// 保留字段（通常设置为0）
    pub win32_version_value: u32,
    /// 整个PE映像在内存中的大小（字节）
    pub size_of_image: u32,
    /// PE文件头和节表的总大小（字节）
    pub size_of_headers: u32,
    /// PE文件的校验和
    pub checksum: u32,
    /// 指示程序运行的子系统
    pub subsystem: u16,
    /// DLL的特性标志
    pub dll_characteristics: u16,
    /// 为线程栈保留的空间大小（字节）
    pub size_of_stack_reserve: u64,
    /// 初始提交到线程栈的空间大小（字节）
    pub size_of_stack_commit: u64,
    /// 为堆保留的空间大小（字节）
    pub size_of_heap_reserve: u64,
    /// 初始提交到堆的空间大小（字节）
    pub size_of_heap_commit: u64,
    /// 加载器标志（通常设置为0）
    pub loader_flags: u32,
    /// 数据目录的数量
    pub number_of_rva_and_sizes: u32,
}

#[repr(C)]
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ImageOptionalHeader {
    // 标准字段
    pub magic: u16,                      // 标识PE文件魔数（0x10B=32位, 0x20B=64位）
    pub major_linker_version: u8,        // 链接器主要版本号
    pub minor_linker_version: u8,        // 链接器次要版本号
    pub size_of_code: u32,               // 所有代码段总大小
    pub size_of_initialized_data: u32,   // 已初始化数据总大小
    pub size_of_uninitialized_data: u32, // 未初始化数据(BSS)总大小
    pub address_of_entry_point: u32,     // 程序入口点RVA
    pub base_of_code: u32,               // 代码段起始RVA
    pub base_of_data: u32,               // 数据段起始RVA（32位PE特有）

    // NT 额外字段
    pub image_base: u32,                     // 首选加载基地址
    pub section_alignment: u32,              // 内存中节对齐方式
    pub file_alignment: u32,                 // 文件中节对齐方式
    pub major_operating_system_version: u16, // 最低OS主版本
    pub minor_operating_system_version: u16, // 最低OS次版本
    pub major_image_version: u16,            // 程序主版本号
    pub minor_image_version: u16,            // 程序次版本号
    pub major_subsystem_version: u16,        // 最低子系统主版本
    pub minor_subsystem_version: u16,        // 最低子系统次版本
    pub win32version_value: u32,             // 保留字段（通常为0）
    pub size_of_image: u32,                  // 内存中整个PE映像大小
    pub size_of_headers: u32,                // PE头+节表总大小
    pub check_sum: u32,                      // 文件校验和
    pub subsystem: u16,                      // 运行子系统类型（GUI/CUI等）
    pub dll_characteristics: u16,            // DLL特性标志
    pub size_of_stack_reserve: u32,          // 线程栈保留空间
    pub size_of_stack_commit: u32,           // 初始提交栈大小
    pub size_of_heap_reserve: u32,           // 堆保留空间
    pub size_of_heap_commit: u32,            // 初始提交堆大小
    pub loader_flags: u32,                   // 加载器标志（通常为0）
    pub number_of_rva_and_sizes: u32,        // 数据目录数量
}
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub(crate) struct ImageDataDirectory {
    pub(crate) virtual_address: u32,
    pub(crate) size: u32,
}
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct DataDirectory(Vec<ImageDataDirectory>);

#[repr(C)]
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct ImageNtHeaders64 {
    pub(crate) signature: u32,
    pub(crate) file_header: ImageFileHeader,
    pub(crate) optional_header: ImageOptionalHeader64,
}
#[repr(C)]
#[derive(Default, Clone, Debug, Eq, PartialEq)]
pub struct ImageNtHeaders {
    pub(crate) signature: u32,
    pub(crate) file_header: ImageFileHeader,
    pub(crate) optional_header: ImageOptionalHeader,
}
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct ImageSectionHeaders(pub(crate) Vec<ImageSectionHeader>);

/// 节头结构体 - 表示PE文件中每个节的元数据
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ImageSectionHeader {
    /// 节名称（最多8个字符）
    pub name: [u8; 8],
    /// 联合体字段（文件地址或虚拟大小）
    pub misc: SectionHeaderMisc,
    /// 当前节在内存中的偏移地址（RVA）
    pub virtual_address: u32,
    /// 当前节在文件中对齐后的大小
    pub size_of_raw_data: u32,
    /// 当前节在文件中的偏移地址
    pub pointer_to_raw_data: u32,
    /// 重定位条目开头的文件指针（通常为0）
    pub pointer_to_relocations: u32,
    /// 行号条目开头的文件指针（通常为0）
    pub pointer_to_linenumbers: u32,
    /// 重定位条目数（可执行文件中通常为0）
    pub number_of_relocations: u16,
    /// 行号条目数
    pub number_of_linenumbers: u16,
    /// 节的特征标志（可执行、可读、可写等）
    pub characteristics: u32,
}

/// 处理节头的联合体字段
#[repr(C)]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SectionHeaderMisc {
    /// 当前节在内存中未对齐时的大小（真实大小）
    pub virtual_size: u32,
}
#[derive(Debug)]
pub(crate) struct SectionData {
    f_address: u32,
    f_size: u32,
    data: Vec<u8>,
}

/// Rva转化文件地址
pub async fn rva_2_fo<T>(nt_head: &T, section_heads: &ImageSectionHeaders, rva: u32) -> Option<u32>
where
    T: NtHeaders + ?Sized,
{
    for i in 0..nt_head.section_number() {
        let start = section_heads.get_section_virtual_address(i as usize);
        let end = section_heads.get_virtual_rva_end(i as usize);
        if rva >= start && rva < end {
            return Some(section_heads.get_section_pointer_to_raw_data(i as usize) + (rva - start));
        }
    }
    None
}

/// export dir表
#[repr(C)]
#[derive(Default, Debug, Eq, PartialEq)]
pub struct ExportDir {
    pub(crate) characteristics: u32,
    pub(crate) time_data_stamp: u32,
    pub(crate) major_vision: u16,
    pub(crate) minor_version: u16,
    pub(crate) name: u32,
    pub(crate) base: u32,
    pub(crate) number_of_func: u32,
    pub(crate) number_of_names: u32,
    /// rva addrees vec
    pub(crate) address_of_functions: u32,
    pub(crate) address_of_names: u32,
    pub(crate) address_of_name_ordinals: u32,
}
/// ExportInfo 用于转递给egui
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ExportInfo {
    pub name_rva: u32,
    pub name_string_fo: u32,
    pub name_max_length: u32,
    pub name: String,
    pub function_address: u32,
    pub function: u32,
    pub ordinals_address: u32,
    pub ordinals: u16,
}
/// ExportInfos 用于传递egui
#[derive(Debug, Clone, Default, Eq, PartialEq)]
pub struct ExportTable(pub(crate) Arc<RefCell<Vec<ExportInfo>>>);

#[repr(C)]
#[derive(Default, Debug)]
pub struct ImportDescriptor {
    pub(crate) dummy_union_name: u32,
    time_date_stamp: u32,
    forwarder_chain: u32,
    pub(crate) name_address: u32,
    first_thunk: u32,
}
/// import dll 用于传递egui
#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct ImportDll {
    pub(crate) name_address: u32,
    pub(crate) name_length: u32,
    pub(crate) name: String,
    pub(crate) time_date_stamp: u32,
    pub(crate) forwarder_chain: u32,
    pub(crate) first_thunk: u32,
    pub(crate) function_info: Vec<ImportFunction>,
    pub(crate) function_size: u32,
}
#[derive(Default, Eq, PartialEq)]
pub struct ImportTable(pub(crate) Arc<RefCell<Vec<ImportDll>>>);

/// import function 用于传递egui
#[derive(Default, Debug, Eq, PartialEq, Clone)]
pub struct ImportFunction {
    pub(crate) name_address: u32,
    pub(crate) name_length: u32,
    pub(crate) name_max_length: u32,
    pub(crate) name: String,
}
