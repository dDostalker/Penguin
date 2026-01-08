use crate::gui::{FileManager, SectionFlag};
use crate::i18n;
use crate::tools_api::read_file::section_headers::SectionCharacteristics;
use eframe::egui::{Context, Label, Ui, Vec2};
const MIN_SCROLLED_HEIGHT: f32 = 400.0;
const SPACING: Vec2 = Vec2::new(20.0, 8.0);
const COLUMNS: usize = 7;
impl FileManager {
    pub(crate) fn section_header_panel(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
        let section_num = self.get_section_num()?;
        if section_num == 0 {
            ui.add(Label::new(i18n::NO_SECTIONS));
            return Ok(());
        }
        // 创建数据副本以避免借用冲突
        let section_items: Vec<_> = (0..section_num)
            .map(|i| {
                (
                    self.get_section_name(i).unwrap_or("None".to_string()),
                    self.get_section_virtual_address(i),
                    self.get_section_size_of_raw_data(i)
                        .unwrap_or("None".to_string()),
                    self.get_section_pointer_to_raw_data(i),
                    self.get_section_characteristics(i),
                    self.get_section_number_of_relocations(i),
                )
            })
            .collect();
        eframe::egui::CentralPanel::default().show(ui.ctx(), |ui| {
            Self::show_main_title(ui, "Section Headers");
            eframe::egui::ScrollArea::vertical()
                .min_scrolled_height(MIN_SCROLLED_HEIGHT)
                .show(ui, |ui| {
                    let width = ui.available_width();
                    let col_width = width / COLUMNS as f32;
                    eframe::egui::Grid::new("section_table")
                        .striped(true)
                        .spacing(SPACING)
                        .num_columns(COLUMNS)
                        .min_col_width(col_width)
                        .show(ui, |ui| {
                            ui.strong(i18n::SECTION_NAME);
                            ui.strong(i18n::VIRTUAL_ADDRESS);
                            ui.strong(i18n::SIZE);
                            ui.strong(i18n::FILE_OFFSET);
                            ui.strong(i18n::RELOCATION_ADDRESS);
                            ui.strong(i18n::CHARACTERISTICS);
                            ui.strong(i18n::OPERATION);
                            ui.end_row();

                            for (
                                index,
                                (
                                    name,
                                    virtual_addr,
                                    size,
                                    file_offset,
                                    characteristics,
                                    relocations,
                                ),
                            ) in section_items.iter().enumerate()
                            {
                                ui.label(name);
                                ui.label(virtual_addr);
                                ui.label(size);
                                ui.label(file_offset);
                                ui.label(relocations);
                                if ui.button(characteristics).clicked() {
                                    self.sub_window_manager
                                        .section_message
                                        .selected_section_index = Some(index);
                                    self.sub_window_manager.section_message.section_flag = None;
                                    if self
                                        .sub_window_manager
                                        .section_message
                                        .section_flag
                                        .is_none()
                                    {
                                        self.sub_window_manager.section_message.section_flag =
                                            Some(SectionFlag::match_flag(
                                                self.get_section_characteristics_u32(index),
                                            ));
                                    }
                                }

                                ui.horizontal(|ui| {
                                    if ui.button(i18n::COPY_BUTTON).clicked() {
                                        let info = format!(
                                            "{}",
                                            i18n::SECTION_INFO_FORMAT
                                                .replace("{}", name)
                                                .replace("{}", virtual_addr)
                                                .replace("{}", size)
                                                .replace("{}", file_offset)
                                                .replace("{}", characteristics)
                                        );
                                        ui.output_mut(|o| o.copied_text = info);
                                    }
                                });
                                ui.end_row();
                            }
                        });
                });
        });

        if self
            .sub_window_manager
            .section_message
            .selected_section_index
            .is_some()
        {
            eframe::egui::TopBottomPanel::bottom("section_detail_window").show(ui.ctx(), |ui| {
                ui.label("Section Details");
                ui.horizontal(|ui| {
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_cnt_code(),
                            "Code",
                        )
                        .clicked()
                    {
                        // need fix
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnCntCode as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_cnt_initialized_data(),
                            "Initialized Data",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^=
                            SectionCharacteristics::ImageScnCntInitializedData as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_cnt_uninitialized_data(),
                            "Uninitialized Data",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^=
                            SectionCharacteristics::ImageScnCntUninitializedData as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_lnk_other(),
                            "Other",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnLnkOther as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_lnk_info(),
                            "Info",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnLnkInfo as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_lnk_remove(),
                            "Remove",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnLnkRemove as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_lnk_comdat(),
                            "Comdat",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnLnkComdat as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_no_defer_spec_exc(),
                            "No Defer Spec Exc",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnNoDeferSpecExc as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_gprel(),
                            "GPREL",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnGprel as u32;
                    }
                });

                ui.horizontal(|ui| {
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_lnk_nreloc_ovfl(),
                            "Link Nreloc Ovfl",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnLnkNrelocOvfl as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_mem_discardable(),
                            "Mem Discardable",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnMemDiscardable as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_mem_not_paged(),
                            "Mem Not Paged",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnMemNotPaged as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_mem_shared(),
                            "Mem Shared",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnMemShared as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_mem_execute(),
                            "Mem Execute",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnMemExecute as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_mem_read(),
                            "Mem Read",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnMemRead as u32;
                    }
                    if ui
                        .checkbox(
                            &mut self
                                .sub_window_manager
                                .section_message
                                .get_image_scn_mem_write(),
                            "Mem Write",
                        )
                        .clicked()
                    {
                        self.files[self.current_index].section_headers.0[self
                            .sub_window_manager
                            .section_message
                            .selected_section_index
                            .unwrap()]
                        .characteristics ^= SectionCharacteristics::ImageScnMemWrite as u32;
                    }
                });

                if ui.button("X").clicked() {
                    self.sub_window_manager
                        .section_message
                        .selected_section_index = None;
                }
            });
        }
        Ok(())
    }
    // unwrap or 修改
    pub(crate) fn get_section_num(&self) -> anyhow::Result<usize> {
        self.files
            .get(self.current_index)
            .unwrap_or(&self.files[0])
            .section_headers
            .get_num()
    }
    pub(crate) fn get_section_size_of_raw_data(&self, index: usize) -> anyhow::Result<String> {
        Ok(format!(
            "0x{:08X}",
            self.files
                .get(self.current_index)
                .unwrap_or(&self.files[0])
                .section_headers
                .get_section_size_of_raw_data(index)?
        ))
    }
    pub(crate) fn get_section_name(&self, index: usize) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap_or(&self.files[0])
                .section_headers
                .get_section_name(index)?
        ))
    }
    pub(crate) fn get_section_pointer_to_raw_data(&self, index: usize) -> String {
        format!(
            "0x{:08X}",
            self.files
                .get(self.current_index)
                .unwrap_or(&self.files[0])
                .section_headers
                .get_section_pointer_to_raw_data(index)
        )
    }
    pub(crate) fn get_section_characteristics(&self, index: usize) -> String {
        format!(
            "0x{:X}",
            self.files
                .get(self.current_index)
                .unwrap_or(&self.files[0])
                .section_headers
                .get_section_characteristics(index)
        )
    }

    // pub(crate) fn get_section_misc(&self, index: usize) -> anyhow::Result<String> {
    //     Ok(format!(
    //         "0x{:X}",
    //         self.files
    //             .get(self.current_index)
    //             .unwrap_or(&self.files[0])
    //             .section_headers
    //             .get_section_misc(index)?
    //     ))
    // }
    pub(crate) fn get_section_virtual_address(&self, index: usize) -> String {
        format!(
            "0x{:08X}",
            self.files
                .get(self.current_index)
                .unwrap_or(&self.files[0])
                .section_headers
                .get_section_virtual_address(index)
        )
    }
    // pub(crate) fn get_section_number_of_linenumbers(&self, index: usize) -> String {
    //     format!(
    //         "{}",
    //         self.files
    //             .get(self.current_index)
    //             .unwrap_or(&self.files[0])
    //             .section_headers
    //             .get_section_number_of_linenumbers(index)
    //     )
    // }
    pub(crate) fn get_section_number_of_relocations(&self, index: usize) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap_or(&self.files[0])
                .section_headers
                .get_section_number_of_relocations(index)
        )
    }
    // pub(crate) fn get_section_pointer_to_relocations(&self, index: usize) -> String {
    //     format!(
    //         "{}",
    //         self.files
    //             .get(self.current_index)
    //             .unwrap_or(&self.files[0])
    //             .section_headers
    //             .get_section_pointer_to_relocations(index)
    //     )
    // }
    fn get_section_characteristics_u32(&self, index: usize) -> u32 {
        self.files
            .get(self.current_index)
            .unwrap_or(&self.files[0])
            .section_headers
            .get_section_characteristics(index)
    }
}
