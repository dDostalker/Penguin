use crate::gui::FileManager;
use crate::i18n;
use crate::tools_api::read_file::ResourceTree;
use crate::tools_api::write_file::copy_file;
use crate::tools_api::{load_file_info, serde_pe::save_to_file};
use eframe::egui::Ui;
use rfd::FileDialog;
use std::path::PathBuf;

impl FileManager {
    pub(crate) fn top_label(&mut self, ctx: &eframe::egui::Context) {
        eframe::egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            eframe::egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button(i18n::FILE_MENU, |ui| {
                    if ui.button(i18n::OPEN_BUTTON).clicked() {
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
                    if let Err(e) = self.save_file(ui) {
                        self.sub_window_manager.show_error(&e.to_string());
                    }
                    if ui.button(i18n::EXIT_BUTTON).clicked() {
                        std::process::exit(0);
                    }
                });

                ui.menu_button(i18n::TOOLS_MENU, |ui| {
                    if ui.button(i18n::SETTINGS_MENU).clicked() {
                        self.sub_window_manager.window_message.show_settings_window = true;
                    }
                    if ui
                        .button(i18n::VIRTUAL_ADDRESS_TO_FILE_OFFSET_MENU)
                        .clicked()
                    {
                        self.sub_window_manager
                            .window_message
                            .show_virtual_address_to_file_offset_window = true;
                    }
                    ui.menu_button(i18n::EXPORT_MENU, |ui| {
                        if let Err(e) = self.save_serde(ui, "toml") {
                            self.sub_window_manager.show_error(&e.to_string());
                        }
                        if let Err(e) = self.save_serde(ui, "json") {
                            self.sub_window_manager.show_error(&e.to_string());
                        }
                    });
                    if ui.button(i18n::EXTRACT_RESOURCE_MENU).clicked() {
                        let files = match self.files.get(self.current_index) {
                            Some(file) => file,
                            None => {
                                self.sub_window_manager.show_error(i18n::FILE_NOT_FOUND);
                                return;
                            }
                        };
                        let mut file = std::fs::File::open(&files.file_path).unwrap();
                        let resource_tree = match ResourceTree::get_resource_tree(
                            &mut file,
                            self.files[self.current_index]
                                .data_directory
                                .get_resource_directory_address()
                                .unwrap(),
                            &*self.files[self.current_index].nt_head,
                            &self.files[self.current_index].section_headers,
                            &self.files[self.current_index].data_directory,
                        ) {
                            Ok(resource_tree) => resource_tree,
                            Err(e) => {
                                self.sub_window_manager.show_error(&e.to_string());
                                return;
                            }
                        };
                        let output_path = FileDialog::new().pick_folder();
                        let output_path = match output_path {
                            Some(path) => path,
                            None => {
                                self.sub_window_manager.show_error(i18n::FILE_NOT_FOUND);
                                return;
                            }
                        };
                        match resource_tree.extract_resources(
                            &mut file,
                            &output_path,
                            &*self.files[self.current_index].nt_head,
                            &self.files[self.current_index].section_headers,
                            &self.files[self.current_index].data_directory,
                        ) {
                            Ok(extracted_files) => extracted_files,
                            Err(e) => {
                                self.sub_window_manager.show_error(&e.to_string());
                                return;
                            }
                        };
                    }
                });

                ui.menu_button(i18n::HELP_MENU, |ui| {
                    if ui.button(i18n::USAGE_HELP_MENU).clicked() {
                        self.sub_window_manager.window_message.show_help_window = true;
                    }
                    if ui.button(i18n::ABOUT_MENU).clicked() {
                        self.sub_window_manager.window_message.show_about_window = true;
                    }
                });
            });
        });
    }
    fn save_serde(&mut self, ui: &mut Ui, file_type: &str) -> anyhow::Result<()> {
        if ui
            .button(format!("{}", i18n::SAVE_AS_FORMAT.replace("{}", file_type)))
            .clicked()
        {
            let file_info = self
                .files
                .get_mut(self.current_index)
                .ok_or(anyhow::anyhow!(i18n::FILE_NOT_FOUND))?;
            let file_path = FileDialog::new()
                .set_file_name(format!(".{}", file_type))
                .save_file();
            if file_path.is_none() {
                return Err(anyhow::anyhow!(i18n::SAVE_FAILED));
            }
            if let Some(file_path) = file_path {
                save_to_file(file_info, &file_path, file_type)?;
                self.sub_window_manager.show_success(i18n::SAVE_SUCCESS);
            }
        }
        Ok(())
    }
    fn save_file(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
        if ui.button(i18n::SAVE_BUTTON).clicked() {
            // todo 迁移到tools_api
            let file_info = self
                .files
                .get_mut(self.current_index)
                .ok_or(anyhow::anyhow!(i18n::FILE_NOT_FOUND))?;
            let mut file_path = PathBuf::from(&file_info.file_path);
            let mut times = 0;
            loop {
                file_path.set_extension(format!("bak{}", times));
                if file_path.exists() {
                    times += 1;
                    continue;
                }
                let mut file = file_info.get_mut_file()?;
                copy_file(&mut file, &file_path)?;
                self.sub_window_manager.show_success(i18n::BACKUP_SUCCESS);
                break;
            }
            let import_dll = file_info.get_imports()?;
            let import_dll_cmp = file_info.import_dll.fclone();
            if import_dll != import_dll_cmp {
                for (i, j) in import_dll
                    .0
                    .borrow()
                    .iter()
                    .zip(import_dll_cmp.0.borrow().iter())
                {
                    if i != j {
                        for (k, l) in i.function_info.iter().zip(j.function_info.iter()) {
                            if k != l {
                                let mut f = file_info.get_mut_file()?;
                                k.write_func_name(&mut f, &l.name)?;
                                self.sub_window_manager
                                    .show_success(i18n::IMPORT_TABLE_MODIFIED);
                            }
                        }
                    }
                }
            }

            // 检查当前导出表是否被修改
            let export_table = file_info.get_export()?;
            if export_table != file_info.export {
                let export_table_ref = export_table.0.borrow();
                for (i, j) in export_table_ref
                    .iter()
                    .zip(file_info.export.0.borrow().iter())
                {
                    if i != j {
                        let mut f = file_info.get_mut_file()?;
                        i.write_func_name(&mut f, &j.name)?;
                        self.sub_window_manager
                            .show_success(i18n::EXPORT_TABLE_MODIFIED);
                        i.write_func_address(&mut f, j.function)?;
                        self.sub_window_manager
                            .show_success(i18n::EXPORT_TABLE_MODIFIED);
                    }
                }
            }
            self.sub_window_manager.show_success(i18n::SAVE_SUCCESS);
        }
        Ok(())
    }
}
