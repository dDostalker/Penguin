use crate::gui::FileManager;
use eframe::egui::panel::Side;
use eframe::egui::{Color32, Frame, Label, RichText, Sense, SidePanel, Ui};

const LEFT_PANEL_WIDTH: f32 = 170.0;
const LEFT_PANEL_FILL_COLOR: Color32 = Color32::from_rgb(43, 45, 48);
const LEFT_PANEL_TEXT_COLOR: Color32 = Color32::from_rgb(114, 151, 88);
const LEFT_PANEL_BACKGROUND_HOVER_COLOR: Color32 = Color32::from_rgb(70, 70, 70);
const LEFT_PANEL_TEXT_SELECTED_COLOR: Color32 = Color32::from_rgb(234, 198, 118);

impl FileManager {
    pub(crate) fn left_label(&mut self, ctx: &eframe::egui::Context) {
        SidePanel::new(Side::Left, "left_panel")
            .frame(Frame::new().fill(LEFT_PANEL_FILL_COLOR))
            .min_width(LEFT_PANEL_WIDTH)
            .max_width(LEFT_PANEL_WIDTH)
            .show(ctx, |ui| {
                Self::scroll_area(self, ui);
            });
    }
    fn scroll_area(&mut self, ui: &mut Ui) {
        let mut files_to_drop = Vec::new();
        eframe::egui::ScrollArea::vertical().show(ui, |ui| {
            // 循环输出文件名
            for (i, file) in self.files.iter().enumerate() {
                let text_context;
                let mut color = LEFT_PANEL_FILL_COLOR;
                let file_name = if file.file.is_some() {
                    file.file_name.clone() + "🔒"
                } else {
                    file.file_name.clone()
                };
                // 判断状态设置颜色
                if i + 1 == self.hover_index {
                    color = LEFT_PANEL_BACKGROUND_HOVER_COLOR;
                    self.hover_index = 0;
                }
                if file == self.get_file() {
                    text_context = RichText::from(file_name).color(LEFT_PANEL_TEXT_SELECTED_COLOR);

                    color = LEFT_PANEL_BACKGROUND_HOVER_COLOR;
                } else {
                    text_context = RichText::from(file_name).color(LEFT_PANEL_TEXT_COLOR);
                }

                // 每一个文件名都是Frame
                Frame::new().fill(color).show(ui, |ui| {
                    // 让label占满整行并创建响应
                    let available = ui.available_width();
                    let label = Label::new(text_context).sense(Sense::click()).wrap();
                    let response = ui.add_sized(
                        [
                            available,
                            ui.text_style_height(&eframe::egui::TextStyle::Body),
                        ],
                        label,
                    );

                    if response.clicked() {
                        self.current_index = self.files.iter().position(|f| f == file).unwrap_or(0);
                        self.sub_window_manager.clear_data();
                    }
                    if response.hovered() {
                        self.hover_index = i + 1;
                    }
                    // 右键点击
                    if response.clicked_by(eframe::egui::PointerButton::Secondary) {
                        files_to_drop.push(i);
                    }
                });
            }
        });

        for &index in files_to_drop.iter().rev() {
            if index < self.files.len()
                && let Err(e) = self.files[index].lock_file()
            {
                self.sub_window_manager.show_error(&e.to_string());
            }
        }
    }
}
