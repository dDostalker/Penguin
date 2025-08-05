use crate::gui::FileManager;
use eframe::egui::{Label, Ui};

const MIN_SCROLLED_HEIGHT: f32 = 400.0;
impl FileManager {
    pub(crate) fn section_header_panel(&mut self, ui: &mut Ui) {
        // 获取节数量
        let section_num = match self.get_section_num() {
            Ok(num) => num,
            Err(e) => {
                self.sub_window_manager.show_error(&e.to_string());
                return;
            }
        };

        if section_num == 0 {
            ui.add(Label::new("该文件无节表"));
            return;
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
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        // 表头
                        ui.strong("节名称");
                        ui.strong("虚拟地址");
                        ui.strong("大小");
                        ui.strong("文件偏移");
                        ui.strong("特征");
                        ui.strong("操作");
                        ui.end_row();

                        for (_index, (name, virtual_addr, size, file_offset, characteristics)) in section_items.iter().enumerate() {
                            ui.label(name);
                            ui.label(virtual_addr);
                            ui.label(size);
                            ui.label(file_offset);
                            ui.label(characteristics);

                            ui.horizontal(|ui| {
                                if ui.button("复制").clicked() {
                                    let info = format!("节名: {}\n虚拟地址: {}\n大小: {}\n文件偏移: {}\n特征: {}",
                                        name, virtual_addr, size, file_offset, characteristics);
                                    ui.output_mut(|o| o.copied_text = info);
                                }
                            });

                            ui.end_row();
                        }
                    });
            });
        });
    }

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
            "{}",
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
            "{}",
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
            "{}",
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
    pub(crate) fn _get_section_number_of_relocations(&self, index: usize) -> String {
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
