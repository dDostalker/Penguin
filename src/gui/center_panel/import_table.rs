use crate::tools_api::file_system::{self, get_dll_folder};
use crate::tools_api::read_file::ImportDll;

use crate::DANGEROUS_FUNCTION_TOML_PATH;
use crate::tools_api::read_file::ImportTable;
use crate::{gui::FileManager, i18n, tools_api::search};
use eframe::egui::{Color32, RichText, ScrollArea, Ui, Vec2};
use std::path::PathBuf;

const MIN_SCROLLED_HEIGHT: f32 = 400.0;
const SPACING: Vec2 = Vec2::new(20.0, 8.0);
const COLUMNS: usize = 3;
const MAX_DLL_NAME_LENGTH: usize = 20;

impl FileManager {
    fn truncate_text(text: &str, max_length: usize) -> String {
        if text.len() <= max_length {
            text.to_string()
        } else {
            format!("{}...", &text[..max_length.saturating_sub(3)])
        }
    }
    pub(crate) fn import_table_panel(&mut self, ui: &mut Ui) -> anyhow::Result<()> {
        let imports = match self.import_dll() {
            Ok(imports) => imports,
            Err(_e) => {
                return Ok(());
            }
        };

        let imports_clone = imports.fclone();
        let selected_index = self.sub_window_manager.import_message.selected_dll_index;
        let selected_function_index = self
            .sub_window_manager
            .import_message
            .selected_function_index;

        eframe::egui::CentralPanel::default().show(ui.ctx(), |ui| {
            Self::show_main_title(ui, "Import");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(i18n::DLL_LIST);
                    self.show_dll_table(ui, &imports_clone.0.borrow());
                });

                ui.separator();
                ui.vertical(|ui| {
                    ui.label(i18n::FUNCTION_LIST);
                    if let Some(selected_index) = selected_index {
                        if let Some(selected_dll) = imports_clone.0.borrow().get(selected_index) {
                            self.show_function_table(ui, selected_dll);
                        } else {
                            ui.label(i18n::SELECT_DLL_PROMPT);
                        }
                    } else {
                        ui.label(i18n::SELECT_DLL_PROMPT);
                    }
                });
            });
        });
        if let Some(selected_index) = selected_index
            && let Some(selected_function_index) = selected_function_index
        {
            eframe::egui::TopBottomPanel::bottom("export_detail_window").show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label(i18n::FUNCTION_DETAILS);
                    let mut import_dll = self.files[self.current_index].import_dll.0.borrow_mut();
                    ui.horizontal(|ui| {
                        ui.label(i18n::FUNCTION_NAME);
                        ui.text_edit_singleline(
                            &mut import_dll[selected_index].function_info[selected_function_index]
                                .name,
                        );
                        if ui.button("X").clicked() {
                            self.sub_window_manager
                                .import_message
                                .selected_function_index = None;
                        }
                    });
                });
            });
        }
        Ok(())
    }

    /// 获取导入表的引用
    pub(crate) fn import_dll(&mut self) -> anyhow::Result<ImportTable> {
        let file = self.files.get_mut(self.current_index).unwrap();
        if file.import_dll.0.borrow().is_empty() {
            file.import_dll = file.get_imports()?;
        }
        Ok(file.import_dll.fclone())
    }

    fn show_dll_table(&mut self, ui: &mut Ui, imports: &[ImportDll]) {
        ScrollArea::vertical()
            .id_salt("dll_table")
            .min_scrolled_height(MIN_SCROLLED_HEIGHT)
            .show(ui, |ui| {
                let width = ui.available_width();
                let col_width = width / (2 * COLUMNS) as f32;

                eframe::egui::Grid::new("dll_table")
                    .striped(true)
                    .spacing(SPACING)
                    .num_columns(COLUMNS)
                    .min_col_width(col_width)
                    .show(ui, |ui| {
                        ui.strong(i18n::DLL_NAME);
                        ui.strong(i18n::FUNCTION_COUNT);
                        ui.strong(i18n::OPERATION);
                        ui.end_row();

                        for (index, dll) in imports.iter().enumerate() {
                            let truncated_dll_name =
                                Self::truncate_text(&dll.name, MAX_DLL_NAME_LENGTH);
                            ui.label(&truncated_dll_name);

                            ui.label(format!("{}", dll.function_info.len()));

                            ui.horizontal(|ui| {
                                if ui.button(i18n::SELECT_BUTTON).clicked() {
                                    self.sub_window_manager.import_message.selected_dll_index =
                                        Some(index);
                                    self.sub_window_manager
                                        .import_message
                                        .selected_function_index = None;
                                }
                                if ui.button(i18n::OPEN_LOCATION).clicked() {
                                    let dll_folder = get_dll_folder(
                                        PathBuf::from(&self.files[self.current_index].file_path),
                                        &dll.name,
                                    )
                                    .unwrap();
                                    if let Err(e) = file_system::open_file_location(&dll_folder) {
                                        self.sub_window_manager.show_error(&e.to_string());
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
                let width = ui.available_width();
                let col_width = width / (2 * COLUMNS) as f32;
                eframe::egui::Grid::new("function_table")
                    .striped(true)
                    .spacing(SPACING)
                    .num_columns(COLUMNS)
                    .min_col_width(col_width)
                    .show(ui, |ui| {
                        ui.label("🔍");
                        ui.text_edit_singleline(
                            &mut self.sub_window_manager.import_message.search_string,
                        );
                        ui.end_row();
                        ui.strong(i18n::SEQUENCE_NUMBER);
                        ui.strong(i18n::FUNCTION_NAME);
                        ui.strong(i18n::OPERATION);
                        ui.end_row();
                        let (er, eg, eb) = (
                            DANGEROUS_FUNCTION_TOML_PATH
                                .danger_color
                                .as_ref()
                                .unwrap()
                                .r,
                            DANGEROUS_FUNCTION_TOML_PATH
                                .danger_color
                                .as_ref()
                                .unwrap()
                                .g,
                            DANGEROUS_FUNCTION_TOML_PATH
                                .danger_color
                                .as_ref()
                                .unwrap()
                                .b,
                        );

                        let (wr, wg, wb) = (
                            DANGEROUS_FUNCTION_TOML_PATH
                                .warning_color
                                .as_ref()
                                .unwrap()
                                .r,
                            DANGEROUS_FUNCTION_TOML_PATH
                                .warning_color
                                .as_ref()
                                .unwrap()
                                .g,
                            DANGEROUS_FUNCTION_TOML_PATH
                                .warning_color
                                .as_ref()
                                .unwrap()
                                .b,
                        );
                        for (index, function) in dll.function_info.iter().enumerate() {
                            if !search(
                                &function.name,
                                &self.sub_window_manager.import_message.search_string,
                            ) {
                                continue;
                            }
                            ui.label(format!("{}", index + 1));
                            let name_color = if DANGEROUS_FUNCTION_TOML_PATH
                                .dangerous
                                .contains(&function.name)
                            {
                                Color32::from_rgb(er, eg, eb)
                            } else if DANGEROUS_FUNCTION_TOML_PATH
                                .warning
                                .contains(&function.name)
                            {
                                Color32::from_rgb(wr, wg, wb)
                            } else {
                                Color32::GRAY
                            };
                            let truncated_function_name = Self::truncate_text(&function.name, 40);
                            ui.label(RichText::new(&truncated_function_name).color(name_color));

                            ui.horizontal(|ui| {
                                if ui.button(i18n::DETAIL_BUTTON).clicked() {
                                    self.sub_window_manager
                                        .import_message
                                        .selected_function_index = Some(index);
                                }
                            });
                            ui.end_row();
                        }
                    });
            });
    }
}
