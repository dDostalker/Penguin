use crate::tools_api::read_file::{ImageSectionHeader, ImageSectionHeaders, SectionData};
use std::io::SeekFrom;
use std::mem::transmute;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

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
