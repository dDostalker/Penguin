use crate::i18n;
use eframe::egui::{Ui, Vec2};

use crate::{
    gui::FileManager,
    tools_api::{read_file::ExportTable, search},
};

const MIN_SCROLLED_HEIGHT: f32 = 400.0;
const DESIGN_SIZE_FUNC_NAME: Vec2 = Vec2::new(400.0 * 0.5, 0.0);
const DESIGN_SIZE_FUNC_ADDR: Vec2 = Vec2::new(400.0 * 0.25, 0.0);
const DESIGN_SIZE_FUNC_OPERATE: Vec2 = Vec2::new(400.0 * 0.25, 0.0);
const SPACING: Vec2 = Vec2::new(20.0, 8.0);
const COLUMNS: usize = 3;
const MAX_FUNC_NAME_LENGTH: usize = 50;
impl FileManager {
    pub(crate) fn export_panel(&mut self, ui: &mut Ui) {
        // 预先获取数据，避免在渲染循环中重复调用
        let export_data = match self.get_export() {
            Ok(export) => export,
            Err(_) => {
                return;
            }
        };

        // clone to avoid borrow conflict
        let export_data_clone = export_data.fclone();
        let selected_index = self.sub_window_manager.export_message.selected_export_index;

        eframe::egui::CentralPanel::default().show(ui.ctx(), |ui| {
            eframe::egui::ScrollArea::vertical()
                .min_scrolled_height(MIN_SCROLLED_HEIGHT)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    let width = ui.available_width();
                    let col_width = width / COLUMNS as f32;
                    eframe::egui::Grid::new("export_table")
                        .striped(true)
                        .spacing(SPACING)
                        .num_columns(COLUMNS)
                        .min_col_width(col_width)
                        .show(ui, |ui| {
                            ui.label("🔍");
                            ui.allocate_ui(
                                eframe::egui::vec2(200.0, ui.spacing().interact_size.y),
                                |ui| {
                                    ui.text_edit_singleline(
                                        &mut self.sub_window_manager.export_message.search_string,
                                    );
                                },
                            );
                            ui.end_row();
                            ui.allocate_ui(DESIGN_SIZE_FUNC_NAME, |ui| {
                                ui.strong(i18n::EXPORT_FUNCTION_NAME);
                            });
                            ui.allocate_ui(DESIGN_SIZE_FUNC_ADDR, |ui| {
                                ui.strong(i18n::EXPORT_FUNCTION_VIRTUAL_ADDRESS);
                            });
                            ui.allocate_ui(DESIGN_SIZE_FUNC_OPERATE, |ui| {
                                ui.strong(i18n::EXPORT_OPERATION);
                            });
                            ui.end_row();

                            for (index, item) in export_data_clone.0.borrow().iter().enumerate() {
                                if !search(
                                    &item.name,
                                    &self.sub_window_manager.export_message.search_string,
                                ) {
                                    continue;
                                }
                                ui.allocate_ui(DESIGN_SIZE_FUNC_NAME, |ui| {
                                    let display_name = if item.name.len() > MAX_FUNC_NAME_LENGTH {
                                        format!("{}...", &item.name[..MAX_FUNC_NAME_LENGTH - 3])
                                    } else {
                                        item.name.clone()
                                    };
                                    ui.label(display_name);
                                });

                                ui.allocate_ui(DESIGN_SIZE_FUNC_ADDR, |ui| {
                                    let addr_display = format!("0x{:X}", item.function);
                                    ui.label(addr_display);
                                });

                                ui.allocate_ui(DESIGN_SIZE_FUNC_OPERATE, |ui| {
                                    ui.horizontal(|ui| {
                                        if ui.button(i18n::EXPORT_DETAIL_BUTTON).clicked() {
                                            self.sub_window_manager
                                                .export_message
                                                .selected_export_index = Some(index);
                                        }
                                    });
                                });
                                ui.end_row();
                            }
                        });
                });
        });

        if let Some(selected_index) = selected_index {
            let mut export_table_ref = self.files[self.current_index].export.0.borrow_mut();
            if selected_index < export_table_ref.len() {
                eframe::egui::TopBottomPanel::bottom("export_detail_window").show(ui.ctx(), |ui| {
                    ui.label(i18n::EXPORT_FUNCTION_DETAILS);
                    ui.horizontal(|ui| {
                        ui.label(i18n::FUNCTION_NAME);
                        ui.text_edit_singleline(&mut export_table_ref[selected_index].name);
                        ui.label(i18n::TARGET_VIRTUAL_ADDRESS);

                        // 将 u32 地址转换为字符串进行编辑
                        let mut addr_string =
                            format!("0x{:X}", export_table_ref[selected_index].function);
                        if ui.text_edit_singleline(&mut addr_string).changed() {
                            // 尝试将用户输入的字符串转换回 u32
                            if let Ok(addr) =
                                u32::from_str_radix(addr_string.trim_start_matches("0x"), 16)
                            {
                                export_table_ref[selected_index].function = addr;
                                self.sub_window_manager.show_success(i18n::ADDRESS_UPDATED);
                            } else {
                                self.sub_window_manager
                                    .show_error(i18n::INVALID_HEX_ADDRESS_FORMAT);
                            }
                        }
                        if ui.button("X").clicked() {
                            self.sub_window_manager.export_message.selected_export_index = None;
                        }
                    });
                });
            }
        }
    }

    fn get_export(&mut self) -> anyhow::Result<ExportTable> {
        if self
            .files
            .get(self.current_index)
            .unwrap()
            .export
            .0
            .borrow()
            .is_empty()
            && let Some(file) = self.files.get_mut(self.current_index)
        {
            file.export = file.get_export()?;
        }
        Ok(self
            .files
            .get_mut(self.current_index)
            .unwrap()
            .export
            .fclone())
    }
}
