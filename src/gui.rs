use crate::tools_api::FileManager;
use crate::tools_api::read_file::section_headers::SectionCharacteristics;
use eframe::egui::{Context, Vec2, vec2};
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

/// 消息管理器
#[derive(Default)]
pub struct SubWindowManager {
    pub export_message: ExportMessage,
    pub import_message: ImportMessage,
    pub section_message: SectionMessage,
    pub window_message: WindowMessage,
    pub toasts: Vec<Toast>,
}
/// 窗口信息
#[derive(Default)]
pub struct WindowMessage {
    pub show_about_window: bool,
    pub show_settings_window: bool,
    pub show_help_window: bool,
    pub show_virtual_address_to_file_offset_window: bool,
    pub virtual_address_string: String,
    pub virtual_address: usize,
}

/// 导出消息管理器
#[derive(Default)]
pub struct ExportMessage {
    pub selected_export_index: Option<usize>,
    pub search_string: String,
}

/// 导入信息管理器
#[derive(Default)]
pub struct ImportMessage {
    selected_function_index: Option<usize>,
    selected_dll_index: Option<usize>,
    pub search_string: String,
}

#[derive(Default)]
pub struct SectionMessage {
    pub selected_section_index: Option<usize>,
    section_flag: Option<SectionFlag>,
}

#[derive(Default)]
struct SectionFlag {
    // 节类型标志
    image_scn_cnt_code: bool,
    image_scn_cnt_initialized_data: bool,
    image_scn_cnt_uninitialized_data: bool,
    image_scn_lnk_other: bool,
    image_scn_lnk_info: bool,
    image_scn_lnk_remove: bool,
    image_scn_lnk_comdat: bool,

    // 特殊标志
    image_scn_no_defer_spec_exc: bool,
    image_scn_gprel: bool,

    // 对齐标志
    image_scn_align1_bytes: bool,
    image_scn_align2_bytes: bool,
    image_scn_align4_bytes: bool,
    image_scn_align8_bytes: bool,
    image_scn_align16_bytes: bool,
    image_scn_align32_bytes: bool,
    image_scn_align64_bytes: bool,
    image_scn_align128_bytes: bool,
    image_scn_align256_bytes: bool,
    image_scn_align512_bytes: bool,
    image_scn_align1024_bytes: bool,
    image_scn_align2048_bytes: bool,
    image_scn_align4096_bytes: bool,
    image_scn_align8192_bytes: bool,

    // 其他标志
    image_scn_lnk_nreloc_ovfl: bool,
    image_scn_mem_discardable: bool,
    image_scn_mem_not_paged: bool,
    image_scn_mem_shared: bool,
    image_scn_mem_execute: bool,
    image_scn_mem_read: bool,
    image_scn_mem_write: bool,
}

