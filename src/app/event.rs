// https://github.com/emilk/egui/discussions/1598
use chrono::{Datelike, Timelike, Utc};

use egui::Ui;

#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct AddEvent {
    pub open: bool,
    pub event: Event,
    pub submitted: bool,
}

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct Event {
    pub name: String,
    pub datetime: chrono::NaiveDateTime,
    pub desc: String,
}

impl Default for Event {
    fn default() -> Self {
        Self {
            name: String::new(),
            datetime: Utc::now().naive_local(),
            desc: String::new(),
        }
    }
}

impl AddEvent {
    pub fn title(&self) -> &'static str {
        "Add New Event Information"
    }

    pub fn show(&mut self, ui: &mut Ui) {
        egui::Window::new(self.title())
            .collapsible(false)
            .resizable(true)
            .show(ui.ctx(), |ui| self.ui(ui));
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        // date pickers
        ui.label("Date:");
        let date = self.event.datetime.date();
        let mut year = date.year();
        let mut month = date.month();
        let mut day = date.day();

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            // year
            ui.label("Y");
            egui::ComboBox::from_id_source("year")
                .selected_text(format!("{:?}", year))
                .width(45.0)
                .show_ui(ui, |ui| {
                    for n in date.year() - 5..date.year() {
                        ui.push_id(format!("year{:?}", n), |ui| {
                            ui.selectable_value(&mut year, n, n.to_string());
                        });
                    }
                });

            // month
            ui.label("M");
            egui::ComboBox::from_id_source("month")
                .selected_text(format!("{:?}", month))
                .width(45.0)
                .show_ui(ui, |ui| {
                    for n in 1..=12 {
                        ui.push_id(format!("month{:?}", n), |ui| {
                            ui.selectable_value(&mut month, n, n.to_string());
                        });
                    }
                });

            // day
            ui.label("D");
            egui::ComboBox::from_id_source("day")
                .selected_text(format!("{:?}", day))
                .width(45.0)
                .show_ui(ui, |ui| {
                    for n in 1..=31 {
                        ui.push_id(format!("day{:?}", n), |ui| {
                            ui.selectable_value(&mut day, n, n.to_string());
                        });
                    }
                });
        });

        let time = self.event.datetime.time();
        let mut hour = time.hour();
        let mut min = time.minute();
        let mut sec = time.second();

        // time selectors
        ui.label("Time:");
        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
            // hour
            ui.label("H");
            egui::ComboBox::from_id_source("H")
                .selected_text(format!("{:?}", hour))
                .width(45.0)
                .show_ui(ui, |ui| {
                    for n in 0..=23 {
                        ui.push_id(format!("H{:?}", n), |ui| {
                            ui.selectable_value(&mut hour, n, n.to_string());
                        });
                    }
                });
            // minute
            ui.label("M");
            egui::ComboBox::from_id_source("M")
                .selected_text(format!("{:?}", min))
                .width(45.0)
                .show_ui(ui, |ui| {
                    for n in 0..=59 {
                        ui.push_id(format!("M{:?}", n), |ui| {
                            ui.selectable_value(&mut min, n, n.to_string());
                        });
                    }
                });
            // second
            ui.label("S");
            egui::ComboBox::from_id_source("S")
                .selected_text(format!("{:?}", sec))
                .width(45.0)
                .show_ui(ui, |ui| {
                    for n in 0..=59 {
                        ui.push_id(format!("S{:?}", n), |ui| {
                            ui.selectable_value(&mut sec, n, n.to_string());
                        });
                    }
                });
        });

        self.event.datetime = chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, min, sec)
            .unwrap();

        ui.label("Name:");
        ui.text_edit_singleline(&mut self.event.name);

        ui.label("Description:");
        ui.text_edit_multiline(&mut self.event.desc);

        ui.horizontal(|ui| {
            if ui.button("Add").clicked() {
                self.submitted = true;
            }
            if ui.button("Cancel").clicked() {
                self.open = false;
                self.event = Event::default();
            }
        });
    }
}
