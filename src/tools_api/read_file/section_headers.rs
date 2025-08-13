use crate::i18n;
use crate::tools_api::read_file::{ImageSectionHeader, ImageSectionHeaders, SectionData};
use std::io::SeekFrom;
use std::mem::transmute;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

#[repr(u32)]
pub enum SectionCharacteristics {
    // 保留的标志
    RESERVED_0 = 0x00000000,
    RESERVED_1 = 0x00000001,
    RESERVED_2 = 0x00000002,
    RESERVED_4 = 0x00000004,
    RESERVED_10 = 0x00000010,
    RESERVED_400 = 0x00000400,
    RESERVED_2000 = 0x00002000,
    RESERVED_10000 = 0x00010000,
    RESERVED_20000 = 0x00020000,
    RESERVED_40000 = 0x00040000,
    RESERVED_80000 = 0x00080000,

    // 节类型标志
    IMAGE_SCN_TYPE_NO_PAD = 0x00000008,
    IMAGE_SCN_CNT_CODE = 0x00000020,
    IMAGE_SCN_CNT_INITIALIZED_DATA = 0x00000040,
    IMAGE_SCN_CNT_UNINITIALIZED_DATA = 0x00000080,
    IMAGE_SCN_LNK_OTHER = 0x00000100,
    IMAGE_SCN_LNK_INFO = 0x00000200,
    IMAGE_SCN_LNK_REMOVE = 0x00000800,
    IMAGE_SCN_LNK_COMDAT = 0x00001000,

    // 特殊标志
    IMAGE_SCN_NO_DEFER_SPEC_EXC = 0x00004000,
    IMAGE_SCN_GPREL = 0x00008000,

    // 对齐标志
    IMAGE_SCN_ALIGN_1BYTES = 0x00100000,
    IMAGE_SCN_ALIGN_2BYTES = 0x00200000,
    IMAGE_SCN_ALIGN_4BYTES = 0x00300000,
    IMAGE_SCN_ALIGN_8BYTES = 0x00400000,
    IMAGE_SCN_ALIGN_16BYTES = 0x00500000,
    IMAGE_SCN_ALIGN_32BYTES = 0x00600000,
    IMAGE_SCN_ALIGN_64BYTES = 0x00700000,
    IMAGE_SCN_ALIGN_128BYTES = 0x00800000,
    IMAGE_SCN_ALIGN_256BYTES = 0x00900000,
    IMAGE_SCN_ALIGN_512BYTES = 0x00A00000,
    IMAGE_SCN_ALIGN_1024BYTES = 0x00B00000,
    IMAGE_SCN_ALIGN_2048BYTES = 0x00C00000,
    IMAGE_SCN_ALIGN_4096BYTES = 0x00D00000,
    IMAGE_SCN_ALIGN_8192BYTES = 0x00E00000,

    // 其他标志
    IMAGE_SCN_LNK_NRELOC_OVFL = 0x01000000,
    IMAGE_SCN_MEM_DISCARDABLE = 0x02000000,
    IMAGE_SCN_MEM_NOT_CACHED = 0x04000000,
    IMAGE_SCN_MEM_NOT_PAGED = 0x08000000,
    IMAGE_SCN_MEM_SHARED = 0x10000000,
    IMAGE_SCN_MEM_EXECUTE = 0x20000000,
    IMAGE_SCN_MEM_READ = 0x40000000,
    IMAGE_SCN_MEM_WRITE = 0x80000000,
}

