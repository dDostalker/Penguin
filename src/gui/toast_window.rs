use crate::gui::{SubWindowManager, Toast, ToastType};
use eframe::egui::Context;
use std::time::{Duration, Instant};
impl SubWindowManager {
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            ..Default::default()
        }
    }
    /// 清空所有数据
    pub fn clear_all_data(&mut self) {
        self.selected_export_index = None;
        self.select_dll_index = None;
        self.select_function_index = None;
        self.selected_section_index = None;
    }

    /// 显示通用信息窗口
    pub fn show_info_window(&mut self, ctx: &Context, title: &str, content: &str, window_id: &str) {
        eframe::egui::Window::new(title)
            .id(eframe::egui::Id::new(window_id))
            .collapsible(false)
            .resizable(true)
            .default_size([400.0, 300.0])
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(content);
                    ui.add_space(20.0);

                    if ui.button("关闭").clicked() {
                        // 根据窗口ID关闭对应的窗口
                        match window_id {
                            "about" => self.show_about_window = false,
                            "settings" => self.show_settings_window = false,
                            "help" => self.show_help_window = false,
                            _ => {}
                        }
                    }
                });
            });
    }

    /// 显示确认对话框
    pub fn show_confirm_dialog(
        &mut self,
        ctx: &Context,
        title: &str,
        message: &str,
        on_confirm: impl FnOnce(),
    ) {
        eframe::egui::Window::new(title)
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(message);
                    ui.add_space(20.0);

                    ui.horizontal(|ui: &mut eframe::egui::Ui| {
                        if ui.button("确认").clicked() {
                            on_confirm();
                        }

                        if ui.button("取消").clicked() {
                            // 关闭窗口
                        }
                    });
                });
            });
    }

    /// 显示关于窗口
    pub fn show_about_window(&mut self, ctx: &Context) {
        if self.show_about_window {
            eframe::egui::Window::new("关于 Penguin")
                .collapsible(false)
                .resizable(false)
                .default_size([400.0, 300.0])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("Penguin PE 分析器");
                        ui.add_space(10.0);
                        ui.label("版本: 0.1.0");
                        ui.label("作者: dDostalker");
                        ui.label("描述: 一个强大的PE文件分析工具");
                        ui.add_space(20.0);

                        ui.horizontal(|ui| {
                            if ui.button("确定").clicked() {
                                self.show_about_window = false;
                            }
                        });
                    });
                });
        }
    }

    /// 显示设置窗口
    pub fn show_settings_window(&mut self, ctx: &Context) {
        if self.show_settings_window {
            eframe::egui::Window::new("设置")
                .collapsible(false)
                .resizable(true)
                .default_size([500.0, 400.0])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("应用程序设置");
                        ui.add_space(20.0);

                        // 主题设置

                        if ui.button("演示通知").clicked() {
                            self.demo_toasts();
                        }

                        if ui.button("取消").clicked() {
                            self.show_settings_window = false;
                        }
                    });
                });
        }
    }

    /// 显示帮助窗口
    pub fn show_help_window(&mut self, ctx: &Context) {
        if self.show_help_window {
            eframe::egui::Window::new("帮助")
                .collapsible(false)
                .resizable(true)
                .default_size([600.0, 500.0])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("使用帮助");
                        ui.add_space(20.0);

                        // ui.label("基本操作:");
                        // ui.label("• 点击左侧面板选择要分析的文件");
                        // ui.label("• 使用顶部标签页切换不同的分析视图");
                        // ui.label("• 在导出表中点击'详情'按钮查看详细信息");
                        // ui.label("• 使用'编辑'按钮修改函数信息");

                        // ui.add_space(20.0);

                        // ui.label("快捷键:");
                        // ui.label("• Ctrl+O: 打开文件");
                        // ui.label("• Ctrl+S: 保存文件");
                        // ui.label("• F1: 显示帮助");

                        ui.add_space(20.0);

                        ui.horizontal(|ui| {
                            if ui.button("关闭").clicked() {
                                self.show_help_window = false;
                            }
                        });
                    });
                });
        }
    }

    /// 添加 toast 通知
    pub fn add_toast(&mut self, message: String, toast_type: ToastType) {
        let toast = Toast {
            message,
            toast_type,
            created_at: None,
            duration: Duration::from_secs(1), // 默认显示3秒
        };
        self.toasts.push(toast);
    }

    /// 添加成功通知
    pub fn show_success(&mut self, message: &str) {
        self.add_toast(message.to_string(), ToastType::Success);
    }

    /// 添加错误通知
    pub fn show_error(&mut self, message: &str) {
        self.add_toast(message.to_string(), ToastType::Error);
    }

    /// 添加警告通知
    pub fn show_warning(&mut self, message: &str) {
        self.add_toast(message.to_string(), ToastType::Warning);
    }

    /// 添加信息通知
    pub fn show_info(&mut self, message: &str) {
        self.add_toast(message.to_string(), ToastType::Info);
    }

    /// 渲染所有 toast 通知
    pub fn render_toasts(&mut self, ctx: &Context) {
        // 清理过期的 toast
        let now = Instant::now();
        self.toasts.retain(|toast| {
            if let Some(created_at) = toast.created_at {
                return now.duration_since(created_at) < toast.duration;
            } else {
                return true;
            }
        });

        // 渲染 toast
        let mut y_offset = 50.0;
        for (index, toast) in self.toasts.iter_mut().enumerate() {
            if index > 10 {
                break;
            }
            let mut progress = 0f32;
            if index == 0 && toast.created_at.is_none() {
                toast.created_at = Some(Instant::now());
            }
            if let Some(created_at) = toast.created_at {
                let elapsed = now.duration_since(created_at);
                progress = elapsed.as_secs_f32() / toast.duration.as_secs_f32();
            }
            // 计算透明度（淡出效果）
            let alpha = if progress > 0.8 {
                1.0 - (progress - 0.8) * 5.0
            } else {
                1.0
            };

            // 根据类型设置颜色
            let color = eframe::egui::Color32::from_rgb(54, 59, 64);

            // 设置图标
            let icon = match toast.toast_type {
                ToastType::Success => "✅",
                ToastType::Error => "❌",
                ToastType::Warning => "⚠️",
                ToastType::Info => "ℹ️",
            };

            eframe::egui::Area::new(eframe::egui::Id::new(format!("toast_{}", index)))
                .fixed_pos(eframe::egui::pos2(
                    ctx.available_rect().right() - 320.0,
                    y_offset,
                ))
                .show(ctx, |ui| {
                    eframe::egui::Frame::default()
                        .fill(color.linear_multiply(alpha))
                        .stroke(eframe::egui::Stroke::new(1.0, color.linear_multiply(alpha)))
                        .corner_radius(2.0)
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.label(
                                    eframe::egui::RichText::new(icon)
                                        .color(eframe::egui::Color32::WHITE)
                                        .size(16.0),
                                );
                                ui.label(
                                    eframe::egui::RichText::new(&toast.message)
                                        .color(eframe::egui::Color32::WHITE)
                                        .size(14.0),
                                );
                            });
                        });
                });

            y_offset += 35.0;
        }
    }

    /// 演示所有类型的 toast 通知
    pub fn demo_toasts(&mut self) {
        self.show_success("操作成功完成！");
        self.show_error("发生了一个错误");
        self.show_warning("请注意这个警告");
        self.show_info("这是一条信息提示");
    }
}
