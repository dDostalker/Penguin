use crate::tools_api::read_file::nt_header::traits::NtHeaders;
use crate::tools_api::read_file::{
    DataDirectory, ExportDir, ExportInfo, ExportTable, ImageSectionHeaders, rva_2_fo,
};
use byteorder::{LittleEndian, ReadBytesExt};
use std::cell::RefCell;
use std::fs::File;
use std::io::SeekFrom;
use std::io::{Read, Seek};
use std::mem::{MaybeUninit, size_of};
use std::rc::Rc;

impl ExportDir {
    /// 读取导出表信息
    pub fn new<T>(
        file: &mut File,
        nt_head: &T,
        image_section_headers: &ImageSectionHeaders,
        data_dir: &DataDirectory,
    ) -> anyhow::Result<Option<ExportDir>>
    where
        T: NtHeaders + ?Sized,
    {
        let mut export_dir: ExportDir = Default::default();
        if let Some(fo) = rva_2_fo(
            nt_head,
            image_section_headers,
            data_dir.get_export_directory_address()?,
        ) {
            file.seek(SeekFrom::Start(fo as u64))?;
            export_dir = unsafe {
                let mut export_dir = MaybeUninit::<ExportDir>::uninit();
                let bytes = std::slice::from_raw_parts_mut(
                    export_dir.as_mut_ptr() as *mut u8,
                    size_of::<ExportDir>(),
                );
                file.read_exact(bytes)?;
                export_dir.assume_init()
            };
        }
        if export_dir.name == 0 {
            return Ok(None);
        }

        Ok(Some(export_dir))
    }
}
impl ExportInfo {
    /// 怎里得到导出表信息，便于传递egui
    pub fn new<T>(
        name_file_offset: u32,
        function_array_file_offset: u32,
        ordinals_array_file_offset: u32,
        file: &mut File,
        nt_head: &T,
        section_headers: &ImageSectionHeaders,
    ) -> anyhow::Result<Option<ExportInfo>>
    where
        T: NtHeaders + ?Sized,
    {
        file.seek(SeekFrom::Start(name_file_offset as _))?;
        let name_string_rva =
            rva_2_fo(nt_head, section_headers, file.read_u32::<LittleEndian>()?).unwrap();
        file.seek(SeekFrom::Start(name_string_rva as u64))?;
        let mut buf = [0; 512];
        file.read(&mut buf)?;
        let mut flag = 1;
        let name_length = buf.iter().position(|&x| x == 0).unwrap_or(0);
        let name_max_length = buf
            .iter()
            .position(|&x| {
                if x != 0 && flag == 0 {
                    return true;
                } else {
                    flag = x;
                }
                false
            })
            .unwrap_or(0) as u32;
        let name = String::from_utf8_lossy(&buf[0..name_length]).to_string();
        file.seek(SeekFrom::Start(function_array_file_offset as _))?;
        let function = file.read_u32::<LittleEndian>()?;
        file.seek(SeekFrom::Start(ordinals_array_file_offset as u64))?;
        let ordinals = file.read_u16::<LittleEndian>()?;
        Ok(Some(ExportInfo {
            name_rva: name_file_offset,
            name_string_fo: name_string_rva,
            name_max_length,
            name,
            function_address: function_array_file_offset,
            function,
            ordinals_address: ordinals_array_file_offset,
            ordinals,
        }))
    }
}
impl ExportTable {
    /// 怎里得到导出表信息，便于传递egui
    pub fn new<T>(
        f: &mut File,
        nt_head: &T,
        image_section_headers: &ImageSectionHeaders,
        export_dir: &ExportDir,
    ) -> anyhow::Result<ExportTable>
    where
        T: NtHeaders + ?Sized,
    {
        let mut export_infos = Vec::<ExportInfo>::new();
        let mut name_array_address =
            rva_2_fo(nt_head, image_section_headers, export_dir.address_of_names).unwrap();
        let mut function_array_address = rva_2_fo(
            nt_head,
            image_section_headers,
            export_dir.address_of_functions,
        )
        .unwrap();
        let mut ordinals_array_address = rva_2_fo(
            nt_head,
            image_section_headers,
            export_dir.address_of_name_ordinals,
        )
        .unwrap();

        for _ in 0..export_dir.number_of_names {
            if let Some(export_info) = ExportInfo::new(
                name_array_address,
                function_array_address,
                ordinals_array_address,
                f,
                nt_head,
                image_section_headers,
            )? {
                export_infos.push(export_info);
                name_array_address += 4;
                function_array_address += 4;
                // todo 存疑，这里应该是2字节
                ordinals_array_address += 2;
            }
        }
        Ok(ExportTable(Rc::new(RefCell::new(export_infos))))
    }

    pub fn fclone(&self) -> Self {
        ExportTable(Rc::clone(&self.0))
    }
    // pub(crate) fn _get_index(&self, index: usize) -> Option<&ExportInfo> {
    //     self.0.borrow().get(index)
    // }
    // pub(crate) fn _get_index_mut(&mut self, index: usize) -> Option<&mut ExportInfo> {
    //     self.0.borrow_mut().get_mut(index)
    // }
}
