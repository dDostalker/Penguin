use eframe::egui::{Label, Ui, Vec2};

use crate::{GLOBAL_RT, gui::FileManager, tools_api::read_file::ExportTable};

impl FileManager {
    pub(crate) fn export_panel(&mut self, ui: &mut Ui) {
        // 预先获取数据，避免在渲染循环中重复调用
        let export_data = match self.get_export() {
            Ok(export) => export,
            Err(_) => {
                ui.add(Label::new("该文件无导出表"));
                return;
            }
        };

        // 创建数据副本以避免借用冲突
        let export_items: Vec<_> = export_data
            .0
            .iter()
            .map(|item| (item.name.clone(), item.function))
            .collect();
        // 使用整个可用空间
        eframe::egui::CentralPanel::default().show(ui.ctx(), |ui| {
            eframe::egui::ScrollArea::vertical()
                .min_scrolled_height(400.0)
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    // 使用表格样式，填满整个宽度
                    eframe::egui::Grid::new("export_table")
                        .striped(true)
                        .spacing([20.0, 8.0])
                        .num_columns(3)
                        .show(ui, |ui| {
                            // 表头 - 使用强化的样式
                            ui.allocate_ui(Vec2::new(400.0 * 0.5, 0.0), |ui| {
                                ui.strong("函数名");
                            });
                            ui.allocate_ui(Vec2::new(400.0 * 0.25, 0.0), |ui| {
                                ui.strong("函数地址");
                            });
                            ui.allocate_ui(Vec2::new(400.0 * 0.25, 0.0), |ui| {
                                ui.strong("操作");
                            });
                            ui.end_row();

                            for (index, (func_name, func_addr)) in export_items.iter().enumerate() {
                                // 函数名列 - 占用50%宽度
                                ui.allocate_ui(Vec2::new(400.0 * 0.5, 0.0), |ui| {
                                    let display_name = if func_name.len() > 70 {
                                        format!("{}...", &func_name[..67])
                                    } else {
                                        func_name.clone()
                                    };
                                    ui.label(display_name);
                                });

                                // 地址列 - 占用25%宽度
                                ui.allocate_ui(Vec2::new(400.0 * 0.25, 0.0), |ui| {
                                    let addr_display = format!("0x{:X}", func_addr);
                                    ui.label(addr_display);
                                });

                                // 操作列 - 占用25%宽度
                                ui.allocate_ui(Vec2::new(400.0 * 0.25, 0.0), |ui| {
                                    ui.horizontal(|ui| {
                                        if ui.button("详情").clicked() {
                                            self.sub_window_manager.selected_export_index =
                                                Some(index);
                                        }

                                        if ui.button("复制").clicked() {
                                            let info = format!(
                                                "函数名: {}\n地址: 0x{:X}",
                                                func_name, func_addr
                                            );
                                            ui.output_mut(|o| o.copied_text = info);
                                            self.sub_window_manager.show_info("已复制到剪贴板");
                                        }
                                    });
                                });

                                ui.end_row();
                            }
                        });
                });
        });
        if let Some(selected_index) = self.sub_window_manager.selected_export_index {
            eframe::egui::TopBottomPanel::bottom("export_detail_window").show(ui.ctx(), |ui| {
                ui.label("导出函数详情");
                ui.horizontal(|ui| {
                    ui.label("函数名:");
                    ui.text_edit_singleline(
                        &mut self.files[self.current_index].export.0[selected_index].name,
                    );
                    ui.label("目标地址:");

                    // 将 u32 地址转换为字符串进行编辑
                    let mut addr_string = format!(
                        "0x{:X}",
                        self.files[self.current_index].export.0[selected_index].function
                    );
                    if ui.text_edit_singleline(&mut addr_string).changed() {
                        // 尝试将用户输入的字符串转换回 u32
                        if let Ok(addr) =
                            u32::from_str_radix(addr_string.trim_start_matches("0x"), 16)
                        {
                            self.files[self.current_index].export.0[selected_index].function = addr;
                            self.sub_window_manager.show_success("地址已更新");
                        } else {
                            self.sub_window_manager.show_error("无效的十六进制地址格式");
                        }
                    }
                    if ui.button("X").clicked() {
                        self.sub_window_manager.selected_export_index = None;
                    }
                });
            });
        }
    }

    /// 回去修改为可以改变的
    fn get_export(&mut self) -> anyhow::Result<&mut ExportTable> {
        // 只在第一次加载时执行异步操作，后续使用缓存
        if self
            .files
            .get(self.current_index)
            .unwrap()
            .export
            .0
            .is_empty()
        {
            // 使用现有的runtime而不是每次都创建新的
            self.files.get_mut(self.current_index).unwrap().export =
                GLOBAL_RT.block_on(self.files.get(self.current_index).unwrap().get_export())?;
        }
        Ok(self
            .files
            .get_mut(self.current_index)
            .unwrap()
            .export
            .as_mut())
    }
}
