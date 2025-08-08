use crate::GLOBAL_RT;
use crate::gui::FileManager;
use crate::tools_api::{load_file_info, serde_pe::{save_to_file}};
use eframe::egui::Ui;
use rfd::FileDialog;
use std::path::PathBuf;
use crate::tools_api::write_file::copy_file;

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
                                            self.files.push(file_info);
                                        }
                                    }
                                    Err(e) => self.sub_window_manager.show_error(&e.to_string()),
                                }
                            }
                        }
                    }
                    if let Err(e) = self.save_file(ui){
                        self.sub_window_manager.show_error(&e.to_string());
                    }
                    if ui.button("exit").clicked() {
                        // 后续添加释放其他行为的功能
                        std::process::exit(0);
                    }
                });

                ui.menu_button("工具", |ui| {
                    if ui.button("设置").clicked() {
                        self.sub_window_manager.show_settings_window = true;
                    }
                    ui.menu_button("导出为...",|ui|{
                        if let Err(e) = self.save_serde(ui,"toml"){
                            self.sub_window_manager.show_error(&e.to_string());
                        }
                        if let Err(e) = self.save_serde(ui,"json"){
                            self.sub_window_manager.show_error(&e.to_string());
                        }
                    })

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
    fn save_serde(&mut self,ui:&mut Ui,file_type:&str)->anyhow::Result<()> {
        if ui.button(format!("保存为{}",file_type)).clicked() {
            let file_info = self.files.get_mut(self.current_index).ok_or(anyhow::anyhow!("文件不存在"))?;
            let file_path = FileDialog::new().set_file_name(format!(".{}",file_type)).save_file();
            if file_path.is_none() {
                return Err(anyhow::anyhow!("保存失败"));
            }
            if let Some(file_path) = file_path {
                GLOBAL_RT.block_on(save_to_file(file_info, &file_path, file_type))?;
                self.sub_window_manager.show_success("保存成功");
            }
        }
        Ok(())
    }
    fn save_file(&mut self,ui:&mut Ui)->anyhow::Result<()> {
        if ui.button("save").clicked() {
            // todo 迁移到tools_api
            let file_info = self.files.get_mut(self.current_index).ok_or(anyhow::anyhow!("文件不存在"))?;
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
                let mut file = file_info.get_mut_file()?;
                GLOBAL_RT.block_on(copy_file(&mut file, &file_path))?;
                self.sub_window_manager.show_success("备份成功");
                break;
            }
            let import_dll = GLOBAL_RT.block_on(file_info.get_imports())?;
            let import_dll_cmp = file_info.import_dll.fclone();
            if import_dll != import_dll_cmp {
                for (i, j) in import_dll.0.borrow().iter().zip(import_dll_cmp.0.borrow().iter()) {
                    if i != j {
                        for (k, l) in i.function_info.iter().zip(j.function_info.iter())
                        {
                            if k != l {
                                let mut f = file_info.get_mut_file()?;
                                    GLOBAL_RT.block_on(k.write_func_name(&mut f, &l.name))
                                ?;
                                self.sub_window_manager.show_success("修改导入表成功");
                            }
                        }
                    }
                }
            }

            // 检查当前导出表是否被修改
            let export_table = GLOBAL_RT
                .block_on(file_info.get_export())?;
            if export_table != file_info.export {
                let export_table_ref = export_table.0.borrow();
                for (i, j) in export_table_ref.iter().zip(file_info.export.0.borrow().iter()) {
                    if i != j {
                        let mut f = file_info.get_mut_file()?;
                        GLOBAL_RT.block_on(i.write_func_name(&mut f, &j.name))?;
                        self.sub_window_manager.show_success("修改导出表成功");
                            GLOBAL_RT
                            .block_on(i.write_func_address(&mut f, j.function))
                        ?;
                        self.sub_window_manager.show_success("修改导出表成功");
                    }
                }
            }
            self.sub_window_manager.show_success("保存成功");
        }
        Ok(())
    }

}
