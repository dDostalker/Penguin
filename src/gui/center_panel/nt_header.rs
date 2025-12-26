use crate::gui::FileManager;
use crate::i18n;
use eframe::egui::{Ui, Vec2};

const SPACING: Vec2 = Vec2::new(20.0, 8.0);
const COLUMNS: usize = 3;
const MIN_SCROLLED_HEIGHT: f32 = 400.0;
const ADD_SPACE: f32 = 10.0;
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
    pub(crate) fn nt_header_panel(&mut self, ui: &mut Ui) {
        // let ctx = ui.ctx();
        eframe::egui::CentralPanel::default().show(ui.ctx(), |ui| {
            Self::show_main_title(ui, "NT Headers");
            eframe::egui::ScrollArea::vertical()
                .min_scrolled_height(MIN_SCROLLED_HEIGHT)
                // .auto_shrink([true, false])
                .show(ui, |ui| {
                    // 签名部分
                    Self::show_sub_title(ui, "Signature");
                    eframe::egui::Grid::new("signature_grid")
                        .striped(true)
                        .spacing(SPACING)
                        .num_columns(COLUMNS)
                        .min_col_width(ui.ctx().used_size().x / COLUMNS as f32)
                        .show(ui, |ui| {
                            ui.strong(i18n::FIELD_NAME);
                            ui.strong(i18n::VALUE);
                            ui.strong(i18n::DESCRIPTION);
                            ui.end_row();

                            ui.label("Signature");
                            ui.label(self.get_signature());
                            ui.label(i18n::FILE_HEADER_SIGNATURE);
                            ui.end_row();
                        });

                    ui.add_space(ADD_SPACE);

                    // File Header 部分
                    Self::show_sub_title(ui, "File Header");
                    eframe::egui::Grid::new("file_header_grid")
                        .min_col_width(ui.ctx().used_size().x / COLUMNS as f32)
                        .striped(true)
                        .spacing(SPACING)
                        .num_columns(COLUMNS)
                        .show(ui, |ui| {
                            ui.strong(i18n::FIELD_NAME);
                            ui.strong(i18n::VALUE);
                            ui.strong(i18n::DESCRIPTION);
                            ui.end_row();

                            ui.label("Machine");
                            ui.label(self.get_machine());
                            ui.label(i18n::MACHINE_DESCRIPTION);
                            ui.end_row();

                            ui.label("NumberOfSections");
                            ui.label(&self.get_number_of_sections());
                            ui.label(i18n::NUMBER_OF_SECTIONS);
                            ui.end_row();

                            ui.label("TimeDateStamp");
                            ui.label(&self.get_time_date_stamp());
                            ui.label(i18n::TIMESTAMP);
                            ui.end_row();

                            ui.label("PointerToSymbolTable");
                            ui.label(&self.get_file_pointer_to_symbol_table());
                            ui.label(i18n::POINTER_TO_SYMBOL_TABLE);
                            ui.end_row();

                            ui.label("NumberOfSymbols");
                            ui.label(&self.get_number_of_symbols());
                            ui.label(i18n::NUMBER_OF_SYMBOLS);
                            ui.end_row();

                            ui.label("SizeOfOptionalHeader");
                            ui.label(&self.get_size_of_optional_header());
                            ui.label(i18n::SIZE_OF_OPTIONAL_HEADER);
                            ui.end_row();

                            ui.label("Characteristics");
                            if ui.button(&self.get_characteristics()).clicked() {
                                self.sub_window_manager
                                    .show_info(&self.get_characteristics_hover());
                            }
                            ui.label(i18n::NT_HEADER_FILE_CHARACTERISTICS);
                            ui.end_row();
                        });

                    ui.add_space(ADD_SPACE);

                    // Optional Header 部分
                    Self::show_sub_title(ui, "Optional Header");
                    eframe::egui::Grid::new("optional_header_grid")
                        .striped(true)
                        .min_col_width(ui.ctx().used_size().x / COLUMNS as f32)
                        .spacing(SPACING)
                        .num_columns(COLUMNS)
                        .show(ui, |ui| {
                            ui.strong(i18n::FIELD_NAME);
                            ui.strong(i18n::VALUE);
                            ui.strong(i18n::DESCRIPTION);
                            ui.end_row();

                            ui.label("Magic");
                            if ui.button(&self.get_magic()).clicked() {
                                self.sub_window_manager.show_info(&self.get_magic_hover());
                            }
                            ui.label(i18n::NT_HEADER_MAGIC);
                            ui.end_row();

                            ui.label("MajorLinkerVersion");
                            ui.label(&self.get_major_linker_version());
                            ui.label(i18n::NT_HEADER_MAJOR_LINKER_VERSION);
                            ui.end_row();

                            ui.label("MinorLinkerVersion");
                            ui.label(&self.get_minor_linker_version());
                            ui.label(i18n::NT_HEADER_MINOR_LINKER_VERSION);
                            ui.end_row();

                            ui.label("SizeOfCode");
                            ui.label(&self.get_size_of_code());
                            ui.label(i18n::NT_HEADER_SIZE_OF_CODE);
                            ui.end_row();

                            ui.label("SizeOfInitializedData");
                            ui.label(self.get_size_of_initialized_data());
                            ui.label(i18n::NT_HEADER_SIZE_OF_INITIALIZED_DATA);
                            ui.end_row();

                            ui.label("SizeOfUninitializedData");
                            ui.label(&self.get_size_of_uninitialized_data());
                            ui.label(i18n::NT_HEADER_SIZE_OF_UNINITIALIZED_DATA);
                            ui.end_row();

                            ui.label("AddressOfEntryPoint");
                            ui.label(&self.get_address_of_entry_point());
                            ui.label(i18n::NT_HEADER_ADDRESS_OF_ENTRY_POINT);
                            ui.end_row();

                            ui.label("BaseOfCode");
                            ui.label(&self.get_base_of_code());
                            ui.label(i18n::NT_HEADER_BASE_OF_CODE);
                            ui.end_row();

                            ui.label("BaseOfData");
                            ui.label(&self.get_base_of_data());
                            ui.label(i18n::NT_HEADER_BASE_OF_DATA);
                            ui.end_row();

                            ui.label("ImageBase");
                            ui.label(&self.get_image_base());
                            ui.label(i18n::NT_HEADER_IMAGE_BASE);
                            ui.end_row();

                            ui.label("SectionAlignment");
                            ui.label(&self.get_section_alignment());
                            ui.label(i18n::NT_HEADER_SECTION_ALIGNMENT);
                            ui.end_row();

                            ui.label("FileAlignment");
                            ui.label(&self.get_file_alignment());
                            ui.label(i18n::NT_HEADER_FILE_ALIGNMENT);
                            ui.end_row();

                            ui.label("MajorOperatingSystemVersion");
                            ui.label(&self.get_major_operating_system_version());
                            ui.label(i18n::NT_HEADER_MAJOR_OPERATING_SYSTEM_VERSION);
                            ui.end_row();

                            ui.label("MinorOperatingSystemVersion");
                            ui.label(&self.get_minor_operating_system_version());
                            ui.label(i18n::NT_HEADER_MINOR_OPERATING_SYSTEM_VERSION);
                            ui.end_row();

                            ui.label("MajorImageVersion");
                            ui.label(&self.get_major_image_version());
                            ui.label(i18n::NT_HEADER_MAJOR_IMAGE_VERSION);
                            ui.end_row();

                            ui.label("MinorImageVersion");
                            ui.label(&self.get_minor_image_version());
                            ui.label(i18n::NT_HEADER_MINOR_IMAGE_VERSION);
                            ui.end_row();

                            ui.label("MajorSubsystemVersion");
                            ui.label(&self.get_major_subsystem_version());
                            ui.label(i18n::NT_HEADER_MAJOR_SUBSYSTEM_VERSION);
                            ui.end_row();

                            ui.label("MinorSubsystemVersion");
                            ui.label(&self.get_minor_subsystem_version());
                            ui.label(i18n::NT_HEADER_MINOR_SUBSYSTEM_VERSION);
                            ui.end_row();

                            ui.label("Win32VersionValue");
                            ui.label(&self.get_win32_version_value());
                            ui.label(i18n::NT_HEADER_WIN32_VERSION_VALUE);
                            ui.end_row();

                            ui.label("SizeOfImage");
                            ui.label(&self.get_size_of_image());
                            ui.label(i18n::NT_HEADER_SIZE_OF_IMAGE);
                            ui.end_row();

                            ui.label("SizeOfHeaders");
                            ui.label(&self.get_size_of_headers());
                            ui.label(i18n::NT_HEADER_SIZE_OF_HEADERS);
                            ui.end_row();

                            ui.label("CheckSum");
                            ui.label(&self.get_checksum());
                            ui.label(i18n::NT_HEADER_CHECKSUM);
                            ui.end_row();

                            ui.label("Subsystem");
                            ui.label(&self.get_subsystem());
                            ui.label(i18n::NT_HEADER_SUBSYSTEM);
                            ui.end_row();

                            ui.label("DllCharacteristics");
                            if ui.button(&self.get_dll_characteristics()).clicked() {
                                self.sub_window_manager
                                    .show_info(&self.get_dll_characteristics_hover());
                            }
                            ui.label(i18n::NT_HEADER_DLL_CHARACTERISTICS);
                            ui.end_row();

                            ui.label("SizeOfStackReserve");
                            ui.label(&self.get_size_of_stack_reserve());
                            ui.label(i18n::NT_HEADER_SIZE_OF_STACK_RESERVE);
                            ui.end_row();

                            ui.label("SizeOfStackCommit");
                            ui.label(&self.get_size_of_stack_commit());
                            ui.label(i18n::NT_HEADER_SIZE_OF_STACK_COMMIT);
                            ui.end_row();

                            ui.label("SizeOfHeapReserve");
                            ui.label(&self.get_size_of_heap_reserve());
                            ui.label(i18n::NT_HEADER_SIZE_OF_HEAP_RESERVE);
                            ui.end_row();

                            ui.label("SizeOfHeapCommit");
                            ui.label(&self.get_size_of_heap_commit());
                            ui.label(i18n::NT_HEADER_SIZE_OF_HEAP_COMMIT);
                            ui.end_row();

                            ui.label("LoaderFlags");
                            ui.label(&self.get_loader_flags());
                            ui.label(i18n::NT_HEADER_LOADER_FLAGS);
                            ui.end_row();

                            ui.label("NumberOfRvaAndSizes");
                            ui.label(&format!("{}", self.get_number_of_rva_and_sizes()));
                            ui.label(i18n::NT_HEADER_NUMBER_OF_RVA_AND_SIZES);
                            ui.end_row();
                        });

                    ui.add_space(ADD_SPACE);

                    // Data Directory 部分
                    Self::show_sub_title(ui, "Data Directory");
                    eframe::egui::Grid::new("data_directory_grid")
                        .striped(true)
                        .spacing(SPACING)
                        .min_col_width(ui.ctx().used_size().x / COLUMNS as f32)
                        .num_columns(COLUMNS)
                        .show(ui, |ui| {
                            ui.strong(i18n::DATA_DIRECTORY_NAME);
                            ui.strong(i18n::SIZE);
                            ui.strong(i18n::VIRTUAL_ADDRESS);
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
    pub(crate) fn get_characteristics_hover(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_characteristics_hover()
        )
    }
    pub(crate) fn get_magic(&self) -> String {
        format!(
            "0x{:04X}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_magic()
        )
    }
    pub(crate) fn get_magic_hover(&self) -> String {
        self.files
            .get(self.current_index)
            .unwrap()
            .nt_head
            .get_magic_hover()
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
            "0x{:08X}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_address_of_entry_point()
        )
    }
    pub(crate) fn get_base_of_code(&self) -> String {
        format!(
            "0x{:08X}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_base_of_code()
        )
    }
    pub(crate) fn get_base_of_data(&self) -> String {
        format!(
            "0x{:08X}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_base_of_data()
        )
    }
    pub(crate) fn get_image_base(&self) -> String {
        format!(
            "0x{:08X}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_image_base()
        )
    }
    pub(crate) fn get_section_alignment(&self) -> String {
        format!(
            "0x{:04X}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_section_alignment()
        )
    }
    pub(crate) fn get_file_alignment(&self) -> String {
        format!(
            "0x{:04X}",
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
    pub(crate) fn get_dll_characteristics_hover(&self) -> String {
        self.files
            .get(self.current_index)
            .unwrap()
            .nt_head
            .get_dll_characteristics_hover()
    }
    pub(crate) fn get_size_of_stack_reserve(&self) -> String {
        format!(
            "0x{:08X}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_stack_reserve()
        )
    }
    pub(crate) fn get_size_of_stack_commit(&self) -> String {
        format!(
            "0x{:08X}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_stack_commit()
        )
    }
    pub(crate) fn get_size_of_heap_reserve(&self) -> String {
        format!(
            "0x{:08X}",
            self.files
                .get(self.current_index)
                .unwrap()
                .nt_head
                .get_size_of_heap_reserve()
        )
    }
    pub(crate) fn get_size_of_heap_commit(&self) -> String {
        format!(
            "0x{:08X}",
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
            "0x{:08X}",
            self.files
                .get(self.current_index)
                .unwrap()
                .data_directory
                .get_data_directory_virtual_address(index)
                .unwrap()
        )
    }
}
