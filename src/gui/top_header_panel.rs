use crate::GLOBAL_RT;
use crate::gui::FileManager;
use crate::tools_api::FileInfo;
use rfd::FileDialog;
use std::path::PathBuf;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

impl FileManager {
    pub(crate) fn top_label(&mut self, ctx: &eframe::egui::Context) {
        eframe::egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            eframe::egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("open").clicked() {
                        let files = FileDialog::new().pick_files();
                        if let Some(paths) = files {
                            for path in paths {
                                match load_file_info(path) {
                                    Ok(file_info) => {
                                        if !self.files.contains(&file_info) {
                                            self.files.push(*file_info);
                                        }
                                    }
                                    Err(e) => self.sub_window_manager.show_error(&e.to_string()),
                                }
                            }
                        }
                    }
                    if ui.button("save").clicked() {
                        // todo 迁移到tools_api
                        let file_info = match self.files.get(self.current_index) {
                            Some(file) => file,
                            None => {
                                self.sub_window_manager.show_error("文件不存在");
                                return;
                            }
                        };
                        // 文件 -> 保存 的逻辑
                        // 创建恢复文件
                        let mut file_path = PathBuf::from(&file_info.file_path);
                        let mut times = 0;
                        loop {
                            file_path.set_extension(format!("bak{}", times));
                            if file_path.exists() {
                                times += 1;
                                continue;
                            }
                            let mut file_bak =
                                GLOBAL_RT.block_on(File::create(&file_path)).unwrap();
                            // 复制原文件内容到备份文件
                            let mut orig_file = self.files[self.current_index].get_mut_file();
                            let mut buf = Vec::new();
                            GLOBAL_RT.block_on(orig_file.read_to_end(&mut buf)).unwrap();
                            GLOBAL_RT.block_on(file_bak.write_all(&buf)).unwrap();
                            break;
                        }
                        // 检查当前dos头是否被修改

                        // 检查当前nt头是否被修改

                        // 检查当前节表头是否被修改

                        // 检查当前导入表是否被修改(todo 这个unwrap没有处理)
                        let import_dll = match GLOBAL_RT.block_on(file_info.get_imports()) {
                            Ok(import_dll) => import_dll,
                            Err(_) => {
                                self.sub_window_manager.show_error("修改导入表失败");
                                return;
                            }
                        };
                        if import_dll != file_info.import_dll {
                            for (i, j) in import_dll.iter().zip(file_info.import_dll.iter()) {
                                if i != j {
                                    for (k, l) in i.function_info.iter().zip(j.function_info.iter())
                                    {
                                        if k != l {
                                            let mut f = file_info.get_mut_file();
                                            match GLOBAL_RT
                                                .block_on(k.write_func_name(&mut f, &l.name))
                                            {
                                                Ok(_) => {
                                                    self.sub_window_manager
                                                        .show_success("修改导入表成功");
                                                }
                                                Err(e) => {
                                                    self.sub_window_manager
                                                        .show_error(&e.to_string());
                                                    return;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }

                        // 检查当前导出表是否被修改
                        let export_table = match GLOBAL_RT
                            .block_on(self.files.get(self.current_index).unwrap().get_export())
                        {
                            Ok(export_table) => Box::from(export_table),
                            Err(_e) => {
                                self.sub_window_manager.show_error("修改导出表失败");
                                return;
                            }
                        };
                        if export_table != file_info.export {
                            for (i, j) in export_table.0.iter().zip(file_info.export.0.iter()) {
                                if i != j {
                                    let mut f = file_info.get_mut_file();
                                    match GLOBAL_RT.block_on(i.write_func_name(&mut f, &j.name)) {
                                        Ok(_) => {
                                            self.sub_window_manager.show_success("修改导出表成功");
                                        }
                                        Err(e) => {
                                            self.sub_window_manager.show_error(&e.to_string());
                                            return;
                                        }
                                    }
                                    match GLOBAL_RT
                                        .block_on(i.write_func_address(&mut f, j.function))
                                    {
                                        Ok(_) => {}
                                        Err(e) => {
                                            self.sub_window_manager.show_error(&e.to_string());
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                        self.sub_window_manager.show_success("保存成功");
                    }
                    if ui.button("exit").clicked() {
                        // 退出应用
                    }
                });

                ui.menu_button("工具", |ui| {
                    if ui.button("设置").clicked() {
                        self.sub_window_manager.show_settings_window = true;
                    }
                });

                ui.menu_button("帮助", |ui| {
                    if ui.button("使用帮助").clicked() {
                        self.sub_window_manager.show_help_window = true;
                    }
                    if ui.button("关于").clicked() {
                        self.sub_window_manager.show_about_window = true;
                    }
                });
            });
        });
    }
}
pub(crate) fn load_file_info(path: PathBuf) -> anyhow::Result<Box<FileInfo>> {
    GLOBAL_RT.block_on(FileInfo::new(path))
}
