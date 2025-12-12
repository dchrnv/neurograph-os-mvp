// NeuroGraph - Высокопроизводительная система пространственных вычислений на основе токенов.
// Copyright (C) 2024-2025 Chernov Denys

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.

// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

// NeuroGraph Desktop UI
// Direct integration with core_rust via FFI

use iced::{Application, Settings};

mod app;
mod auth;
mod core;   // Прямой доступ к neurograph-core
mod theme;
mod workspaces;
mod metrics;
mod layout;  // Header & Status Bar components (V3)

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
