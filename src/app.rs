use core::str;

use egui::FontFamily::Proportional;
use egui::FontId;
use egui::TextStyle::*;

mod about;
mod event;
mod toggle;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TimelineApp {
    // About window
    about: about::About,

    // Events
    add_event: event::AddEvent,
    events: Vec<event::Event>,

    light: bool,
}

impl TimelineApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        let mut s = (*cc.egui_ctx.style()).clone();

        // Redefine text_styles
        s.text_styles = [
            (Heading, FontId::new(30.0, Proportional)),
            (Name("Heading2".into()), FontId::new(25.0, Proportional)),
            (Name("Context".into()), FontId::new(23.0, Proportional)),
            (Body, FontId::new(22.0, Proportional)),
            (Monospace, FontId::new(20.0, Proportional)),
            (Button, FontId::new(35.0, Proportional)),
            (Small, FontId::new(25.0, Proportional)),
        ]
        .into();

        // Mutate global style with above changes
        cc.egui_ctx.set_style(s);

        Default::default()
    }
}

impl eframe::App for TimelineApp {
    // Set auto save interval
    fn auto_save_interval(&self) -> std::time::Duration {
        std::time::Duration::from_secs(10)
    }

    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {
            about: _,
            events: _,
            add_event: _,
            light: _,
        } = self;

        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Reset").clicked() {
                        self.events = vec![];
                        self.add_event.event = event::Event::default();
                        self.add_event.open = false;
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        self.about.open = true;
                        self.about.show(ui);
                        ui.close_menu();
                    }
                    ui.menu_button("Visuals", |ui| {
                        if ui.button("Dark").clicked() {
                            ctx.set_visuals(egui::Visuals::dark())
                        }
                        if ui.button("Light").clicked() {
                            ctx.set_visuals(egui::Visuals::light())
                        }
                    })
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                ui.vertical_centered(|ui| {
                    for e in self.events.iter() {
                        {
                            let mut name = e.name.clone();
                            let mut desc = e.desc.clone();
                            ui.label(&e.datetime.to_string());
                            ui.label("Name:");
                            ui.text_edit_singleline(&mut name);
                            ui.label("Description:");
                            ui.text_edit_multiline(&mut desc);
                            ui.separator();
                        }
                    }

                    // show about window
                    if self.about.open {
                        self.about.show(ui)
                    }

                    // show add event window
                    if self.add_event.open {
                        self.add_event.show(ui);
                    }
                    // add new event if one is submitted
                    if self.add_event.submitted {
                        let e = self.add_event.event.clone();
                        self.events.push(e);
                        self.events.sort_by(|a, b| b.datetime.cmp(&a.datetime));
                        self.add_event = event::AddEvent::default();
                    }
                });
            })
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                // style this button
                ui.style_mut().spacing.button_padding = egui::vec2(10.0, 10.0);

                if ui.button("Add Event").clicked() {
                    self.add_event.open = true
                }
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                ui.add(toggle::toggle(&mut self.light));
                if self.light {
                    ctx.set_visuals(egui::Visuals::light())
                } else {
                    ctx.set_visuals(egui::Visuals::dark())
                }
            })
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
