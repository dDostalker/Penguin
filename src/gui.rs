use crate::tools_api::structure::FileInfo;
use eframe::egui::{Context, vec2};
use eframe::{HardwareAcceleration, Renderer};
use std::sync::Arc;
use std::time::{Duration, Instant};

mod center_panel;
mod left_panel;
mod top_header_panel;

/// Toast 通知类型
#[derive(Debug, Clone)]
pub enum ToastType {
    Success,
    Error,
    Warning,
    Info,
}

/// Toast 通知结构
#[derive(Debug, Clone)]
pub struct Toast {
    pub message: String,
    pub toast_type: ToastType,
    pub created_at: Instant,
    pub duration: Duration,
}

/// 子窗口管理器
#[derive(Default)]
pub struct SubWindowManager {
    pub selected_export_index: Option<usize>,
    pub select_dll_index: Option<usize>,
    pub select_function_index: Option<usize>,
    pub selected_section_index: Option<usize>,
    pub show_about_window: bool,
    pub show_settings_window: bool,
    pub show_help_window: bool,
    pub toasts: Vec<Toast>, // 新增：toast 通知列表
}

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
            created_at: Instant::now(),
            duration: Duration::from_secs(3), // 默认显示3秒
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
        self.toasts
            .retain(|toast| now.duration_since(toast.created_at) < toast.duration);

        // 渲染 toast
        let mut y_offset = 50.0;
        for (index, toast) in self.toasts.iter().enumerate() {
            let elapsed = now.duration_since(toast.created_at);
            let progress = elapsed.as_secs_f32() / toast.duration.as_secs_f32();

            // 计算透明度（淡出效果）
            let alpha = if progress > 0.8 {
                1.0 - (progress - 0.8) * 5.0
            } else {
                1.0
            };

            // 根据类型设置颜色
            let color = match toast.toast_type {
                ToastType::Success => eframe::egui::Color32::from_rgb(76, 175, 80),
                ToastType::Error => eframe::egui::Color32::from_rgb(244, 67, 54),
                ToastType::Warning => eframe::egui::Color32::from_rgb(255, 152, 0),
                ToastType::Info => eframe::egui::Color32::from_rgb(33, 150, 243),
            };

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
                        .corner_radius(8.0)
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
                            ui.add_space(8.0);
                        });
                });

            y_offset += 60.0;
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

/// 窗口默认设置
pub fn create_native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder {
            title: None,
            app_id: None,
            position: None,
            inner_size: None,
            min_inner_size: Some(vec2(1000.0, 600.0)),
            max_inner_size: None,
            clamp_size_to_monitor_size: None,
            fullscreen: None,
            maximized: None,
            resizable: None,
            transparent: None,
            decorations: None,
            icon: None,
            active: None,
            visible: None,
            fullsize_content_view: None,
            movable_by_window_background: None,
            title_shown: None,

            titlebar_buttons_shown: None,
            titlebar_shown: None,
            has_shadow: None,
            drag_and_drop: None,
            taskbar: None,
            close_button: None,
            minimize_button: Some(true),
            maximize_button: Some(true),
            window_level: None,
            mouse_passthrough: None,
            window_type: None,
        },

        vsync: false,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        hardware_acceleration: HardwareAcceleration::Required,
        renderer: Renderer::default(),
        run_and_return: false,
        event_loop_builder: None,
        window_builder: None,
        shader_version: None,
        centered: false,
        persist_window: false,
        persistence_path: None,
        dithering: false,
    }
}

/// 窗口数组及其信息
#[derive(Default)]
pub enum Page {
    #[default]
    DosHead,
    DosStub,
    NtHead,
    SectionHead,
    Import,
    Export,
}

#[derive(Default)]

pub struct FileManager {
    pub files: Vec<FileInfo>,                        // 文件列表
    pub(crate) current_index: usize,                 // 当前文件索引
    pub(crate) page: Page,                           // 目标页面
    pub(crate) hover_index: usize,                   // 左边栏悬停
    pub(crate) sub_window_manager: SubWindowManager, // 子窗口管理器
}
impl FileManager {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            sub_window_manager: SubWindowManager::new(),
            ..Default::default()
        }
    }
    pub fn get_file(&self) -> &FileInfo {
        self.files.get(self.current_index).unwrap()
    }
}

/// 主程序主题布局
impl eframe::App for FileManager {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        self.top_label(ctx);
        self.left_label(ctx);
        self.center(ctx);
        load_global_font(ctx);

        // 显示子窗口
        self.sub_window_manager.show_about_window(ctx);
        self.sub_window_manager.show_settings_window(ctx);
        self.sub_window_manager.show_help_window(ctx);
        self.sub_window_manager.render_toasts(ctx); // 渲染 toast
    }
}



/// 中文设置
///全局加载支持中文的字体
#[cfg(target_os = "windows")]
pub fn load_global_font(ctx: &Context) {
    let mut fonts = eframe::egui::FontDefinitions::default();
    fonts.font_data.insert(
        "msyh".to_owned(),
        Arc::from(eframe::egui::FontData::from_static(include_bytes!(
            "C:\\Windows\\Fonts\\msyh.ttc"
        ))),
    );

    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "msyh".to_owned());
    fonts
        .families
        .get_mut(&eframe::egui::FontFamily::Monospace)
        .unwrap()
        .push("msyh".to_owned());
    ctx.set_fonts(fonts);
}
#[cfg(target_os = "linux")]
pub fn load_global_font(ctx: &Context) {}
