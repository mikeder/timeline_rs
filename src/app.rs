use core::str;

use egui::Ui;
// use egui_datepicker::{Date, DatePicker, Utc};

mod event;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TimelineApp {
    // this how you opt-out of serialization of a member
    // #[serde(skip)]
    // value: f32,

    // Show splash
    splash_enabled: bool,

    // Events
    add_event: event::AddEvent,
    events: Vec<event::Event>,
}

impl Default for TimelineApp {
    fn default() -> Self {
        Self {
            splash_enabled: true,
            events: vec![],

            add_event: event::AddEvent::default(),
        }
    }
}

impl TimelineApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }

    pub fn show_about(&mut self, ctx: &egui::Context) -> bool {
        egui::Window::new("TimelineRS")
            .collapsible(false)
            .open(&mut self.splash_enabled)
            .auto_sized()
            .show(ctx, |ui| {
                ui.label("TimelineRS is a simple timeline app written in Rust!");
                ui.hyperlink("https://github.com/mikeder/timeline_rs");
                egui::warn_if_debug_build(ui);
            })
            .is_none()
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
            events: _,
            add_event: _,
            splash_enabled: _,
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
                        self.splash_enabled = true;
                        _ = self.show_about(ctx);
                        ui.close_menu();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // Show splash/about screen if enabled
            if self.splash_enabled {
                let closed = self.show_about(ctx);
                if closed {
                    self.splash_enabled = false;

                    let storage = _frame.storage_mut();
                    match storage {
                        None => println!("no storage"),
                        Some(s) => {
                            println!("saving to storage");
                            self.save(s);
                        }
                    }
                };
            }

            // position buttons across the top
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("Add Event").clicked() {
                    self.add_event.open = true
                }
            });

            if self.add_event.open {
                self.add_event.show(ui);
            }
            if self.add_event.submitted {
                let e = self.add_event.event.clone();
                self.events.push(e);
                self.add_event = event::AddEvent::default();
            }

            ui.add(egui::Separator::default());

            egui::ScrollArea::vertical().show(ui, |ui: &mut Ui| {
                self.events.sort_by_key(|x| x.datetime);
                for e in self.events.iter() {
                    ui.label(
                        egui::RichText::new(&e.datetime.to_string())
                            .color(egui::Color32::LIGHT_GRAY),
                    );
                    ui.columns(2, |columns| {
                        columns[0].label(&e.name);
                        columns[1].label(&e.desc);
                    })
                }
            });
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
