// NeuroGraph Desktop UI
// Direct integration with core_rust via FFI

use iced::{Application, Settings};

mod app;
mod auth;
mod core;   // Прямой доступ к neurograph-core
mod theme;
mod workspaces;

fn main() -> iced::Result {
    println!("🚀 Starting NeuroGraph Desktop (Direct Rust Core)...");

    app::NeuroGraphApp::run(Settings {
        window: iced::window::Settings {
            size: iced::Size::new(900.0, 600.0),
            resizable: true,
            ..Default::default()
        },
        ..Default::default()
    })
}
