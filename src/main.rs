#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        default_theme: eframe::Theme::Dark,
        fullscreen: false,
        follow_system_theme: true,
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        icon_data: None,
        initial_window_pos: None,
        initial_window_size: Option::from(egui::Vec2::new(800_f32, 600_f32)),
        min_window_size: None,
        max_window_size: None,
        renderer: eframe::Renderer::Glow,
        resizable: true,
        run_and_return: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
    };

    eframe::run_native(
        "TimelineRS",
        native_options,
        Box::new(|cc| Box::new(timeline_rs::TimelineApp::new(cc))),
    );
}

// when compiling to web using trunk.
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    eframe::start_web(
        "timeline_rs", // hardcode it
        web_options,
        Box::new(|cc| Box::new(timeline_rs::TimelineApp::new(cc))),
    )
    .expect("failed to start eframe");
}
