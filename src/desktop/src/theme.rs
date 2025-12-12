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

// Theme configuration - Terminal Modern aesthetic for NeuroGraph OS
// Inspired by: Linear, Raycast, GitHub Dark, Warp Terminal
use iced::Color;

pub use iced::Theme;

// Color palette from DESKTOP_UI_SPEC_V3.md
pub struct TerminalColors;

impl TerminalColors {
    // Фоны (GitHub Dark inspired)
    pub const BG_PRIMARY: Color = Color::from_rgb(0.051, 0.067, 0.090);     // #0d1117
    pub const BG_SECONDARY: Color = Color::from_rgb(0.086, 0.106, 0.133);   // #161b22
    pub const BG_HOVER: Color = Color::from_rgb(0.129, 0.149, 0.176);       // #21262d
    pub const BG_ACTIVE: Color = Color::from_rgb(0.188, 0.212, 0.239);      // #30363d

    // Текст
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.788, 0.820, 0.851);   // #c9d1d9
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.545, 0.580, 0.620); // #8b949e
    pub const TEXT_MUTED: Color = Color::from_rgb(0.282, 0.310, 0.345);     // #484f58

    // Акцент (мягкий голубой)
    pub const ACCENT_PRIMARY: Color = Color::from_rgb(0.345, 0.651, 1.0);   // #58a6ff
    pub const ACCENT_HOVER: Color = Color::from_rgb(0.475, 0.722, 1.0);     // #79b8ff

    // Статусы
    pub const STATUS_OK: Color = Color::from_rgb(0.247, 0.725, 0.314);      // #3fb950
    pub const STATUS_WARNING: Color = Color::from_rgb(0.824, 0.600, 0.133); // #d29922
    pub const STATUS_CRITICAL: Color = Color::from_rgb(0.973, 0.318, 0.286);// #f85149
    pub const STATUS_INFO: Color = Color::from_rgb(0.345, 0.651, 1.0);      // #58a6ff

    // Границы
    pub const BORDER_DEFAULT: Color = Color::from_rgb(0.188, 0.212, 0.239); // #30363d
    pub const BORDER_FOCUS: Color = Color::from_rgb(0.345, 0.651, 1.0);     // #58a6ff
    pub const BORDER_SUBTLE: Color = Color::from_rgb(0.129, 0.149, 0.176);  // #21262d
}

// Размеры текста
pub mod text_size {
    pub const XS: u16 = 11;
    pub const SM: u16 = 13;
    pub const BASE: u16 = 14;
    pub const LG: u16 = 16;
    pub const XL: u16 = 20;
    pub const XXL: u16 = 24;
    pub const DISPLAY: u16 = 32;
}

// Отступы
pub mod spacing {
    pub const XS: u16 = 4;
    pub const SM: u16 = 8;
    pub const BASE: u16 = 12;
    pub const LG: u16 = 16;
    pub const XL: u16 = 24;
    pub const XXL: u16 = 32;
}
