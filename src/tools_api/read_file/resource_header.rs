use crate::tools_api::read_file::ImageResourceDirectory;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};
use crate::tools_api::read_file::nt_header::traits::NtHeaders;
use crate::tools_api::read_file::ImageSectionHeaders;
use crate::tools_api::read_file::DataDirectory;
use crate::tools_api::read_file::rva_2_fo;
use std::mem::size_of;
use std::mem::transmute;

impl ImageResourceDirectory {
    pub async fn new<T>(file: &mut File,nt_head: &T,image_section_headers: &ImageSectionHeaders,data_dir: &DataDirectory) -> anyhow::Result<Self> 
    where
        T: NtHeaders + ?Sized,
    {
        let mut resource_directory: ImageResourceDirectory = Default::default();
        if let Some(fo) = rva_2_fo(nt_head, image_section_headers, data_dir.get_resource_directory_address().await?) {
            file.seek(SeekFrom::Start(fo as u64)).await?;
            println!("fo: {}", fo);
            unsafe {
                let read: &mut [u8; size_of::<ImageResourceDirectory>()] = transmute(&mut resource_directory);
                file.read(read).await?;
            }
        }
        Ok(resource_directory)
    }
}

#[cfg(test)]
mod tests {
    use crate::tools_api::read_file::nt_header::{read_nt_head,traits::NtHeaders};
    use crate::tools_api::read_file::ImageDosHeader;
    use crate::tools_api::read_file::ImageSectionHeaders;
    use crate::tools_api::read_file::ImageNtHeaders64;
    use tokio::fs::File;
    use tokio::io::{AsyncReadExt, AsyncSeekExt, SeekFrom};
    use crate::tools_api::read_file::ImageResourceDirectory;

    #[tokio::test]
    async fn test_image_resource_directory() {
        let mut file = File::open(r"C:\Users\Admin\Desktop\Project.Zomboid.v41.78.16\ProjectZomboid64.exe").await.unwrap();
        let image_dos_header = ImageDosHeader::new(&mut file).await.unwrap();
        let (nt_head, data_dir) = read_nt_head::<ImageNtHeaders64>(&mut file, image_dos_header.e_lfanew as u16).await.unwrap();
        let image_section_headers = ImageSectionHeaders::new(&mut file, nt_head.section_start(image_dos_header.e_lfanew as u16), nt_head.section_number()).await.unwrap();
        let resource_directory = ImageResourceDirectory::new(&mut file, &nt_head, &image_section_headers, &data_dir).await.unwrap();
        println!("{:?}", resource_directory);
    }
}