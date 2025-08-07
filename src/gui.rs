use crate::tools_api::FileManager;
use eframe::egui::{Context, vec2, Vec2};
use eframe::{HardwareAcceleration, Renderer};
use std::sync::Arc;
use std::time::{Duration, Instant};

mod center_panel;
mod left_panel;
mod toast_window;
mod top_header_panel;


const MIN_INNER_SIZE: Vec2 = vec2(1000.0, 600.0);
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
    pub created_at: Option<Instant>,
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

/// 窗口默认设置
pub fn create_native_options() -> eframe::NativeOptions {
    eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder {
            title: None,
            app_id: None,
            position: None,
            inner_size: None,
            min_inner_size: Some(MIN_INNER_SIZE),
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