impl SectionFlag {
    pub fn match_flag(flag: u32) -> Self {
        Self {
            image_scn_cnt_code: flag & SectionCharacteristics::ImageScnCntCode as u32 != 0,
            image_scn_cnt_initialized_data: flag
                & SectionCharacteristics::ImageScnCntInitializedData as u32
                != 0,
            image_scn_cnt_uninitialized_data: flag
                & SectionCharacteristics::ImageScnCntUninitializedData as u32
                != 0,
            image_scn_lnk_other: flag & SectionCharacteristics::ImageScnLnkOther as u32 != 0,
            image_scn_lnk_info: flag & SectionCharacteristics::ImageScnLnkInfo as u32 != 0,
            image_scn_lnk_remove: flag & SectionCharacteristics::ImageScnLnkRemove as u32 != 0,
            image_scn_lnk_comdat: flag & SectionCharacteristics::ImageScnLnkComdat as u32 != 0,
            image_scn_no_defer_spec_exc: flag
                & SectionCharacteristics::ImageScnNoDeferSpecExc as u32
                != 0,
            image_scn_gprel: flag & SectionCharacteristics::ImageScnGprel as u32 != 0,
            image_scn_align1_bytes: flag & SectionCharacteristics::ImageScnAlign1Bytes as u32 != 0,
            image_scn_align2_bytes: flag & SectionCharacteristics::ImageScnAlign2Bytes as u32 != 0,
            image_scn_align4_bytes: flag & SectionCharacteristics::ImageScnAlign4Bytes as u32 != 0,
            image_scn_align8_bytes: flag & SectionCharacteristics::ImageScnAlign8Bytes as u32 != 0,
            image_scn_align16_bytes: flag & SectionCharacteristics::ImageScnAlign16Bytes as u32
                != 0,
            image_scn_align32_bytes: flag & SectionCharacteristics::ImageScnAlign32Bytes as u32
                != 0,
            image_scn_align64_bytes: flag & SectionCharacteristics::ImageScnAlign64Bytes as u32
                != 0,
            image_scn_align128_bytes: flag & SectionCharacteristics::ImageScnAlign128Bytes as u32
                != 0,
            image_scn_align256_bytes: flag & SectionCharacteristics::ImageScnAlign256Bytes as u32
                != 0,
            image_scn_align512_bytes: flag & SectionCharacteristics::ImageScnAlign512Bytes as u32
                != 0,
            image_scn_align1024_bytes: flag & SectionCharacteristics::ImageScnAlign1024Bytes as u32
                != 0,
            image_scn_align2048_bytes: flag & SectionCharacteristics::ImageScnAlign2048Bytes as u32
                != 0,
            image_scn_align4096_bytes: flag & SectionCharacteristics::ImageScnAlign4096Bytes as u32
                != 0,
            image_scn_align8192_bytes: flag & SectionCharacteristics::ImageScnAlign8192Bytes as u32
                != 0,
            image_scn_lnk_nreloc_ovfl: flag & SectionCharacteristics::ImageScnLnkNrelocOvfl as u32
                != 0,
            image_scn_mem_discardable: flag & SectionCharacteristics::ImageScnMemDiscardable as u32
                != 0,
            image_scn_mem_not_paged: flag & SectionCharacteristics::ImageScnMemNotPaged as u32 != 0,
            image_scn_mem_shared: flag & SectionCharacteristics::ImageScnMemShared as u32 != 0,
            image_scn_mem_execute: flag & SectionCharacteristics::ImageScnMemExecute as u32 != 0,
            image_scn_mem_read: flag & SectionCharacteristics::ImageScnMemRead as u32 != 0,
            image_scn_mem_write: flag & SectionCharacteristics::ImageScnMemWrite as u32 != 0,
        }
    }
}
/// Windows default settings
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

