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
        ui.label("Name:");
        ui.text_edit_singleline(&mut self.event.name);

        // date picker
        ui.label("Date:");
        let mut date = chrono::Date::from_utc(self.event.datetime.date(), chrono::Utc);
        ui.add(egui_extras::DatePickerButton::new(&mut date));

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

        self.event.datetime =
            chrono::NaiveDate::from_ymd_opt(date.year(), date.month(), date.day())
                .unwrap()
                .and_hms_opt(hour, min, sec)
                .unwrap();

        ui.label("Description:");
        ui.text_edit_multiline(&mut self.event.desc);

        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
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
