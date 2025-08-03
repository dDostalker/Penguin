use crate::gui::FileManager;
use eframe::egui::panel::Side;
use eframe::egui::{Color32, Frame, Label, RichText, Sense, SidePanel};

impl FileManager {
    pub(crate) fn left_label(&mut self, ctx: &eframe::egui::Context) {
        SidePanel::new(Side::Left, "left_panel")
            .frame(Frame::new().fill(Color32::from_rgb(43, 45, 48)))
            .min_width(170.0)
            .max_width(170.0)
            .show(ctx, |ui| {
                // 循环输出文件名
                for (i, file) in self.files.iter().enumerate() {
                    let text_context;
                    let mut color = Color32::from_rgb(43, 45, 48);

                    // 判断状态设置颜色
                    if i + 1 == self.hover_index {
                        // text_context = RichText::from(&file.file_name).color(Color32::from_rgb(114, 151, 88));
                        color = Color32::from_rgb(70, 70, 70);
                        self.hover_index = 0;
                    }
                    if file == self.get_file() {
                        text_context =
                            RichText::from(&file.file_name).color(Color32::from_rgb(234, 198, 118));

                        color = Color32::from_rgb(70, 70, 70);
                    } else {
                        text_context =
                            RichText::from(&file.file_name).color(Color32::from_rgb(114, 151, 88));
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
                            self.current_index =
                                self.files.iter().position(|f| f == file).unwrap_or(0);
                            // 清空其他数据

                            self.sub_window_manager.clear_all_data();
                        }
                        if response.hovered() {
                            self.hover_index = i + 1;
                        }
                    });
                }
            });
    }
}
