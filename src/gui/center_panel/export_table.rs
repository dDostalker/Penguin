use eframe::egui::{Ui, Vec2};

use crate::{GLOBAL_RT, gui::FileManager, tools_api::read_file::ExportTable};

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

        // 克隆数据以避免借用冲突
        let export_data_clone = export_data.fclone();
        let selected_index = self.sub_window_manager.export_message.selected_export_index;
        
        eframe::egui::CentralPanel::default().show(ui.ctx(), |ui| {
            eframe::egui::ScrollArea::vertical()
                .min_scrolled_height(MIN_SCROLLED_HEIGHT)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // 使用表格样式，填满整个宽度
                    eframe::egui::Grid::new("export_table")
                        .striped(true)
                        .spacing(SPACING)
                        .num_columns(COLUMNS)
                        .show(ui, |ui| {
                            // 表头 - 使用强化的样式
                            ui.allocate_ui(DESIGN_SIZE_FUNC_NAME, |ui| {
                                ui.strong("函数名");
                            });
                            ui.allocate_ui(DESIGN_SIZE_FUNC_ADDR, |ui| {
                                ui.strong("函数地址");
                            });
                            ui.allocate_ui(DESIGN_SIZE_FUNC_OPERATE, |ui| {
                                ui.strong("操作");
                            });
                            ui.end_row();

                            for (index, item) in export_data_clone.0.borrow().iter().enumerate() {
                                // 函数名列 - 占用50%宽度
                                ui.allocate_ui(DESIGN_SIZE_FUNC_NAME, |ui| {
                                    let display_name = if item.name.len() > MAX_FUNC_NAME_LENGTH {
                                        format!("{}...", &item.name[..MAX_FUNC_NAME_LENGTH - 3])
                                    } else {
                                        item.name.clone()
                                    };
                                    ui.label(display_name);
                                });

                                // 地址列 - 占用25%宽度
                                ui.allocate_ui(DESIGN_SIZE_FUNC_ADDR, |ui| {
                                    let addr_display = format!("0x{:X}", item.function);
                                    ui.label(addr_display);
                                });

                                // 操作列 - 占用25%宽度
                                ui.allocate_ui(DESIGN_SIZE_FUNC_OPERATE, |ui| {
                                    ui.horizontal(|ui| {
                                        if ui.button("详情").clicked() {
                                            self.sub_window_manager.export_message.selected_export_index =
                                                Some(index);
                                        }
                                    });
                                });
                                ui.end_row();
                            }
                        });
                });
        });
        
        // 在渲染循环外处理编辑逻辑
        if let Some(selected_index) = selected_index {
            let mut export_table_ref = self.files[self.current_index].export.0.borrow_mut();
            if selected_index < export_table_ref.len() {
                eframe::egui::TopBottomPanel::bottom("export_detail_window").show(ui.ctx(), |ui| {
                    ui.label("导出函数详情");
                    ui.horizontal(|ui| {
                        ui.label("函数名:");
                        ui.text_edit_singleline(
                            &mut export_table_ref[selected_index].name,
                        );
                        ui.label("目标地址:");

                        // 将 u32 地址转换为字符串进行编辑
                        let mut addr_string = format!(
                            "0x{:X}",
                            export_table_ref[selected_index].function
                        );
                        if ui.text_edit_singleline(&mut addr_string).changed() {
                            // 尝试将用户输入的字符串转换回 u32
                            if let Ok(addr) =
                                u32::from_str_radix(addr_string.trim_start_matches("0x"), 16)
                            {
                                export_table_ref[selected_index].function = addr;
                                self.sub_window_manager.show_success("地址已更新");
                            } else {
                                self.sub_window_manager.show_error("无效的十六进制地址格式");
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
        {
            if let Some(file) = self.files.get_mut(self.current_index) {
                file.export = GLOBAL_RT.block_on(file.get_export())?;
            }
        }
        Ok(self
            .files
            .get_mut(self.current_index)
            .unwrap()
            .export.fclone())
    }
}
