// Theme system for NeuroGraph Desktop UI
// Based on DESKTOP_UI_SPEC_V3.md - Terminal Modern style

use iced::Color;

pub mod colors;
pub mod styles;

pub use colors::*;
pub use styles::*;

/// Theme selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

impl Theme {
    pub fn palette(&self) -> ThemePalette {
        match self {
            Theme::Dark => ThemePalette::dark(),
            Theme::Light => ThemePalette::light(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Theme::Dark
    }
}

/// Complete theme palette
#[derive(Debug, Clone)]
pub struct ThemePalette {
    pub background: BackgroundColors,
    pub text: TextColors,
    pub accent: AccentColors,
    pub status: StatusColors,
    pub border: BorderColors,
}

#[derive(Debug, Clone)]
pub struct BackgroundColors {
    pub primary: Color,      // Main app background
    pub secondary: Color,    // Cards and panels
    pub hover: Color,        // Hover state
    pub active: Color,       // Active element
}

#[derive(Debug, Clone)]
pub struct TextColors {
    pub primary: Color,      // Main text
    pub secondary: Color,    // Labels, hints
    pub muted: Color,        // Placeholder, disabled
    pub link: Color,         // Links
}

#[derive(Debug, Clone)]
pub struct AccentColors {
    pub primary: Color,      // Main accent (buttons, active)
    pub hover: Color,        // Accent hover state
}

#[derive(Debug, Clone)]
pub struct StatusColors {
    pub success: Color,      // Green
    pub warning: Color,      // Yellow-orange
    pub error: Color,        // Red
    pub info: Color,         // Blue
}

#[derive(Debug, Clone)]
pub struct BorderColors {
    pub normal: Color,       // Regular borders
    pub focus: Color,        // Focused elements
    pub subtle: Color,       // Dividers
}

impl ThemePalette {
    /// Dark theme (default) - GitHub Dark inspired
    pub fn dark() -> Self {
        Self {
            background: BackgroundColors {
                primary: hex_to_color("#0d1117"),
                secondary: hex_to_color("#161b22"),
                hover: hex_to_color("#21262d"),
                active: hex_to_color("#30363d"),
            },
            text: TextColors {
                primary: hex_to_color("#c9d1d9"),
                secondary: hex_to_color("#8b949e"),
                muted: hex_to_color("#484f58"),
                link: hex_to_color("#58a6ff"),
            },
            accent: AccentColors {
                primary: hex_to_color("#58a6ff"),
                hover: hex_to_color("#79b8ff"),
            },
            status: StatusColors {
                success: hex_to_color("#3fb950"),
                warning: hex_to_color("#d29922"),
                error: hex_to_color("#f85149"),
                info: hex_to_color("#58a6ff"),
            },
            border: BorderColors {
                normal: hex_to_color("#30363d"),
                focus: hex_to_color("#58a6ff"),
                subtle: hex_to_color("#21262d"),
            },
        }
    }

    /// Light theme
    pub fn light() -> Self {
        Self {
            background: BackgroundColors {
                primary: hex_to_color("#ffffff"),
                secondary: hex_to_color("#f6f8fa"),
                hover: hex_to_color("#eaeef2"),
                active: hex_to_color("#dde4eb"),
            },
            text: TextColors {
                primary: hex_to_color("#24292f"),
                secondary: hex_to_color("#57606a"),
                muted: hex_to_color("#8c959f"),
                link: hex_to_color("#0969da"),
            },
            accent: AccentColors {
                primary: hex_to_color("#0969da"),
                hover: hex_to_color("#0550ae"),
            },
            status: StatusColors {
                success: hex_to_color("#1a7f37"),
                warning: hex_to_color("#9a6700"),
                error: hex_to_color("#d1242f"),
                info: hex_to_color("#0969da"),
            },
            border: BorderColors {
                normal: hex_to_color("#d0d7de"),
                focus: hex_to_color("#0969da"),
                subtle: hex_to_color("#eaeef2"),
            },
        }
    }
}

/// Convert hex color to iced::Color
fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);

    Color::from_rgb(
        r as f32 / 255.0,
        g as f32 / 255.0,
        b as f32 / 255.0,
    )
}
