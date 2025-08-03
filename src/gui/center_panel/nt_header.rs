use crate::gui::FileManager;
use eframe::egui::Ui;

const DATA_DIRECTORY_NAME: [&str; 16] = [
    "Export Table",
    "Import Table",
    "Resource Table",
    "Exception Table",
    "Security Directory",
    "Base Relocation Table",
    "Debug Directory",
    "X86 usage",
    "Architecture Sepcific Data",
    "RVA of GP",
    "TLS Directory",
    "Load Configuration Directory",
    "Bound Import Directory",
    "Import Address Table",
    "Delay Load Import Descriptors",
    "COM_DESCRiPTOR",
];

impl FileManager {
    pub(crate) fn nt_header_panel(&self, ui: &mut Ui) {
        eframe::egui::CentralPanel::default().show(ui.ctx(), |ui| {
            Self::show_main_title(ui, "NT Headers");
            eframe::egui::ScrollArea::vertical()
                .min_scrolled_height(400.0)
                .auto_shrink([true, false])
                .show(ui, |ui| {
                    // 签名部分
                    Self::show_sub_title(ui, "Signature");
                    eframe::egui::Grid::new("signature_grid")
                        .striped(true)
                        .spacing([10.0, 4.0])
                        .show(ui, |ui| {
                            ui.strong("字段名");
                            ui.strong("值");
                            ui.strong("描述");
                            ui.end_row();

                            ui.label("Signature");
                            ui.label(self.get_signature());
                            ui.label("文件头签名");
                            ui.end_row();
                        });

                    ui.add_space(10.0);

                    // File Header 部分
                    Self::show_sub_title(ui, "File Header");
                    eframe::egui::Grid::new("file_header_grid")
                        .striped(true)
                        .spacing([10.0, 4.0])
                        .show(ui, |ui| {
                            ui.strong("字段名");
                            ui.strong("值");
                            ui.strong("描述");
                            ui.end_row();

                            ui.label("Machine");
                            ui.label(self.get_machine());
                            ui.label("标记可以程序可以运行在什么样的CPU上");
                            ui.end_row();

                            ui.label("NumberOfSections");
                            ui.label(&self.get_number_of_sections());
                            ui.label("节数");
                            ui.end_row();

                            ui.label("TimeDateStamp");
                            ui.label(&self.get_time_date_stamp());
                            ui.label("时间戳");
                            ui.end_row();

                            ui.label("PointerToSymbolTable");
                            ui.label(&self.get_file_pointer_to_symbol_table());
                            ui.label("符号表指针");
                            ui.end_row();

                            ui.label("NumberOfSymbols");
                            ui.label(&self.get_number_of_symbols());
                            ui.label("符号数");
                            ui.end_row();

                            ui.label("SizeOfOptionalHeader");
                            ui.label(&self.get_size_of_optional_header());
                            ui.label("可选头大小");
                            ui.end_row();

                            ui.label("Characteristics");
                            ui.label(&self.get_characteristics());
                            ui.label("文件属性");
                            ui.end_row();
                        });

                    ui.add_space(10.0);

                    // Optional Header 部分
                    Self::show_sub_title(ui, "Optional Header");
                    eframe::egui::Grid::new("optional_header_grid")
                        .striped(true)
                        .spacing([10.0, 4.0])
                        .show(ui, |ui| {
                            ui.strong("字段名");
                            ui.strong("值");
                            ui.strong("描述");
                            ui.end_row();

                            ui.label("Magic");
                            ui.label(&self.get_magic());
                            ui.label("标记文件头");
                            ui.end_row();

                            ui.label("MajorLinkerVersion");
                            ui.label(&self.get_major_linker_version());
                            ui.label("链接器主版本号");
                            ui.end_row();

                            ui.label("MinorLinkerVersion");
                            ui.label(&self.get_minor_linker_version());
                            ui.label("链接器次版本号");
                            ui.end_row();

                            ui.label("SizeOfCode");
                            ui.label(&self.get_size_of_code());
                            ui.label("代码大小");
                            ui.end_row();

                            ui.label("SizeOfInitializedData");
                            ui.label(&self.get_size_of_initialized_data());
                            ui.label("初始化数据大小");
                            ui.end_row();

                            ui.label("SizeOfUninitializedData");
                            ui.label(&self.get_size_of_uninitialized_data());
                            ui.label("未初始化数据大小");
                            ui.end_row();

                            ui.label("AddressOfEntryPoint");
                            ui.label(&self.get_address_of_entry_point());
                            ui.label("入口点地址");
                            ui.end_row();

                            ui.label("BaseOfCode");
                            ui.label(&self.get_base_of_code());
                            ui.label("代码基址");
                            ui.end_row();

                            ui.label("BaseOfData");
                            ui.label(&self.get_base_of_data());
                            ui.label("数据基址");
                            ui.end_row();

                            ui.label("ImageBase");
                            ui.label(&self.get_image_base());
                            ui.label("映像基址");
                            ui.end_row();

                            ui.label("SectionAlignment");
                            ui.label(&self.get_section_alignment());
                            ui.label("节对齐");
                            ui.end_row();

                            ui.label("FileAlignment");
                            ui.label(&self.get_file_alignment());
                            ui.label("文件对齐");
                            ui.end_row();

                            ui.label("MajorOperatingSystemVersion");
                            ui.label(&self.get_major_operating_system_version());
                            ui.label("操作系统主版本号");
                            ui.end_row();

                            ui.label("MinorOperatingSystemVersion");
                            ui.label(&self.get_minor_operating_system_version());
                            ui.label("操作系统次版本号");
                            ui.end_row();

                            ui.label("MajorImageVersion");
                            ui.label(&self.get_major_image_version());
                            ui.label("映像主版本号");
                            ui.end_row();

                            ui.label("MinorImageVersion");
                            ui.label(&self.get_minor_image_version());
                            ui.label("映像次版本号");
                            ui.end_row();

                            ui.label("MajorSubsystemVersion");
                            ui.label(&self.get_major_subsystem_version());
                            ui.label("子系统主版本号");
                            ui.end_row();

                            ui.label("MinorSubsystemVersion");
                            ui.label(&self.get_minor_subsystem_version());
                            ui.label("子系统次版本号");
                            ui.end_row();

                            ui.label("Win32VersionValue");
                            ui.label(&self.get_win32_version_value());
                            ui.label("Win32版本值");
                            ui.end_row();

                            ui.label("SizeOfImage");
                            ui.label(&self.get_size_of_image());
                            ui.label("映像大小");
                            ui.end_row();

                            ui.label("SizeOfHeaders");
                            ui.label(&self.get_size_of_headers());
                            ui.label("头大小");
                            ui.end_row();

                            ui.label("CheckSum");
                            ui.label(&self.get_checksum());
                            ui.label("校验和");
                            ui.end_row();

                            ui.label("Subsystem");
                            ui.label(&self.get_subsystem());
                            ui.label("子系统");
                            ui.end_row();

                            ui.label("DllCharacteristics");
                            ui.label(&self.get_dll_characteristics());
                            ui.label("DLL属性");
                            ui.end_row();

                            ui.label("SizeOfStackReserve");
                            ui.label(&self.get_size_of_stack_reserve());
                            ui.label("堆栈预留大小");
                            ui.end_row();

                            ui.label("SizeOfStackCommit");
                            ui.label(&self.get_size_of_stack_commit());
                            ui.label("堆栈提交大小");
                            ui.end_row();

                            ui.label("SizeOfHeapReserve");
                            ui.label(&self.get_size_of_heap_reserve());
                            ui.label("堆预留大小");
                            ui.end_row();

                            ui.label("SizeOfHeapCommit");
                            ui.label(&self.get_size_of_heap_commit());
                            ui.label("堆提交大小");
                            ui.end_row();

                            ui.label("LoaderFlags");
                            ui.label(&self.get_loader_flags());
                            ui.label("加载器属性");
                            ui.end_row();

                            ui.label("NumberOfRvaAndSizes");
                            ui.label(&format!("{}", self.get_number_of_rva_and_sizes()));
                            ui.label("RVA和尺寸数");
                            ui.end_row();
                        });

                    ui.add_space(10.0);

                    // Data Directory 部分
                    Self::show_sub_title(ui, "Data Directory");
                    eframe::egui::Grid::new("data_directory_grid")
                        .striped(true)
                        .spacing([10.0, 4.0])
                        .show(ui, |ui| {
                            ui.strong("目录名称");
                            ui.strong("大小");
                            ui.strong("虚拟地址");
                            ui.end_row();

                            for i in 0..self.get_number_of_rva_and_sizes() {
                                let size = self.get_data_directory_size(i);
                                if size == "0" {
                                    continue;
                                }

                                ui.label(DATA_DIRECTORY_NAME[i as usize]);
                                ui.label(&size);
                                ui.label(&self.get_data_directory_virtual_address(i));
                                ui.end_row();
                            }
                        });
                });
        });
    }
    pub(crate) fn get_signature(&self) -> &str {
        self.files
            .get(self.current_index)
            .unwrap()
            .nt_head
            .get_signature()
    }
    pub(crate) fn get_machine(&self) -> &str {
        self.files
            .get(self.current_index)
            .unwrap()
            .nt_head
            .get_machine()
    }
    pub(crate) fn get_number_of_sections(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_number_of_sections()
        )
    }
    pub(crate) fn get_time_date_stamp(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_time_date_stamp()
        )
    }
    pub(crate) fn get_file_pointer_to_symbol_table(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_pointer_to_symbol_table()
        )
    }
    pub(crate) fn get_number_of_symbols(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_number_of_symbols()
        )
    }
    pub(crate) fn get_size_of_optional_header(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_optional_header()
        )
    }
    pub(crate) fn get_characteristics(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_characteristics()
        )
    }
    pub(crate) fn get_magic(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_magic()
        )
    }
    pub(crate) fn get_major_linker_version(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_major_linker_version()
        )
    }
    pub(crate) fn get_minor_linker_version(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_long_minor_linker_version()
        )
    }
    pub(crate) fn get_size_of_code(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_code()
        )
    }
    pub(crate) fn get_size_of_initialized_data(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_initialized_data()
        )
    }
    pub(crate) fn get_size_of_uninitialized_data(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_uninitialized_data()
        )
    }
    pub(crate) fn get_address_of_entry_point(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_address_of_entry_point()
        )
    }
    pub(crate) fn get_base_of_code(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_base_of_code()
        )
    }
    pub(crate) fn get_base_of_data(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_base_of_data()
        )
    }
    pub(crate) fn get_image_base(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_image_base()
        )
    }
    pub(crate) fn get_section_alignment(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_section_alignment()
        )
    }
    pub(crate) fn get_file_alignment(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_file_alignment()
        )
    }
    pub(crate) fn get_major_operating_system_version(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_major_os_version()
        )
    }
    pub(crate) fn get_minor_operating_system_version(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_minor_os_version()
        )
    }
    pub(crate) fn get_major_image_version(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_major_image_version()
        )
    }
    pub(crate) fn get_minor_image_version(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_minor_image_version()
        )
    }
    pub(crate) fn get_major_subsystem_version(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_major_subsystem_version()
        )
    }
    pub(crate) fn get_minor_subsystem_version(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_minor_subsystem_version()
        )
    }
    pub(crate) fn get_win32_version_value(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_win32_version_value()
        )
    }
    pub(crate) fn get_size_of_image(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_image()
        )
    }
    pub(crate) fn get_size_of_headers(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_headers()
        )
    }
    pub(crate) fn get_checksum(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_checksum()
        )
    }
    pub(crate) fn get_subsystem(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_subsystem()
        )
    }
    pub(crate) fn get_dll_characteristics(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_dll_characteristics()
        )
    }
    pub(crate) fn get_size_of_stack_reserve(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_stack_reserve()
        )
    }
    pub(crate) fn get_size_of_stack_commit(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_stack_commit()
        )
    }
    pub(crate) fn get_size_of_heap_reserve(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_heap_reserve()
        )
    }
    pub(crate) fn get_size_of_heap_commit(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_heap_commit()
        )
    }
    pub(crate) fn get_loader_flags(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_loader_flags()
        )
    }
    pub(crate) fn get_number_of_rva_and_sizes(&self) -> u32 {
        self.files
            .get(self.current_index)
            .unwrap()
            .nt_head
            .get_number_of_rva_and_sizes()
    }
    pub(crate) fn get_data_directory_size(&self, index: u32) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .data_directory
                .get_data_directory_size(index)
                .unwrap()
        )
    }
    pub(crate) fn get_data_directory_virtual_address(&self, index: u32) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .data_directory
                .get_data_directory_virtual_address(index)
                .unwrap()
        )
    }
}
