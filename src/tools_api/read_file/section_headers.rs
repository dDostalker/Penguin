use crate::tools_api::read_file::{ImageSectionHeader, ImageSectionHeaders, SectionData};
use std::fs::File;
use std::io::SeekFrom;
use std::io::{Read, Seek};
use std::mem::transmute;

#[repr(u32)]
pub enum SectionCharacteristics {
    // 保留的标志
    RESERVED0 = 0x00000000,
    RESERVED1 = 0x00000001,
    RESERVED2 = 0x00000002,
    RESERVED4 = 0x00000004,
    RESERVED10 = 0x00000010,
    RESERVED400 = 0x00000400,
    RESERVED2000 = 0x00002000,
    RESERVED10000 = 0x00010000,
    RESERVED20000 = 0x00020000,
    RESERVED40000 = 0x00040000,
    RESERVED80000 = 0x00080000,

    // 节类型标志
    ImageScnTypeNoPad = 0x00000008,
    ImageScnCntCode = 0x00000020,
    ImageScnCntInitializedData = 0x00000040,
    ImageScnCntUninitializedData = 0x00000080,
    ImageScnLnkOther = 0x00000100,
    ImageScnLnkInfo = 0x00000200,
    ImageScnLnkRemove = 0x00000800,
    ImageScnLnkComdat = 0x00001000,

    // 特殊标志
    ImageScnNoDeferSpecExc = 0x00004000,
    ImageScnGprel = 0x00008000,

    // 对齐标志
    ImageScnAlign1Bytes = 0x00100000,
    ImageScnAlign2Bytes = 0x00200000,
    ImageScnAlign4Bytes = 0x00300000,
    ImageScnAlign8Bytes = 0x00400000,
    ImageScnAlign16Bytes = 0x00500000,
    ImageScnAlign32Bytes = 0x00600000,
    ImageScnAlign64Bytes = 0x00700000,
    ImageScnAlign128Bytes = 0x00800000,
    ImageScnAlign256Bytes = 0x00900000,
    ImageScnAlign512Bytes = 0x00A00000,
    ImageScnAlign1024Bytes = 0x00B00000,
    ImageScnAlign2048Bytes = 0x00C00000,
    ImageScnAlign4096Bytes = 0x00D00000,
    ImageScnAlign8192Bytes = 0x00E00000,

    // 其他标志
    ImageScnLnkNrelocOvfl = 0x01000000,
    ImageScnMemDiscardable = 0x02000000,
    ImageScnMemNotCached = 0x04000000,
    ImageScnMemNotPaged = 0x08000000,
    ImageScnMemShared = 0x10000000,
    ImageScnMemExecute = 0x20000000,
    ImageScnMemRead = 0x40000000,
    ImageScnMemWrite = 0x80000000,
}

