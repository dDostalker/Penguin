use crate::{gui::{SubWindowManager, Toast, ToastType}, tools_api::{parse_address_string, read_file::{nt_header::traits::NtHeaders, rva_2_fo, ImageSectionHeaders}}};

use eframe::egui::Context;
use std::time::{Duration, Instant};
const TOAST_WINDOW_WIDTH: f32 = 400.0;
const TOAST_WINDOW_HEIGHT: f32 = 300.0;
const TOAST_WINDOW_SPACING: f32 = 20.0;
const TOAST_WINDOW_BUTTON_SPACING: f32 = 10.0;
const TOAST_WINDOW_ICON_SIZE: f32 = 16.0;
const TOAST_WINDOW_TEXT_SIZE2: f32 = 14.0;
const TOAST_WINDOW_SIDE_OFFSET: f32 = 200.0;
const Y_OFFSET: f32 = 50.0;
const Y_OFFSET_STEP: f32 = 35.0;
const RGB_COLOR: eframe::egui::Color32 = eframe::egui::Color32::from_rgb(54, 59, 64);

impl SubWindowManager {
    /// 显示关于窗口
    pub fn show_about_window(&mut self, ctx: &Context) {
        if self.window_message.show_about_window {
            eframe::egui::Window::new("关于 Penguin")
                .collapsible(false)
                .resizable(false)
                .default_size([TOAST_WINDOW_WIDTH, TOAST_WINDOW_HEIGHT])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("Penguin PE 分析器");
                        ui.add_space(TOAST_WINDOW_SPACING);
                        ui.label("版本: 0.1.0");
                        ui.label("作者: dDostalker");
                        ui.label("描述: 一个强大的PE文件分析工具");
                        ui.add_space(TOAST_WINDOW_SPACING);

                        ui.horizontal(|ui| {
                            if ui.button("确定").clicked() {
                                self.window_message.show_about_window = false;
                            }
                        });
                    });
                });
        }
    }

    /// 显示设置窗口
    pub fn show_settings_window(&mut self, ctx: &Context) {
        if self.window_message.show_settings_window {
            eframe::egui::Window::new("设置")
                .collapsible(false)
                .resizable(true)
                .default_size([TOAST_WINDOW_WIDTH, TOAST_WINDOW_HEIGHT])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("应用程序设置");
                        ui.add_space(TOAST_WINDOW_SPACING);

                        // 主题设置

                        if ui.button("演示通知").clicked() {
                            self.demo_toasts();
                        }

                        if ui.button("取消").clicked() {
                            self.window_message.show_settings_window = false;
                        }
                    });
                });
        }
    }

    /// 显示虚拟地址->文件偏移窗口

    /// 解析16进制或10进制字符串为usize


    pub fn show_virtual_address_to_file_offset_window<T>(&mut self, ctx: &Context,nt_header:&T,section_headers:&ImageSectionHeaders)
    where T:NtHeaders + ?Sized {
        if self.window_message.show_virtual_address_to_file_offset_window {
            eframe::egui::Window::new("虚拟地址->文件偏移")
                .collapsible(false)
                .resizable(true)
                .default_size([TOAST_WINDOW_WIDTH, TOAST_WINDOW_HEIGHT])
                .show(ctx, |ui| {
                    ui.label("虚拟地址->文件偏移");
                    ui.add_space(TOAST_WINDOW_SPACING);
                    ui.label("虚拟地址 (支持10进制和16进制，如: 1234 或 0x4D2):");
                    if ui.text_edit_singleline(&mut self.window_message.virtual_address_string).changed()
                    {
                        match parse_address_string(&self.window_message.virtual_address_string) {
                            Ok(addr) => {
                                self.window_message.virtual_address = addr;
                            }
                            Err(e) => {

                            }
                        }
                    }
                    ui.label("文件偏移:");
                    let fo = rva_2_fo(nt_header,&section_headers,self.window_message.virtual_address as u32);
                    if let Some(fo) = fo {
                        ui.label(format!("{} (0x{:X})", fo, fo));
                    }
                    else {
                        ui.label("未找到");
                    }
                    if ui.button("关闭").clicked() {
                        self.window_message.show_virtual_address_to_file_offset_window = false;
                    }
                    ui.add_space(TOAST_WINDOW_SPACING);
                });
        }
    }

    /// 显示帮助窗口
    pub fn show_help_window(&mut self, ctx: &Context) {
        if self.window_message.show_help_window {
            eframe::egui::Window::new("帮助")
                .collapsible(false)
                .resizable(true)
                .default_size([TOAST_WINDOW_WIDTH, TOAST_WINDOW_HEIGHT])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading("使用帮助");
                        ui.add_space(TOAST_WINDOW_SPACING);

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

                        ui.add_space(TOAST_WINDOW_BUTTON_SPACING);

                        ui.horizontal(|ui| {
                            if ui.button("关闭").clicked() {
                                self.window_message.show_help_window = false;
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
                now.duration_since(created_at) < toast.duration
            } else {
                true
            }
        });

        // 渲染 toast
        let mut y_offset = Y_OFFSET;
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
            let color = RGB_COLOR;

            // 设置图标
            let icon = match toast.toast_type {
                ToastType::Success => "✅",
                ToastType::Error => "❌",
                ToastType::Warning => "⚠️",
                ToastType::Info => "ℹ️",
            };

            eframe::egui::Area::new(eframe::egui::Id::new(format!("toast_{}", index)))
                .fixed_pos(eframe::egui::pos2(
                    ctx.available_rect().right() - TOAST_WINDOW_SIDE_OFFSET,
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
                                        .size(TOAST_WINDOW_ICON_SIZE),
                                );
                                ui.label(
                                    eframe::egui::RichText::new(&toast.message)
                                        .color(eframe::egui::Color32::WHITE)
                                        .size(TOAST_WINDOW_TEXT_SIZE2),
                                );
                            });
                        });
                });

            y_offset += Y_OFFSET_STEP;
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
