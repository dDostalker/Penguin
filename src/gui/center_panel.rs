mod dos_header;
mod dos_stub;
mod export_table;
mod import_table;
mod nt_header;
mod section;

use crate::gui::{show_error_message, FileManager, Page};
use crate::tools_api::file_system::{open_explorer, open_file_location};
use crate::tools_api::structure::FileInfo;
use eframe::egui::{Area, CentralPanel, Color32, Frame, Id, Ui};

impl FileManager {
    /// 信息栏一级标题
    fn show_main_title(ui: &mut Ui, title_name: &str) {
        ui.label(
            eframe::egui::RichText::new(title_name)
                .heading()
                .strong()
                .color(Color32::from_rgb(255, 255, 255))
                .size(32.0) // 设置更大字号
                .strong(),
        );
    }
    /// 信息栏二级标题
    fn show_sub_title(ui: &mut Ui, title_name: &str) {
        ui.label(
            eframe::egui::RichText::new(title_name)
                .heading()
                .color(Color32::from_rgb(255, 255, 255))
                .strong(),
        );
    }
    /// center底部信息
    fn show_bottom_panel(file: &FileInfo, ctx: &eframe::egui::Context) {
        eframe::egui::TopBottomPanel::bottom("bottom_panel")
            .frame(Frame::new().fill(Color32::from_rgb(43, 45, 48)))
            .show(ctx, |ui| {
                ui.label(format!("File Name: {:?}", file.file_name));
                ui.horizontal(|ui| {
                    ui.label(format!("File Path: {:?}", file.file_path));
                    if ui.button("jump").clicked() {
                        if let Err(e) = open_file_location(&file.file_path) {
                            show_error_message(ctx, &e.to_string());
                        }
                    }
                });
                ui.label(format!("File Size: {:?}", file.file_size));
                ui.label(format!("File Modified Time: {:?}", file.file_hash));
            });
    }
    /// 主窗口中心
    pub(crate) fn center(&mut self, ctx: &eframe::egui::Context) {
        // 拖拽文件检测和处理
        let dropped_files = ctx.input(|i| i.raw.dropped_files.clone());
        for file in dropped_files {
            if let Some(path) = &file.path {
                match crate::gui::top_header_panel::load_file_info(path.clone()) {
                    Ok(file_info) => {
                        if !self.files.contains(&file_info) {
                            self.files.push(*file_info);
                        }
                    }
                    Err(e) => show_error_message(ctx, &e.to_string()),
                }
            }
        }

        CentralPanel::default()
            .frame(Frame::new().fill(Color32::from_rgb(30, 31, 34)))
            .show(ctx, |_ui| {
                if let Some(file) = self.files.get(self.current_index) {
                    Self::show_bottom_panel(file, ctx);
                    CentralPanel::default()
                        .frame(Frame::new().fill(Color32::from_rgb(30, 31, 34)))
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
                                    Page::DosHead => self.dos_header_panel(ui),
                                    Page::DosStub => self.dos_stub_panel(ui),
                                    Page::NtHead => self.nt_header_panel(ui),
                                    Page::SectionHead => self.section_header_panel(ui),
                                    Page::Import => self.import_table_panel(ui).unwrap(),
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
