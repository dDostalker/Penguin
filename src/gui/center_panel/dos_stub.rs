use crate::gui::FileManager;
use crate::i18n;
use eframe::egui::Vec2;

const MIN_SCROLLED_HEIGHT: f32 = 400.0;
const CHUNK_SIZE: usize = 16;
const SPACING: Vec2 = Vec2::new(20.0, 8.0);
const COLUMNS: usize = 3;
impl FileManager {
    /// dos_stub windows
    pub(crate) fn dos_stub_panel(&self, ui: &mut eframe::egui::Ui) {
        eframe::egui::CentralPanel::default().show(ui.ctx(), |ui| {
            let stub = &self.files.get(self.current_index).unwrap().dos_stub.buffer;

            Self::show_main_title(ui, "DOS Stub");

            if stub.is_empty() {
                ui.label(i18n::NO_DOS_STUB);
                return;
            }

            eframe::egui::ScrollArea::vertical()
                .min_scrolled_height(MIN_SCROLLED_HEIGHT)
                .show(ui, |ui| {
                    // 使用表格样式显示十六进制数据
                    eframe::egui::Grid::new("dos_stub_grid")
                        .min_col_width(ui.ctx().used_size().x / COLUMNS as f32)
                        .striped(true)
                        .spacing(SPACING)
                        .num_columns(COLUMNS)
                        .show(ui, |ui| {
                            // 表头
                            ui.strong(i18n::OFFSET);
                            ui.strong(i18n::HEXADECIMAL);
                            ui.strong("ASCII");
                            ui.end_row();

                            // 行循环
                            for (row, chunk) in stub.chunks(CHUNK_SIZE).enumerate() {
                                let offset = row * CHUNK_SIZE;
                                let hex: String =
                                    chunk.iter().map(|b| format!("{:02X} ", b)).collect();
                                // 转义
                                let ascii: String = chunk
                                    .iter()
                                    .map(|b| {
                                        let c = *b as char;
                                        if c.is_ascii_graphic() { c } else { '.' }
                                    })
                                    .collect();

                                ui.monospace(format!("{:08X}:", offset));
                                ui.monospace(hex);
                                ui.monospace(ascii);
                                ui.end_row();
                            }
                        });
                });
        });
    }
}
