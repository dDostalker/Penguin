use crate::gui::FileManager;

const MIN_SCROLLED_HEIGHT: f32 = 400.0;
const CHUNK_SIZE: usize = 16;
impl FileManager {
    /// dos_stub 窗口
    pub(crate) fn dos_stub_panel(&self, ui: &mut eframe::egui::Ui) {
        eframe::egui::CentralPanel::default().show(ui.ctx(), |ui| {
            let stub = &self.files.get(self.current_index).unwrap().dos_stub.buffer;

            Self::show_main_title(ui, "DOS Stub");

            if stub.is_empty() {
                ui.label("该文件无 DOS Stub 数据");
                return;
            }

            eframe::egui::ScrollArea::vertical()
                .min_scrolled_height(MIN_SCROLLED_HEIGHT)
                .show(ui, |ui| {
                    // 使用表格样式显示十六进制数据
                    eframe::egui::Grid::new("dos_stub_grid")
                        .striped(true)
                        .spacing([10.0, 2.0])
                        .show(ui, |ui| {
                            // 表头
                            ui.strong("偏移");
                            ui.strong("十六进制");
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
