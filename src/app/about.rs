use egui::Ui;

#[derive(Default, serde::Deserialize, serde::Serialize)]
pub struct About {
    pub open: bool,
}

impl About {
    pub fn title(&self) -> &'static str {
        "About TimelineRS"
    }

    pub fn show(&mut self, ui: &mut Ui) {
        let mut open = self.open;
        egui::Window::new(self.title())
            .collapsible(false)
            .resizable(true)
            .open(&mut open)
            .show(ui.ctx(), |ui| self.ui(ui));
        self.open = open;
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        ui.label("TimelineRS is a simple timeline app written in Rust!");
        ui.hyperlink("https://github.com/mikeder/timeline_rs");
        egui::warn_if_debug_build(ui);
    }
}
