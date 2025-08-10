use crate::gui::FileManager;
use crate::i18n;
use eframe::egui::{Label, Ui, Vec2};

const MIN_SCROLLED_HEIGHT: f32 = 400.0;
const SPACING: Vec2 = Vec2::new(20.0, 8.0);
const COLUMNS: usize = 5;
impl FileManager {
    pub(crate) fn section_header_panel(&mut self, ui: &mut Ui)->anyhow::Result<()> {
        // 获取节数量
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
                // 使用表格样式
                eframe::egui::Grid::new("section_table")
                    .striped(true)
                    .spacing(SPACING)
                    .num_columns(COLUMNS)
                    .show(ui, |ui| {
                        // 表头
                        ui.strong(i18n::SECTION_NAME);
                        ui.strong(i18n::VIRTUAL_ADDRESS);
                        ui.strong(i18n::SIZE);
                        ui.strong(i18n::FILE_OFFSET);
                        ui.strong(i18n::RELOCATION_ADDRESS);
                        ui.strong(i18n::CHARACTERISTICS);
                        ui.strong(i18n::OPERATION);
                        ui.end_row();

                        for (index, (name, virtual_addr, size, file_offset, characteristics, relocations)) in section_items.iter().enumerate() {
                            ui.label(name);
                            ui.label(virtual_addr);
                            ui.label(size);
                            ui.label(file_offset);
                            ui.label(relocations);
                            if ui.button(characteristics).clicked() {
                                self.sub_window_manager.show_info(&self.get_section_characteristics_hover(index));
                            }

                            ui.horizontal(|ui| {
                                if ui.button(i18n::COPY_BUTTON).clicked() {
                                    let info = format!("{}", i18n::SECTION_INFO_FORMAT
                                        .replace("{}", name)
                                        .replace("{}", virtual_addr)
                                        .replace("{}", size)
                                        .replace("{}", file_offset)
                                        .replace("{}", characteristics));
                                    ui.output_mut(|o| o.copied_text = info);
                                }
                            });
                            ui.end_row();
                        }   
                    });
            });
        });
        Ok(())
    }
    // unwrap or 修改
    pub(crate) fn get_section_num(&self) -> anyhow::Result<usize> {
        Ok(self
            .files
            .get(self.current_index)
            .unwrap_or(&self.files[0])
            .section_headers
            .get_num()?)
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
            "{}",
            self.files
                .get(self.current_index)
                .unwrap_or(&self.files[0])
                .section_headers
                .get_section_characteristics(index)
        )
    }
    pub(crate) fn get_section_characteristics_hover(&self, index: usize) -> String {
        self.files
            .get(self.current_index)
            .unwrap_or(&self.files[0])
            .section_headers
            .get_section_characteristics_hover(index)
    }
    pub(crate) fn _get_section_misc(&self, index: usize) -> anyhow::Result<String> {
        Ok(format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap_or(&self.files[0])
                .section_headers
                .get_section_misc(index)?
        ))
    }
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
    pub(crate) fn _get_section_number_of_linenumbers(&self, index: usize) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap_or(&self.files[0])
                .section_headers
                .get_section_number_of_linenumbers(index)
        )
    }
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
    pub(crate) fn _get_section_pointer_to_relocations(&self, index: usize) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap_or(&self.files[0])
                .section_headers
                .get_section_pointer_to_relocations(index)
        )
    }
}