// /// 获取标志的描述信息
// pub fn section_description(section_characteristics: u32) -> String {
//     const SECTION_ENUM_DESCRIPTIONS: &[(u32, &str)] = &[
//         (
//             SectionCharacteristics::RESERVED0 as u32,
//             i18n::SECTION_RESERVED,
//         ),
//         (
//             SectionCharacteristics::RESERVED1 as u32,
//             i18n::SECTION_RESERVED,
//         ),
//         (
//             SectionCharacteristics::RESERVED2 as u32,
//             i18n::SECTION_RESERVED,
//         ),
//         (
//             SectionCharacteristics::RESERVED4 as u32,
//             i18n::SECTION_RESERVED,
//         ),
//         (
//             SectionCharacteristics::RESERVED10 as u32,
//             i18n::SECTION_RESERVED,
//         ),
//         (
//             SectionCharacteristics::RESERVED400 as u32,
//             i18n::SECTION_RESERVED,
//         ),
//         (
//             SectionCharacteristics::RESERVED2000 as u32,
//             i18n::SECTION_RESERVED,
//         ),
//         (
//             SectionCharacteristics::RESERVED10000 as u32,
//             i18n::SECTION_RESERVED,
//         ),
//         (
//             SectionCharacteristics::RESERVED20000 as u32,
//             i18n::SECTION_RESERVED,
//         ),
//         (
//             SectionCharacteristics::RESERVED40000 as u32,
//             i18n::SECTION_RESERVED,
//         ),
//         (
//             SectionCharacteristics::RESERVED80000 as u32,
//             i18n::SECTION_RESERVED,
//         ),
//         (
//             SectionCharacteristics::ImageScnTypeNoPad as u32,
//             i18n::SECTION_NO_PAD,
//         ),
//         (
//             SectionCharacteristics::ImageScnCntCode as u32,
//             i18n::SECTION_CONTAINS_CODE,
//         ),
//         (
//             SectionCharacteristics::ImageScnCntInitializedData as u32,
//             i18n::SECTION_CONTAINS_INITIALIZED_DATA,
//         ),
//         (
//             SectionCharacteristics::ImageScnCntUninitializedData as u32,
//             i18n::SECTION_CONTAINS_UNINITIALIZED_DATA,
//         ),
//         (
//             SectionCharacteristics::ImageScnLnkOther as u32,
//             i18n::SECTION_OTHER,
//         ),
//         (
//             SectionCharacteristics::ImageScnLnkInfo as u32,
//             i18n::SECTION_INFO,
//         ),
//         (
//             SectionCharacteristics::ImageScnLnkRemove as u32,
//             i18n::SECTION_REMOVE,
//         ),
//         (
//             SectionCharacteristics::ImageScnLnkComdat as u32,
//             i18n::SECTION_COMDAT,
//         ),
//         (
//             SectionCharacteristics::ImageScnNoDeferSpecExc as u32,
//             i18n::SECTION_NO_DEFER_SPEC_EXC,
//         ),
//         (
//             SectionCharacteristics::ImageScnGprel as u32,
//             i18n::SECTION_GPREL,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign1Bytes as u32,
//             i18n::SECTION_ALIGN_1BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign2Bytes as u32,
//             i18n::SECTION_ALIGN_2BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign4Bytes as u32,
//             i18n::SECTION_ALIGN_4BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign8Bytes as u32,
//             i18n::SECTION_ALIGN_8BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign16Bytes as u32,
//             i18n::SECTION_ALIGN_16BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign32Bytes as u32,
//             i18n::SECTION_ALIGN_32BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign64Bytes as u32,
//             i18n::SECTION_ALIGN_64BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign128Bytes as u32,
//             i18n::SECTION_ALIGN_128BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign256Bytes as u32,
//             i18n::SECTION_ALIGN_256BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign512Bytes as u32,
//             i18n::SECTION_ALIGN_512BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign1024Bytes as u32,
//             i18n::SECTION_ALIGN_1024BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign2048Bytes as u32,
//             i18n::SECTION_ALIGN_2048BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign4096Bytes as u32,
//             i18n::SECTION_ALIGN_4096BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnAlign8192Bytes as u32,
//             i18n::SECTION_ALIGN_8192BYTES,
//         ),
//         (
//             SectionCharacteristics::ImageScnLnkNrelocOvfl as u32,
//             i18n::SECTION_RELOC_OVFL,
//         ),
//         (
//             SectionCharacteristics::ImageScnMemDiscardable as u32,
//             i18n::SECTION_MEM_DISCARDABLE,
//         ),
//         (
//             SectionCharacteristics::ImageScnMemNotCached as u32,
//             i18n::SECTION_MEM_NOT_CACHED,
//         ),
//         (
//             SectionCharacteristics::ImageScnMemNotPaged as u32,
//             i18n::SECTION_MEM_NOT_PAGED,
//         ),
//         (
//             SectionCharacteristics::ImageScnMemShared as u32,
//             i18n::SECTION_MEM_SHARED,
//         ),
//         (
//             SectionCharacteristics::ImageScnMemExecute as u32,
//             i18n::SECTION_MEM_EXECUTE,
//         ),
//         (
//             SectionCharacteristics::ImageScnMemRead as u32,
//             i18n::SECTION_MEM_READ,
//         ),
//         (
//             SectionCharacteristics::ImageScnMemWrite as u32,
//             i18n::SECTION_MEM_WRITE,
//         ),
//     ];
//     SECTION_ENUM_DESCRIPTIONS
//         .iter()
//         .filter_map(|(flag, description)| {
//             if section_characteristics & flag != 0 {
//                 Some(*description)
//             } else {
//                 None
//             }
//         })
//         .collect::<Vec<_>>()
//         .join("\n")
// }

impl ImageSectionHeader {
    pub(crate) fn new(file: &mut File) -> anyhow::Result<ImageSectionHeader> {
        let mut section_header: ImageSectionHeader = Default::default();
        unsafe {
            let f: &mut [u8; size_of::<ImageSectionHeader>()] = transmute(&mut section_header);
            file.read(f)?;
        }
        Ok(section_header)
    }
}

impl ImageSectionHeaders {
    pub fn new(
        file: &mut File,
        section_addr: u32,
        section_num: u16,
    ) -> anyhow::Result<ImageSectionHeaders> {
        file.seek(SeekFrom::Start(section_addr as u64))?;
        let mut section_headers: ImageSectionHeaders = Default::default();
        for _ in 0..section_num {
            section_headers.add(ImageSectionHeader::new(file)?);
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
    // pub(crate) fn get_section_characteristics_hover(&self, index: usize) -> String {
    //     section_description(self.0.get(index).unwrap().characteristics)
    // }
    pub fn get_section_pointer_to_relocations(&self, index: usize) -> u32 {
        self.0.get(index).unwrap().pointer_to_relocations
    }
}

impl SectionData {
    pub fn new(
        file: &mut File,
        point_to_raw_data: u32,
        size_of_raw_data: u32,
    ) -> anyhow::Result<Box<SectionData>> {
        file.seek(SeekFrom::Start(point_to_raw_data as u64))?;
        let mut section_data: Box<SectionData> = Box::new(SectionData {
            f_address: point_to_raw_data,
            f_size: size_of_raw_data,
            data: vec![0u8; size_of_raw_data as usize],
        });
        file.read(&mut section_data.data)?;
        Ok(section_data)
    }
}
