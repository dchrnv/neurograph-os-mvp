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

// Auth module - Phase 3: PIN-based authentication (V3 spec)

use iced::{widget::{column, text, text_input, button, container, row}, Element, Length};
use crate::app::Message;
use crate::theme::{TerminalColors, text_size, spacing};

pub struct AuthState {
    pin_input: String,
    is_authenticated: bool,
    is_admin: bool,
    error: Option<String>,
    failed_attempts: u8,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            pin_input: String::new(),
            is_authenticated: false,
            is_admin: false,
            error: None,
            failed_attempts: 0,
        }
    }

    pub fn update_password(&mut self, input: String) {
        // Только цифры, максимум 6
        if input.len() <= 6 && input.chars().all(|c| c.is_ascii_digit()) {
            self.pin_input = input;
            self.error = None;
        }
    }

    pub fn try_login(&mut self, is_root: bool) -> bool {
        // PIN должен быть 4-6 цифр
        if self.pin_input.len() < 4 {
            self.error = Some("PIN must be 4-6 digits".to_string());
            return false;
        }

        // Mock PINs для demo:
        // User: 1234
        // Root: 0000
        let valid = if is_root {
            self.pin_input == "0000"
        } else {
            self.pin_input == "1234"
        };

        if valid {
            self.is_authenticated = true;
            self.is_admin = is_root;
            self.pin_input.clear();
            self.failed_attempts = 0;
            true
        } else {
            self.failed_attempts += 1;
            self.error = Some(format!(
                "Invalid PIN (attempt {}/3)",
                self.failed_attempts
            ));
            self.pin_input.clear();
            false
        }
    }

    pub fn lock(&mut self) {
        self.is_authenticated = false;
        self.pin_input.clear();
        self.failed_attempts = 0;
    }

    pub fn is_authenticated(&self) -> bool {
        self.is_authenticated
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    pub fn view(&self) -> Element<Message> {
        // Logo (небольшой, моноширинный)
        let logo_text = text("NEUROGRAPH")
            .size(text_size::DISPLAY)
            .style(iced::theme::Text::Color(TerminalColors::ACCENT_PRIMARY));

        // Подпись
        let subtitle = text("Enter PIN")
            .size(text_size::LG)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY));

        // PIN визуализация (● ● ● ○ ○ ○)
        let pin_dots = self.render_pin_dots();

        // Hidden input для ввода PIN
        let pin_input = text_input("", &self.pin_input)
            .on_input(Message::PasswordInput)
            .on_submit(Message::LoginAttempt(false))
            .padding(spacing::BASE)
            .size(text_size::LG)
            .width(Length::Fixed(200.0));

        // Error message
        let error_msg = if let Some(err) = &self.error {
            text(err)
                .size(text_size::SM)
                .style(iced::theme::Text::Color(TerminalColors::STATUS_CRITICAL))
        } else {
            text("")
        };

        // Buttons
        let buttons = row![
            button(
                text("User")
                    .size(text_size::BASE)
                    .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY))
            )
            .on_press(Message::LoginAttempt(false))
            .padding(spacing::BASE)
            .style(iced::theme::Button::Custom(Box::new(PinButtonStyle))),
            button(
                text("Root")
                    .size(text_size::BASE)
                    .style(iced::theme::Text::Color(TerminalColors::STATUS_WARNING))
            )
            .on_press(Message::LoginAttempt(true))
            .padding(spacing::BASE)
            .style(iced::theme::Button::Custom(Box::new(PinButtonStyle))),
        ]
        .spacing(spacing::BASE as f32);

        // Hint
        let hint = text("User PIN: 1234 | Root PIN: 0000")
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_MUTED));

        // Version
        let version = text("v0.46.0")
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_MUTED));

        // Card content
        let card_content = column![
            logo_text,
            subtitle,
            pin_dots,
            pin_input,
            error_msg,
            buttons,
            hint,
        ]
        .spacing(spacing::LG as f32)
        .padding(spacing::XXL)
        .width(Length::Fixed(400.0))
        .align_items(iced::Alignment::Center);

        // Card container
        let card = container(card_content)
            .style(iced::theme::Container::Custom(Box::new(PinCardStyle)))
            .width(Length::Shrink)
            .height(Length::Shrink);

        // Full screen container
        let full_screen = column![card, version]
            .spacing(spacing::XL as f32)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_items(iced::Alignment::Center)
            .padding(spacing::XXL);

        container(full_screen)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(iced::theme::Container::Custom(Box::new(AuthBgStyle)))
            .into()
    }

    fn render_pin_dots(&self) -> Element<Message> {
        let filled = self.pin_input.len();
        let total = 6;

        let mut dots = row![].spacing(spacing::SM as f32);

        for i in 0..total {
            let dot_char = if i < filled { "●" } else { "○" };
            let dot_color = if i < filled {
                TerminalColors::ACCENT_PRIMARY
            } else {
                TerminalColors::TEXT_MUTED
            };

            dots = dots.push(
                text(dot_char)
                    .size(text_size::XXL)
                    .style(iced::theme::Text::Color(dot_color)),
            );
        }

        dots.into()
    }
}

// ============================================================================
// Styles
// ============================================================================

struct AuthBgStyle;

impl iced::widget::container::StyleSheet for AuthBgStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_PRIMARY)),
            ..Default::default()
        }
    }
}

struct PinCardStyle;

impl iced::widget::container::StyleSheet for PinCardStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_SECONDARY)),
            border: iced::Border {
                color: TerminalColors::BORDER_DEFAULT,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        }
    }
}

struct PinButtonStyle;

impl iced::widget::button::StyleSheet for PinButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_HOVER)),
            border: iced::Border {
                color: TerminalColors::BORDER_DEFAULT,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_ACTIVE)),
            border: iced::Border {
                color: TerminalColors::ACCENT_PRIMARY,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }
}
