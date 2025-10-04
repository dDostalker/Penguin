mod dos_header;
mod dos_stub;
mod export_table;
mod import_table;
mod nt_header;
mod section;
use crate::gui::FileManager;
use crate::i18n;
use crate::tools_api::calc::{calc_md5, calc_sha1};
use crate::tools_api::file_system::open_file_location;
use crate::tools_api::load_file_info;
use crate::tools_api::{FileInfo, HashInfo, Page};
use eframe::egui::{Area, CentralPanel, Color32, Context, Frame, Id, RichText, Ui};

const CENTER_PANEL_FILL_COLOR: Color32 = Color32::from_rgb(30, 31, 34);
const CENTER_PANEL_BOTTOM_FILL_COLOR: Color32 = Color32::from_rgb(43, 45, 48);
const CENTER_PANEL_TITLE_COLOR: Color32 = Color32::from_rgb(255, 255, 255);
const CENTER_PANEL_TITLE_SIZE: f32 = 32.0;

impl FileManager {
    /// 信息栏一级标题
    fn show_main_title(ui: &mut Ui, title_name: &str) {
        ui.label(
            RichText::new(title_name)
                .heading()
                .strong()
                .color(CENTER_PANEL_TITLE_COLOR)
                .size(CENTER_PANEL_TITLE_SIZE) // 设置更大字号
                .strong(),
        );
    }
    /// 信息栏二级标题
    fn show_sub_title(ui: &mut Ui, title_name: &str) {
        ui.label(
            RichText::new(title_name)
                .heading()
                .color(CENTER_PANEL_TITLE_COLOR)
                .strong(),
        );
    }
    /// center底部信息
    fn show_bottom_panel(file: &mut FileInfo, ctx: &Context) -> anyhow::Result<()> {
        eframe::egui::TopBottomPanel::bottom("bottom_panel")
            .frame(Frame::new().fill(CENTER_PANEL_BOTTOM_FILL_COLOR))
            .show(ctx, |ui| {
                ui.label(format!("File Name: {:?}", file.file_name));
                ui.horizontal(|ui| -> anyhow::Result<()> {
                    ui.label(format!("File Path: {:?}", file.file_path));
                    if ui.button(i18n::JUMP).clicked() {
                        open_file_location(&file.file_path)?;
                    }
                    Ok(())
                });
                ui.label(format!("File Size: {}B", file.file_size));
                if file.file_hash.is_none() {
                    file.file_hash = Some(HashInfo {
                        md5: calc_md5(&file.file_path),
                        sha1: calc_sha1(&file.file_path),
                    });
                }
                if let Some(file_hash) = &file.file_hash {
                    ui.horizontal(|ui| {
                        ui.label(format!("File MD5: {}", file_hash.md5));
                        ui.label(format!("File SHA1: {}", file_hash.sha1));
                    });
                }
            });
        Ok(())
    }
    /// 主窗口中心
    pub(crate) fn center(&mut self, ctx: &Context) {
        // 拖拽文件检测和处理
        let dropped_files = ctx.input(|i| i.raw.dropped_files.clone());
        for file in dropped_files {
            if let Some(path) = &file.path {
                match load_file_info(path.clone()) {
                    Ok(file_info) => {
                        if !self.files.contains(&file_info) {
                            self.files.push(file_info);
                        }
                    }
                    Err(e) => self.sub_window_manager.show_error(&e.to_string()),
                }
            }
        }

        CentralPanel::default()
            .frame(Frame::new().fill(CENTER_PANEL_FILL_COLOR))
            .show(ctx, |_ui| {
                if let Some(file) = self.files.get_mut(self.current_index) {
                    if let Err(e) = Self::show_bottom_panel(file, ctx) {
                        self.sub_window_manager.show_error(&e.to_string());
                    }
                    CentralPanel::default()
                        .frame(Frame::new().fill(CENTER_PANEL_FILL_COLOR))
                        .show(ctx, |_ui| {
                            Area::new(Id::from("area")).movable(false).show(ctx, |ui| {
                                eframe::egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
                                    ui.horizontal(|ui| {
                                        if ui.button("Dos head").clicked() {
                                            self.page = Page::DosHead
                                        }
                                        if ui.button("Dos stub").clicked() {
                                            self.page = Page::DosStub
                                        }
                                        if ui.button("Nt header").clicked() {
                                            self.page = Page::NtHead
                                        }
                                        if ui.button("Section header").clicked() {
                                            self.page = Page::SectionHead
                                        }
                                        if ui.button("Import table").clicked() {
                                            self.page = Page::Import
                                        }
                                        if ui.button("Export table").clicked() {
                                            self.page = Page::Export
                                        }
                                    });
                                });
                                match self.page {
                                    Page::DosHead => {
                                        if let Err(e) = self.dos_header_panel(ui) {
                                            self.sub_window_manager.show_error(&e.to_string());
                                        }
                                    }
                                    Page::DosStub => {
                                        self.dos_stub_panel(ui);
                                    }
                                    Page::NtHead => self.nt_header_panel(ui),
                                    Page::SectionHead => {
                                        if let Err(e) = self.section_header_panel(ui) {
                                            self.sub_window_manager.show_error(&e.to_string());
                                        }
                                    }
                                    Page::Import => {
                                        if let Err(e) = self.import_table_panel(ui) {
                                            self.sub_window_manager.show_error(&e.to_string());
                                        }
                                    }
                                    Page::Export => {
                                        self.export_panel(ui);
                                    }
                                }
                            });
                        });
                    // 这里可以添加更多具体的文件信息展示
                }
            });
    }
}