impl ExportMessage {
    pub fn clear(&mut self) {
        self.selected_export_index = None;
        self.search_string = String::new();
    }
}
impl ImportMessage {
    pub fn clear(&mut self) {
        self.selected_function_index = None;
        self.selected_dll_index = None;
        self.search_string = String::new();
    }
}
impl SectionMessage {
    pub fn clear(&mut self) {
        self.selected_section_index = None;
        self.section_flag = None;
    }
    pub fn get_image_scn_cnt_code(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_cnt_code
    }
    pub fn get_image_scn_cnt_initialized_data(&mut self) -> &mut bool {
        &mut self
            .section_flag
            .as_mut()
            .unwrap()
            .image_scn_cnt_initialized_data
    }
    pub fn get_image_scn_cnt_uninitialized_data(&mut self) -> &mut bool {
        &mut self
            .section_flag
            .as_mut()
            .unwrap()
            .image_scn_cnt_uninitialized_data
    }
    pub fn get_image_scn_lnk_other(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_lnk_other
    }
    pub fn get_image_scn_lnk_info(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_lnk_info
    }
    pub fn get_image_scn_lnk_remove(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_lnk_remove
    }
    pub fn get_image_scn_lnk_comdat(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_lnk_comdat
    }
    pub fn get_image_scn_no_defer_spec_exc(&mut self) -> &mut bool {
        &mut self
            .section_flag
            .as_mut()
            .unwrap()
            .image_scn_no_defer_spec_exc
    }
    pub fn get_image_scn_gprel(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_gprel
    }
    pub fn get_image_scn_align1_bytes(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_align1_bytes
    }
    pub fn get_image_scn_align2_bytes(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_align2_bytes
    }
    pub fn get_image_scn_align4_bytes(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_align4_bytes
    }
    pub fn get_image_scn_align8_bytes(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_align8_bytes
    }
    pub fn get_image_scn_align16_bytes(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_align16_bytes
    }
    pub fn get_image_scn_align32_bytes(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_align32_bytes
    }
    pub fn get_image_scn_align64_bytes(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_align64_bytes
    }
    pub fn get_image_scn_align128_bytes(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_align128_bytes
    }
    pub fn get_image_scn_align256_bytes(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_align256_bytes
    }
    pub fn get_image_scn_align512_bytes(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_align512_bytes
    }
    pub fn get_image_scn_align1024_bytes(&mut self) -> &mut bool {
        &mut self
            .section_flag
            .as_mut()
            .unwrap()
            .image_scn_align1024_bytes
    }
    pub fn get_image_scn_align2048_bytes(&mut self) -> &mut bool {
        &mut self
            .section_flag
            .as_mut()
            .unwrap()
            .image_scn_align2048_bytes
    }
    pub fn get_image_scn_align4096_bytes(&mut self) -> &mut bool {
        &mut self
            .section_flag
            .as_mut()
            .unwrap()
            .image_scn_align4096_bytes
    }
    pub fn get_image_scn_align8192_bytes(&mut self) -> &mut bool {
        &mut self
            .section_flag
            .as_mut()
            .unwrap()
            .image_scn_align8192_bytes
    }
    pub fn get_image_scn_lnk_nreloc_ovfl(&mut self) -> &mut bool {
        &mut self
            .section_flag
            .as_mut()
            .unwrap()
            .image_scn_lnk_nreloc_ovfl
    }
    pub fn get_image_scn_mem_discardable(&mut self) -> &mut bool {
        &mut self
            .section_flag
            .as_mut()
            .unwrap()
            .image_scn_mem_discardable
    }
    pub fn get_image_scn_mem_not_paged(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_mem_not_paged
    }
    pub fn get_image_scn_mem_shared(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_mem_shared
    }
    pub fn get_image_scn_mem_execute(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_mem_execute
    }
    pub fn get_image_scn_mem_read(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_mem_read
    }
    pub fn get_image_scn_mem_write(&mut self) -> &mut bool {
        &mut self.section_flag.as_mut().unwrap().image_scn_mem_write
    }
}
impl SubWindowManager {
    pub fn new() -> Self {
        Self {
            toasts: Vec::new(),
            ..Default::default()
        }
    }
    /// 清空所有数据
    pub fn clear_data(&mut self) {
        self.export_message.clear();
        self.import_message.clear();
        self.section_message.clear();
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
        self.sub_window_manager.render_toasts(ctx);

        if !self.files.is_empty() {
            let file = &self.files[self.current_index];
            self.sub_window_manager
                .show_virtual_address_to_file_offset_window(
                    ctx,
                    &*file.nt_head,
                    &file.section_headers,
                );
        }
    }
}

// /// 表格结构，所有的信息显示尽可能的通过这个表格结构进行创建
// fn graph_pannel(ui: &mut Ui,size:usize) {}

// Fonts

/// Windows: Set Font to support Chinese
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
pub fn load_global_font(ctx: &Context) {
    let mut fonts = eframe::egui::FontDefinitions::default();
    fonts.font_data.insert(
        "msyh".to_owned(),
        Arc::from(eframe::egui::FontData::from_static(include_bytes!(
            "/usr/share/fonts/opentype/noto/NotoSansCJK-Black.ttc"
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