/// 获取标志的描述信息
pub fn section_description(section_characteristics: u32) -> String {
    const SECTION_ENUM_DESCRIPTIONS: &[(u32, &str)] = &[
        (
            SectionCharacteristics::RESERVED_0 as u32,
            i18n::SECTION_RESERVED,
        ),
        (
            SectionCharacteristics::RESERVED_1 as u32,
            i18n::SECTION_RESERVED,
        ),
        (
            SectionCharacteristics::RESERVED_2 as u32,
            i18n::SECTION_RESERVED,
        ),
        (
            SectionCharacteristics::RESERVED_4 as u32,
            i18n::SECTION_RESERVED,
        ),
        (
            SectionCharacteristics::RESERVED_10 as u32,
            i18n::SECTION_RESERVED,
        ),
        (
            SectionCharacteristics::RESERVED_400 as u32,
            i18n::SECTION_RESERVED,
        ),
        (
            SectionCharacteristics::RESERVED_2000 as u32,
            i18n::SECTION_RESERVED,
        ),
        (
            SectionCharacteristics::RESERVED_10000 as u32,
            i18n::SECTION_RESERVED,
        ),
        (
            SectionCharacteristics::RESERVED_20000 as u32,
            i18n::SECTION_RESERVED,
        ),
        (
            SectionCharacteristics::RESERVED_40000 as u32,
            i18n::SECTION_RESERVED,
        ),
        (
            SectionCharacteristics::RESERVED_80000 as u32,
            i18n::SECTION_RESERVED,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_TYPE_NO_PAD as u32,
            i18n::SECTION_NO_PAD,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_CNT_CODE as u32,
            i18n::SECTION_CONTAINS_CODE,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_CNT_INITIALIZED_DATA as u32,
            i18n::SECTION_CONTAINS_INITIALIZED_DATA,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_CNT_UNINITIALIZED_DATA as u32,
            i18n::SECTION_CONTAINS_UNINITIALIZED_DATA,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_LNK_OTHER as u32,
            i18n::SECTION_OTHER,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_LNK_INFO as u32,
            i18n::SECTION_INFO,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_LNK_REMOVE as u32,
            i18n::SECTION_REMOVE,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_LNK_COMDAT as u32,
            i18n::SECTION_COMDAT,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_NO_DEFER_SPEC_EXC as u32,
            i18n::SECTION_NO_DEFER_SPEC_EXC,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_GPREL as u32,
            i18n::SECTION_GPREL,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_1BYTES as u32,
            i18n::SECTION_ALIGN_1BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_2BYTES as u32,
            i18n::SECTION_ALIGN_2BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_4BYTES as u32,
            i18n::SECTION_ALIGN_4BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_8BYTES as u32,
            i18n::SECTION_ALIGN_8BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_16BYTES as u32,
            i18n::SECTION_ALIGN_16BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_32BYTES as u32,
            i18n::SECTION_ALIGN_32BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_64BYTES as u32,
            i18n::SECTION_ALIGN_64BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_128BYTES as u32,
            i18n::SECTION_ALIGN_128BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_256BYTES as u32,
            i18n::SECTION_ALIGN_256BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_512BYTES as u32,
            i18n::SECTION_ALIGN_512BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_1024BYTES as u32,
            i18n::SECTION_ALIGN_1024BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_2048BYTES as u32,
            i18n::SECTION_ALIGN_2048BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_4096BYTES as u32,
            i18n::SECTION_ALIGN_4096BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_ALIGN_8192BYTES as u32,
            i18n::SECTION_ALIGN_8192BYTES,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_LNK_NRELOC_OVFL as u32,
            i18n::SECTION_RELOC_OVFL,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_MEM_DISCARDABLE as u32,
            i18n::SECTION_MEM_DISCARDABLE,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_MEM_NOT_CACHED as u32,
            i18n::SECTION_MEM_NOT_CACHED,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_MEM_NOT_PAGED as u32,
            i18n::SECTION_MEM_NOT_PAGED,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_MEM_SHARED as u32,
            i18n::SECTION_MEM_SHARED,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_MEM_EXECUTE as u32,
            i18n::SECTION_MEM_EXECUTE,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_MEM_READ as u32,
            i18n::SECTION_MEM_READ,
        ),
        (
            SectionCharacteristics::IMAGE_SCN_MEM_WRITE as u32,
            i18n::SECTION_MEM_WRITE,
        ),
    ];
    SECTION_ENUM_DESCRIPTIONS
        .iter()
        .filter_map(|(flag, description)| {
            if section_characteristics & flag != 0 {
                Some(*description)
            } else {
                None
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

impl ImageSectionHeader {
    pub(crate) async fn new(file: &mut File) -> anyhow::Result<ImageSectionHeader> {
        let mut section_header: ImageSectionHeader = Default::default();
        unsafe {
            let f: &mut [u8; size_of::<ImageSectionHeader>()] = transmute(&mut section_header);
            file.read(f).await?;
        }
        Ok(section_header)
    }
}

impl ImageSectionHeaders {
    pub async fn new(
        file: &mut File,
        section_addr: u32,
        section_num: u16,
    ) -> anyhow::Result<ImageSectionHeaders> {
        file.seek(SeekFrom::Start(section_addr as u64)).await?;
        let mut section_headers: ImageSectionHeaders = Default::default();
        for _ in 0..section_num {
            section_headers.add(ImageSectionHeader::new(file).await?);
        }
        Ok(section_headers)
    }
    pub fn add(&mut self, section_header: ImageSectionHeader) {
        self.0.push(section_header);
    }
    pub fn get_num(&self) -> anyhow::Result<usize> {
        Ok(self.0.len())
    }
    pub fn get_virtual_rva_end(&self, index: usize) -> u32 {
        self.0[index].virtual_address + self.0[index].size_of_raw_data
    }
    // 制取方法
    pub(crate) fn get_section_name(&self, index: usize) -> anyhow::Result<String> {
        let section_name = self.0.get(index).unwrap().name;
        let section_name = String::from_utf8(section_name.to_vec())?;
        Ok(section_name)
    }
    ///文件地址或当前节在内存中未对齐时的大小，即真实大小
    pub fn get_section_misc(&self, index: usize) -> anyhow::Result<u32> {
        Ok(self.0.get(index).unwrap().misc.virtual_size)
    }

    ///当前节在文件中对齐后大小
    pub fn get_section_size_of_raw_data(&self, index: usize) -> anyhow::Result<u32> {
        Ok(self.0.get(index).unwrap().size_of_raw_data)
    }
    ///当前节在文件中的偏移地址
    pub fn get_section_pointer_to_raw_data(&self, index: usize) -> u32 {
        self.0.get(index).unwrap().pointer_to_raw_data
    }
    ///当前节在内存中的偏移地址
    pub fn get_section_virtual_address(&self, index: usize) -> u32 {
        self.0[index].virtual_address
    }
    pub(crate) fn get_section_number_of_linenumbers(&self, index: usize) -> u16 {
        self.0.get(index).unwrap().number_of_linenumbers
    }
    pub(crate) fn get_section_number_of_relocations(&self, index: usize) -> u16 {
        self.0.get(index).unwrap().number_of_relocations
    }
    pub(crate) fn get_section_pointer_to_linenumbers(&self, index: usize) -> u32 {
        self.0.get(index).unwrap().pointer_to_linenumbers
    }
    pub(crate) fn get_section_characteristics(&self, index: usize) -> u32 {
        self.0.get(index).unwrap().characteristics
    }
    pub(crate) fn get_section_characteristics_hover(&self, index: usize) -> String {
        section_description(self.0.get(index).unwrap().characteristics as u32)
    }
    pub fn get_section_pointer_to_relocations(&self, index: usize) -> u32 {
        self.0.get(index).unwrap().pointer_to_relocations
    }
}

impl SectionData {
    pub async fn new(
        file: &mut File,
        point_to_raw_data: u32,
        size_of_raw_data: u32,
    ) -> anyhow::Result<Box<SectionData>> {
        file.seek(SeekFrom::Start(point_to_raw_data as u64)).await?;
        let mut section_data: Box<SectionData> = Box::new(SectionData {
            f_address: point_to_raw_data,
            f_size: size_of_raw_data,
            data: vec![0u8; size_of_raw_data as usize],
        });
        file.read(&mut section_data.data).await?;
        Ok(section_data)
    }
}
