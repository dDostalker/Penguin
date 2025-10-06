use crate::{
    gui::{SubWindowManager, Toast, ToastType},
    i18n,
    tools_api::{
        parse_address_string,
        read_file::{ImageSectionHeaders, nt_header::traits::NtHeaders, rva_2_fo},
        serde_pe::DangerousFunction,
    },
};
use eframe::egui::Context;
use std::{
    fs,
    time::{Duration, Instant},
};
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
            eframe::egui::Window::new(i18n::ABOUT_TITLE)
                .collapsible(false)
                .resizable(false)
                .default_size([TOAST_WINDOW_WIDTH, TOAST_WINDOW_HEIGHT])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading(i18n::APP_TITLE);
                        ui.add_space(TOAST_WINDOW_SPACING);
                        ui.label(i18n::VERSION);
                        ui.label(i18n::AUTHOR);
                        ui.label(i18n::PEDESCRIPTION);
                        ui.label("github: https://github.com/dDostalker/Penguin");
                        ui.add_space(TOAST_WINDOW_SPACING);

                        ui.horizontal(|ui| {
                            if ui.button(i18n::OK_BUTTON).clicked() {
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
            eframe::egui::Window::new(i18n::SETTINGS_TITLE)
                .collapsible(false)
                .resizable(true)
                .default_size([TOAST_WINDOW_WIDTH, TOAST_WINDOW_HEIGHT])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading(i18n::APP_SETTINGS);
                        ui.add_space(TOAST_WINDOW_SPACING);
                        if ui.button(i18n::DEMO_NOTIFICATIONS).clicked() {
                            self.demo_toasts();
                        }
                        ui.horizontal(|ui| {
                            ui.label(i18n::CREATE_DANGEROUS_FUNCTION);
                            if ui.button(i18n::CREATE).clicked() {
                                if !fs::exists("DangerFunc.toml").unwrap() {
                                    let content =
                                        toml::to_string(&DangerousFunction::default()).unwrap();
                                    fs::write("DangerFunc.toml", content).unwrap();
                                }
                                self.show_success(i18n::CREATE_DANGEROUS_FUNCTION_SUCCESS);
                            }
                        });
                        if ui.button(i18n::CANCEL_BUTTON).clicked() {
                            self.window_message.show_settings_window = false;
                        }
                    });
                });
        }
    }

    /// 显示虚拟地址->文件偏移窗口

    /// 解析16进制或10进制字符串为usize

    pub fn show_virtual_address_to_file_offset_window<T>(
        &mut self,
        ctx: &Context,
        nt_header: &T,
        section_headers: &ImageSectionHeaders,
    ) where
        T: NtHeaders + ?Sized,
    {
        if self
            .window_message
            .show_virtual_address_to_file_offset_window
        {
            eframe::egui::Window::new(i18n::VIRTUAL_ADDRESS_TO_FILE_OFFSET)
                .collapsible(false)
                .resizable(true)
                .default_size([TOAST_WINDOW_WIDTH, TOAST_WINDOW_HEIGHT])
                .show(ctx, |ui| {
                    ui.label(i18n::VIRTUAL_ADDRESS_TO_FILE_OFFSET);
                    ui.add_space(TOAST_WINDOW_SPACING);
                    ui.label(i18n::VIRTUAL_ADDRESS_LABEL);
                    if ui
                        .text_edit_singleline(&mut self.window_message.virtual_address_string)
                        .changed()
                    {
                        match parse_address_string(&self.window_message.virtual_address_string) {
                            Ok(addr) => {
                                self.window_message.virtual_address = addr;
                            }
                            Err(_e) => {}
                        }
                    }
                    ui.label(i18n::FILE_OFFSET_LABEL);
                    let fo = rva_2_fo(
                        nt_header,
                        section_headers,
                        self.window_message.virtual_address as u32,
                    );
                    if let Some(fo) = fo {
                        ui.label(format!("{} (0x{:X})", fo, fo));
                    } else {
                        ui.label(i18n::NOT_FOUND);
                    }
                    if ui.button(i18n::CLOSE_BUTTON).clicked() {
                        self.window_message
                            .show_virtual_address_to_file_offset_window = false;
                    }
                    ui.add_space(TOAST_WINDOW_SPACING);
                });
        }
    }

    pub fn show_help_window(&mut self, ctx: &Context) {
        if self.window_message.show_help_window {
            eframe::egui::Window::new(i18n::HELP_TITLE)
                .collapsible(false)
                .resizable(true)
                .default_size([TOAST_WINDOW_WIDTH, TOAST_WINDOW_HEIGHT])
                .show(ctx, |ui| {
                    ui.vertical(|ui| {
                        ui.heading(i18n::USAGE_HELP);
                        ui.add_space(TOAST_WINDOW_SPACING);
                        ui.label("https://github.com/dDostalker/Penguin.wiki.git");
                        ui.add_space(TOAST_WINDOW_SPACING);
                        ui.add_space(TOAST_WINDOW_BUTTON_SPACING);
                        ui.horizontal(|ui| {
                            if ui.button(i18n::CLOSE_BUTTON).clicked() {
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
        self.show_success(i18n::DEMO_OPERATION_SUCCESS);
        self.show_error(i18n::DEMO_ERROR_OCCURRED);
        self.show_warning(i18n::DEMO_WARNING_NOTICE);
        self.show_info(i18n::DEMO_INFO_MESSAGE);
    }
}
