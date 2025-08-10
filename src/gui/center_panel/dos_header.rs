use crate::gui::FileManager;
use crate::i18n;
use eframe::egui::{Ui, Vec2};

const MIN_SCROLLED_HEIGHT: f32 = 400.0;
const SPACING: Vec2 = Vec2::new(20.0, 8.0);
const COLUMNS: usize = 3;
impl FileManager {
    /// dos_header 窗口
    pub(crate) fn dos_header_panel(&self, ui: &mut Ui)->anyhow::Result<()> {
        eframe::egui::CentralPanel::default().show(ui.ctx(), |ui| {
            Self::show_main_title(ui, i18n::DOS_HEADER_TITLE);

            eframe::egui::ScrollArea::vertical()
                .min_scrolled_height(MIN_SCROLLED_HEIGHT)
                .show(ui, |ui| {
                    // 使用表格样式
                    eframe::egui::Grid::new("dos_header_grid")
                        .striped(true)
                        .spacing(SPACING)
                        .num_columns(COLUMNS)
                        .show(ui, |ui| {
                            // 表头
                            ui.strong(i18n::DOS_HEADER_FIELD_NAME);
                            ui.strong(i18n::DOS_HEADER_VALUE);
                            ui.strong(i18n::DOS_HEADER_DESCRIPTION);
                            ui.end_row();

                            // DOS Header 字段
                            ui.label("e_cblp");
                            ui.label(&self.get_cblp());
                            ui.label(i18n::DOS_HEADER_E_CBLP);
                            ui.end_row();

                            ui.label("e_cp");
                            ui.label(&self.get_cp());
                            ui.label(i18n::DOS_HEADER_E_CP);
                            ui.end_row();

                            ui.label("e_crlc");
                            ui.label(&self.get_crlc());
                            ui.label(i18n::DOS_HEADER_E_CRLC);
                            ui.end_row();

                            ui.label("e_cparhdr");
                            ui.label(&self.get_cparhdr());
                            ui.label(i18n::DOS_HEADER_E_CPARHDR);
                            ui.end_row();

                            ui.label("e_minalloc");
                            ui.label(&self.get_minalloc());
                            ui.label(i18n::DOS_HEADER_E_MINALLOC);
                            ui.end_row();

                            ui.label("e_maxalloc");
                            ui.label(&self.get_maxalloc());
                            ui.label(i18n::DOS_HEADER_E_MAXALLOC);
                            ui.end_row();

                            ui.label("e_ss");
                            ui.label(&self.get_ss());
                            ui.label(i18n::DOS_HEADER_E_SS);
                            ui.end_row();

                            ui.label("e_sp");
                            ui.label(&self.get_sp());
                            ui.label(i18n::DOS_HEADER_E_SP);
                            ui.end_row();

                            ui.label("e_csum");
                            ui.label(&self.get_csum());
                            ui.label(i18n::DOS_HEADER_E_CSUM);
                            ui.end_row();

                            ui.label("e_ip");
                            ui.label(&self.get_ip());
                            ui.label(i18n::DOS_HEADER_E_IP);
                            ui.end_row();

                            ui.label("e_cs");
                            ui.label(&self.get_cs());
                            ui.label(i18n::DOS_HEADER_E_CS);
                            ui.end_row();

                            ui.label("e_lfarlc");
                            ui.label(&self.get_lfarlc());
                            ui.label(i18n::DOS_HEADER_E_LFARLC);
                            ui.end_row();

                            ui.label("e_ovno");
                            ui.label(&self.get_ovno());
                            ui.label(i18n::DOS_HEADER_E_OVNO);
                            ui.end_row();

                            ui.label("e_res");
                            ui.label(&self.get_res());
                            ui.label(i18n::DOS_HEADER_E_RES);
                            ui.end_row();

                            ui.label("e_oemid");
                            ui.label(&self.get_oemid());
                            ui.label(i18n::DOS_HEADER_E_OEMID);
                            ui.end_row();

                            ui.label("e_oeminfo");
                            ui.label(&self.get_oeminfo());
                            ui.label(i18n::DOS_HEADER_E_OEMINFO);
                            ui.end_row();

                            ui.label("e_res2");
                            ui.label(&self.get_res2());
                            ui.label(i18n::DOS_HEADER_E_RES2);
                            ui.end_row();

                            ui.label("e_lfanew");
                            ui.label(&self.get_lfanew());
                            ui.label(i18n::DOS_HEADER_E_LFANEW);
                            ui.end_row();
                        });
                });
        });
        Ok(())
    }
    pub(crate) fn get_cblp(&self) -> String {
        format!(
            "{}",
            self.files.get(self.current_index).unwrap().dos_head.e_cblp
        )
    }
    pub(crate) fn get_cp(&self) -> String {
        format!(
            "{}",
            self.files.get(self.current_index).unwrap().dos_head.e_cp
        )
    }
    pub(crate) fn get_crlc(&self) -> String {
        format!(
            "{}",
            self.files.get(self.current_index).unwrap().dos_head.e_crlc
        )
    }
    pub(crate) fn get_cparhdr(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .dos_head
                .e_cparhdr
        )
    }
    pub(crate) fn get_minalloc(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .dos_head
                .e_minalloc
        )
    }
    pub(crate) fn get_maxalloc(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .dos_head
                .e_maxalloc
        )
    }
    pub(crate) fn get_ss(&self) -> String {
        format!(
            "{}",
            self.files.get(self.current_index).unwrap().dos_head.e_ss
        )
    }
    pub(crate) fn get_sp(&self) -> String {
        format!(
            "{}",
            self.files.get(self.current_index).unwrap().dos_head.e_sp
        )
    }
    pub(crate) fn get_csum(&self) -> String {
        format!(
            "{}",
            self.files.get(self.current_index).unwrap().dos_head.e_csum
        )
    }
    pub(crate) fn get_ip(&self) -> String {
        format!(
            "{}",
            self.files.get(self.current_index).unwrap().dos_head.e_ip
        )
    }
    pub(crate) fn get_cs(&self) -> String {
        format!(
            "{}",
            self.files.get(self.current_index).unwrap().dos_head.e_cs
        )
    }
    pub(crate) fn get_lfarlc(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .dos_head
                .e_lfarlc
        )
    }
    pub(crate) fn get_ovno(&self) -> String {
        format!(
            "{}",
            self.files.get(self.current_index).unwrap().dos_head.e_ovno
        )
    }
    pub(crate) fn get_res(&self) -> String {
        format!(
            "{:?}",
            self.files.get(self.current_index).unwrap().dos_head.e_res
        )
    }
    pub(crate) fn get_oemid(&self) -> String {
        format!(
            "{}",
            self.files.get(self.current_index).unwrap().dos_head.e_oemid
        )
    }
    pub(crate) fn get_oeminfo(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .dos_head
                .e_oeminfo
        )
    }
    pub(crate) fn get_res2(&self) -> String {
        format!(
            "{:?}",
            self.files.get(self.current_index).unwrap().dos_head.e_res2
        )
    }
    pub(crate) fn get_lfanew(&self) -> String {
        format!(
            "{}",
            self.files
                .get(self.current_index)
                .unwrap()
                .dos_head
                .e_lfanew
        )
    }
}
