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

// Layout components - Header and Status Bar for Terminal Modern UI

use iced::{widget::{container, text, row, Space}, Element, Length, Alignment};
use crate::app::Message;
use crate::theme::{TerminalColors, text_size, spacing};
use crate::workspaces::Workspace;

// ============================================================================
// Header Component (top bar)
// ============================================================================

pub fn header<'a>(current_workspace: Workspace, system_status: SystemStatus) -> Element<'a, Message> {
    let logo = row![
        text("NEUROGRAPH")
            .size(text_size::BASE)
            .style(iced::theme::Text::Color(TerminalColors::ACCENT_PRIMARY)),
    ]
    .spacing(spacing::SM as f32)
    .align_items(Alignment::Center);

    let breadcrumb = text(workspace_title(current_workspace))
        .size(text_size::BASE)
        .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY));

    let status_indicator = status_dot(system_status);

    let header_content = row![
        logo,
        text(" / ").style(iced::theme::Text::Color(TerminalColors::TEXT_MUTED)),
        breadcrumb,
        Space::with_width(Length::Fill),
        status_indicator,
    ]
    .spacing(spacing::SM as f32)
    .padding(spacing::BASE)
    .align_items(Alignment::Center);

    container(header_content)
        .width(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(HeaderStyle)))
        .into()
}

fn workspace_title(workspace: Workspace) -> &'static str {
    match workspace {
        Workspace::Welcome => "Welcome",
        Workspace::Dashboard => "Dashboard",
        Workspace::Chat => "Chat",
        Workspace::Status => "Status",
        Workspace::Modules => "Modules",
        Workspace::Settings => "Settings",
        Workspace::Admin => "Admin",
        Workspace::Logs => "Logs",
        Workspace::Integrations => "Integrations",
    }
}

fn status_dot(status: SystemStatus) -> Element<'static, Message> {
    let (color, label) = match status {
        SystemStatus::Running => (TerminalColors::STATUS_OK, "Running"),
        SystemStatus::Starting => (TerminalColors::STATUS_WARNING, "Starting"),
        SystemStatus::Error => (TerminalColors::STATUS_CRITICAL, "Error"),
    };

    row![
        text("●")
            .size(text_size::SM)
            .style(iced::theme::Text::Color(color)),
        text(label)
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
    ]
    .spacing(spacing::XS as f32)
    .align_items(Alignment::Center)
    .into()
}

struct HeaderStyle;

impl iced::widget::container::StyleSheet for HeaderStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_SECONDARY)),
            border: iced::Border {
                color: TerminalColors::BORDER_SUBTLE,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
}

// ============================================================================
// Status Bar Component (bottom bar)
// ============================================================================

#[derive(Debug, Clone)]
pub struct StatusBarData {
    pub tokens_count: usize,
    pub connections_count: usize,
    pub memory_used: usize,
    pub memory_total: usize,
    pub cpu_usage: u8,
    pub uptime: String,
}

impl Default for StatusBarData {
    fn default() -> Self {
        Self {
            tokens_count: 0,
            connections_count: 0,
            memory_used: 0,
            memory_total: 2048,
            cpu_usage: 0,
            uptime: "0h 0m".to_string(),
        }
    }
}

pub fn status_bar(data: &StatusBarData) -> Element<Message> {
    let content = row![
        text("Tokens:")
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text(data.tokens_count.to_string())
            .size(text_size::XS)
            .font(iced::Font::MONOSPACE)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
        separator(),
        text("Connections:")
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text(data.connections_count.to_string())
            .size(text_size::XS)
            .font(iced::Font::MONOSPACE)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
        separator(),
        text("Memory:")
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text(format!("{}/{} MB", data.memory_used, data.memory_total))
            .size(text_size::XS)
            .font(iced::Font::MONOSPACE)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
        separator(),
        text("CPU:")
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text(format!("{}%", data.cpu_usage))
            .size(text_size::XS)
            .font(iced::Font::MONOSPACE)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
        separator(),
        text("Uptime:")
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text(&data.uptime)
            .size(text_size::XS)
            .font(iced::Font::MONOSPACE)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
    ]
    .spacing(spacing::XS as f32)
    .padding(spacing::XS)
    .align_items(Alignment::Center);

    container(content)
        .width(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(StatusBarStyle)))
        .into()
}

fn status_item_direct<'a>(label: &'static str, value: &'a str) -> Element<'a, Message> {
    row![
        text(label)
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text(value)
            .size(text_size::XS)
            .font(iced::Font::MONOSPACE)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
    ]
    .spacing(spacing::XS as f32)
    .align_items(Alignment::Center)
    .into()
}

fn separator<'a>() -> Element<'a, Message> {
    text("|")
        .size(text_size::XS)
        .style(iced::theme::Text::Color(TerminalColors::TEXT_MUTED))
        .into()
}

struct StatusBarStyle;

impl iced::widget::container::StyleSheet for StatusBarStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_SECONDARY)),
            border: iced::Border {
                color: TerminalColors::BORDER_SUBTLE,
                width: 0.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
}

// ============================================================================
// System Status Enum
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemStatus {
    Running,
    Starting,
    Error,
}
