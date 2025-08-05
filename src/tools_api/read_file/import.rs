use std::any::Any;
use std::io::SeekFrom;
use std::mem::transmute;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt};

use crate::tools_api::read_file::{
    DataDirectory, ImageDosHeader, ImageSectionHeaders, ImportDescriptor, ImportDll,
    ImportFunction, is_64, rva_2_fo,
};

use crate::tools_api::read_file::nt_header::traits::NtHeaders;

impl ImportDescriptor {
    pub async fn new<T>(
        file: &mut File,
        nt_head: &T,
        image_section_headers: &ImageSectionHeaders,
        data_dir: &DataDirectory,
        index: u32,
    ) -> anyhow::Result<Option<ImportDescriptor>>
    where
        T: NtHeaders + ?Sized,
    {
        let mut import_descriptor: ImportDescriptor = Default::default();
        let fo: u32 = rva_2_fo(
            nt_head,
            image_section_headers,
            data_dir.get_import_directory_address().await?,
        )
        .await
        .unwrap() as _;
        file.seek(SeekFrom::Start((fo + index * 0x14) as u64))
            .await?;
        unsafe {
            let read: &mut [u8; size_of::<ImportFunction>()] = transmute(&mut import_descriptor);
            file.read(read).await?;
        }
        // 特殊的情况，有时pe的data dic的大小并不完全代表着他import dll的个数，而是类似列表最后为0来结束
        if import_descriptor.name_address == 0 {
            return Ok(None);
        }
        Ok(Some(import_descriptor))
    }
}
impl ImportFunction {
    pub async fn new(file: &mut File, addr: u32) -> anyhow::Result<Option<ImportFunction>> {
        file.seek(SeekFrom::Start((addr + 2) as u64)).await?;
        let mut buf = [0; 256];
        file.read(&mut buf).await?;
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
        Ok(Some(ImportFunction {
            name_address: addr + 2,
            name_length: name.len() as u32,
            name_max_length,
            name,
        }))
    }
}

impl ImportDll {
    pub async fn new<T>(
        file: &mut File,
        image_dos_header: &ImageDosHeader,
        import_descriptor: ImportDescriptor,
        nt_head: &T,
        section_headers: &ImageSectionHeaders,
    ) -> anyhow::Result<ImportDll>
    where
        T: NtHeaders + Any + ?Sized,
    {
        let mut function_info = Vec::new();
        let mut addr;
        let function_info_address;
        #[cfg(debug_assertions)]
        eprintln!(
            "tools_api:import:import_descriptor.dummy_union_name {}",
            import_descriptor.dummy_union_name
        );
        match rva_2_fo(nt_head, section_headers, import_descriptor.dummy_union_name).await {
            None => return Err(anyhow::anyhow!("End")),
            Some(ret) => function_info_address = ret,
        }
        file.seek(SeekFrom::Start(function_info_address as u64))
            .await?;
        let mut i = 0;
        loop {
            if is_64(file, image_dos_header).await? {
                file.seek(SeekFrom::Start(function_info_address as u64 + i * 8u64))
                    .await?;

                addr = file.read_u64_le().await?;
            } else {
                file.seek(SeekFrom::Start(function_info_address as u64 + i * 4u64))
                    .await?;
                addr = file.read_u32_le().await? as u64;
            }
            if addr == 0 {
                break;
            }
            i += 1;

            //pe文件存在值为 0x8000000000000003的情况，此时的值是无效的，只截取后位即可
            let addr: u32 = if addr > 0x0FFFFFFF {
                function_info.push(ImportFunction {
                    name_address: addr as u32,
                    name_length: 0,
                    name_max_length: 0,
                    name: format!("0x{:x}", addr & 0x0FFFFFFF),
                });
                continue;
            } else {
                addr as u32
            };
            #[cfg(debug_assertions)]
            eprintln!("tools_api:read_file:imports.rs addr_rva:{}", addr);
            if let Some(addr) = rva_2_fo(nt_head, section_headers, addr).await {
                #[cfg(debug_assertions)]
                eprintln!("tools_api:read_file:imports.rs addr_foa:{}", addr);
                match ImportFunction::new(file, addr).await? {
                    None => {
                        break;
                    }
                    Some(import_func_info) => {
                        function_info.push(import_func_info);
                    }
                }
            }
        }
        let mut name = [0u8; 256];
        file.seek(SeekFrom::Start(
            rva_2_fo(nt_head, section_headers, import_descriptor.name_address)
                .await
                .unwrap() as u64,
        ))
        .await?;
        file.read(&mut name).await? as u64;
        let name = String::from_utf8_lossy(name.split(|x| *x == 0).next().unwrap()).parse()?;
        Ok(ImportDll {
            name_address: import_descriptor.name_address,
            name_length: 0,
            name,
            time_date_stamp: 0,
            forwarder_chain: 0,
            first_thunk: 0,
            function_info,
            function_size: 0,
        })
    }
}
