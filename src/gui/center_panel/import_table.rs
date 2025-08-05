use crate::tools_api::file_system::{self, get_dll_folder};
use crate::tools_api::read_file::ImportDll;
use crate::{GLOBAL_RT, gui::FileManager};
use eframe::egui::{ScrollArea, Ui};
use std::path::PathBuf;
const MIN_SCROLLED_HEIGHT: f32 = 400.0;
impl FileManager {
    /// 截断文本到指定长度，超出部分用省略号表示
    fn truncate_text(text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            text.to_string()
        } else {
            format!("{}...", &text[..max_length.saturating_sub(3)])
        }
    }
    /// 导入表主面板
    pub(crate) fn import_table_panel(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
        // 预先获取数据，避免在渲染循环中重复调用
        let imports = match self.import_dll_mut() {
            Ok(imports) => imports,
            Err(_) => {
                return Ok(());
            }
        };

        // 克隆数据以避免借用冲突
        let imports_clone = imports.clone();
        let selected_index = self.sub_window_manager.select_dll_index;
        let selected_function_index = self.sub_window_manager.select_function_index;

        // 显示主标题

        eframe::egui::CentralPanel::default().show(ui.ctx(), |ui| {
            Self::show_main_title(ui, "Import");
            // 创建左右并排的布局
            ui.horizontal(|ui| {
                // 左侧表格：DLL列表
                ui.vertical(|ui| {
                    ui.label("DLL列表");
                    self.show_dll_table(ui, &imports_clone);
                });

                // 添加分隔线
                ui.separator();

                // 右侧表格：函数列表
                ui.vertical(|ui| {
                    ui.label("函数列表");
                    if let Some(selected_index) = selected_index {
                        if let Some(selected_dll) = imports_clone.get(selected_index) {
                            self.show_function_table(ui, selected_dll);
                        } else {
                            ui.label("请选择一个DLL查看其函数");
                        }
                    } else {
                        ui.label("请选择一个DLL查看其函数");
                    }
                });
            });
        });
        // 下方功能栏
        if let Some(selected_index) = selected_index
            && let Some(selected_function_index) = selected_function_index
        {
            eframe::egui::TopBottomPanel::bottom("export_detail_window").show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("导出函数详情");
                    ui.horizontal(|ui| {
                        ui.label("函数名:");
                        ui.text_edit_singleline(
                            &mut self.files[self.current_index].import_dll[selected_index]
                                .function_info[selected_function_index]
                                .name,
                        );
                        if ui.button("X").clicked() {
                            self.sub_window_manager.select_function_index = None;
                        }
                    });
                });
            });
        }
        Ok(())
    }

    pub(crate) fn import_dll_mut(&mut self) -> anyhow::Result<&mut Vec<ImportDll>> {
        if self
            .files
            .get(self.current_index)
            .unwrap()
            .import_dll
            .is_empty()
        {
            self.files.get_mut(self.current_index).unwrap().import_dll =
                GLOBAL_RT.block_on(self.files.get(self.current_index).unwrap().get_imports())?;
        }
        Ok(&mut self.files.get_mut(self.current_index).unwrap().import_dll)
    }

    fn show_dll_table(&mut self, ui: &mut Ui, imports: &[ImportDll]) {
        ScrollArea::vertical()
            .id_salt("dll_table")
            .min_scrolled_height(MIN_SCROLLED_HEIGHT)
            .show(ui, |ui| {
                eframe::egui::Grid::new("dll_table")
                    .striped(true)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        // 表头
                        ui.strong("DLL名称");
                        ui.strong("函数数量");
                        ui.strong("操作");
                        ui.end_row();

                        for (index, dll) in imports.iter().enumerate() {
                            // DLL名称显示（限制最大30个字符）
                            let truncated_dll_name = Self::truncate_text(&dll.name, 30);
                            ui.label(&truncated_dll_name);

                            // 函数数量显示
                            ui.label(&format!("{}", dll.function_info.len()));

                            // 操作按钮
                            ui.horizontal(|ui| {
                                if ui.button("选择").clicked() {
                                    self.sub_window_manager.select_dll_index = Some(index);
                                }
                                if ui.button("复制").clicked() {
                                    let info = format!("DLL: {}", dll.name);
                                    ui.output_mut(|o| o.copied_text = info);
                                }
                                // 添加打开资源管理器按钮
                                if ui.button("打开位置").clicked() {
                                    let dll_folder = get_dll_folder(
                                        PathBuf::from(&self.files[self.current_index].file_path),
                                        &dll.name,
                                    )
                                    .unwrap();
                                    if let Err(e) = file_system::open_file_location(&dll_folder) {
                                        eprintln!("打开文件位置失败: {}", e);
                                    }
                                }
                            });
                            ui.end_row();
                        }
                    });
            });
    }

    fn show_function_table(&mut self, ui: &mut Ui, dll: &ImportDll) {
        ScrollArea::vertical()
            .id_salt("function_table")
            .min_scrolled_height(MIN_SCROLLED_HEIGHT)
            .show(ui, |ui| {
                eframe::egui::Grid::new("function_table")
                    .striped(true)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        // 表头
                        ui.strong("序号");
                        ui.strong("函数名");
                        ui.strong("操作");
                        ui.end_row();

                        for (index, function) in dll.function_info.iter().enumerate() {
                            // 序号显示
                            ui.label(&format!("{}", index + 1));

                            // 函数名显示（限制最大40个字符）
                            let truncated_function_name = Self::truncate_text(&function.name, 40);
                            ui.label(&truncated_function_name);

                            // 操作按钮
                            ui.horizontal(|ui| {
                                if ui.button("复制").clicked() {
                                    let info = format!("函数: {}", function.name);
                                    ui.output_mut(|o| o.copied_text = info);
                                }
                                if ui.button("详细").clicked() {
                                    self.sub_window_manager.select_function_index = Some(index);
                                }
                            });
                            ui.end_row();
                        }
                    });
            });
    }
}
