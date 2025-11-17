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

// Theme configuration - Cyberpunk aesthetic for NeuroGraph OS
use iced::Color;

pub use iced::Theme;

// Color palette from UI_Control_Panel_V2.md
pub struct CyberColors;

impl CyberColors {
    // Фоны
    pub const BG_PRIMARY: Color = Color::from_rgb(0.04, 0.04, 0.04);     // #0a0a0a
    pub const BG_SECONDARY: Color = Color::from_rgb(0.08, 0.08, 0.08);   // #141414
    pub const BG_TERTIARY: Color = Color::from_rgb(0.10, 0.10, 0.10);    // #1a1a1a
    pub const BG_HOVER: Color = Color::from_rgb(0.15, 0.15, 0.15);       // #252525

    // Текст
    pub const TEXT_PRIMARY: Color = Color::from_rgb(0.88, 0.88, 0.88);   // #e0e0e0
    pub const TEXT_SECONDARY: Color = Color::from_rgb(0.63, 0.63, 0.63); // #a0a0a0
    pub const TEXT_MUTED: Color = Color::from_rgb(0.38, 0.38, 0.38);     // #606060

    // Акценты
    pub const ACCENT_PRIMARY: Color = Color::from_rgb(0.0, 1.0, 0.8);    // #00ffcc (неоново-бирюзовый)
    pub const ACCENT_BLUE: Color = Color::from_rgb(0.2, 0.6, 1.0);       // #3399ff
    pub const ACCENT_PURPLE: Color = Color::from_rgb(0.6, 0.4, 1.0);     // #9966ff

    // Статусы
    pub const STATUS_OK: Color = Color::from_rgb(0.2, 1.0, 0.4);         // #33ff66
    pub const STATUS_WARNING: Color = Color::from_rgb(1.0, 0.67, 0.2);   // #ffaa33
    pub const STATUS_CRITICAL: Color = Color::from_rgb(1.0, 0.2, 0.4);   // #ff3366

    // Режимы
    pub const MODE_USER: Color = Color::from_rgba(0.0, 1.0, 0.8, 0.2);   // #00ffcc33 (прозрачный)
    pub const MODE_ROOT: Color = Color::from_rgb(1.0, 0.4, 0.0);         // #ff6600 (оранжевый)
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
