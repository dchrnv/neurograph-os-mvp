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

// Workspaces - Different screens

use iced::{widget::{column, text, button, container, row, text_input, scrollable, Space}, Element, Length};
use crate::{app::{Message, ChatMessage, MessageRole}, auth::AuthState, core::CoreBridge};
use crate::theme::{TerminalColors, text_size, spacing};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Workspace {
    Welcome,
    Dashboard,  // Главный экран с обзором (V3)
    Chat,
    Modules,
    Logs,       // Новый - просмотр логов (V3)
    Settings,
    Integrations, // Новый - внешние интеграции (V3)
    Status,     // Deprecated - будет заменен на Dashboard
    Admin,
}

// Unicode иконки для Sidebar (из спецификации DESKTOP_UI_SPEC_V3.md)
impl Workspace {
    fn icon(&self) -> &'static str {
        match self {
            Workspace::Welcome => "□",        // Главная
            Workspace::Dashboard => "□",      // Dashboard
            Workspace::Chat => "💬",          // Чат
            Workspace::Modules => "◧",        // Модули
            Workspace::Logs => "☰",           // Логи
            Workspace::Settings => "⚙",       // Настройки
            Workspace::Integrations => "⊕",   // Интеграции
            Workspace::Status => "◉",         // Статус (deprecated)
            Workspace::Admin => "!",          // Админ
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Workspace::Welcome => "Welcome",
            Workspace::Dashboard => "Dashboard",
            Workspace::Chat => "Chat",
            Workspace::Modules => "Modules",
            Workspace::Logs => "Logs",
            Workspace::Settings => "Settings",
            Workspace::Integrations => "Integrations",
            Workspace::Status => "Status",
            Workspace::Admin => "Admin",
        }
    }
}

pub trait WorkspaceView {
    fn view<'a>(
        &self,
        auth: &AuthState,
        core: &CoreBridge,
        chat_input: &'a str,
        chat_history: &'a [ChatMessage],
        chat_scroll: &scrollable::Id,
        chat_mode: crate::app::ChatMode,  // V3: Chat/Terminal tabs
    ) -> Element<'a, Message>;
}

impl WorkspaceView for Workspace {
    fn view<'a>(
        &self,
        auth: &AuthState,
        core: &CoreBridge,
        chat_input: &'a str,
        chat_history: &'a [ChatMessage],
        chat_scroll: &scrollable::Id,
        chat_mode: crate::app::ChatMode,  // V3: Chat/Terminal tabs
    ) -> Element<'a, Message> {
        // Sidebar (left navigation) - Terminal Modern стиль V3
        let mut dock_buttons = vec![
            dock_button(Workspace::Dashboard, *self),
            dock_button(Workspace::Chat, *self),
            dock_button(Workspace::Modules, *self),
            dock_button(Workspace::Logs, *self),
            dock_button(Workspace::Settings, *self),
            dock_button(Workspace::Integrations, *self),
        ];

        if auth.is_admin() {
            dock_buttons.push(dock_button(Workspace::Admin, *self));
        }

        let dock = container(
            column(dock_buttons)
                .spacing(spacing::SM as f32)
                .padding(spacing::BASE as f32)
        )
        .style(iced::theme::Container::Custom(Box::new(DockStyle)))
        .width(80)
        .height(Length::Fill);

        // Main content area
        let content = match self {
            Workspace::Welcome => welcome_view(auth),
            Workspace::Dashboard => dashboard_view(core),
            Workspace::Chat => chat_view(auth, chat_input, chat_history, chat_scroll, chat_mode),  // V3: Pass chat_mode
            Workspace::Modules => modules_view(),
            Workspace::Logs => logs_view(),
            Workspace::Settings => settings_view(),
            Workspace::Integrations => integrations_view(),
            Workspace::Status => status_view(core), // Deprecated
            Workspace::Admin => admin_view(),
        };

        let main_area = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20);

        row![dock, main_area].into()
    }
}

fn dock_button<'a>(workspace: Workspace, current: Workspace) -> Element<'a, Message> {
    let is_active = workspace == current;

    let btn = button(
        column![
            text(workspace.icon())
                .size(text_size::XL)
                .style(iced::theme::Text::Color(
                    if is_active { TerminalColors::ACCENT_PRIMARY } else { TerminalColors::TEXT_SECONDARY }
                )),
        ]
        .align_items(iced::Alignment::Center)
        .width(Length::Fill)
    )
    .on_press(Message::SwitchWorkspace(workspace))
    .padding(spacing::BASE)
    .width(Length::Fill);

    if is_active {
        btn.style(iced::theme::Button::Custom(Box::new(ActiveDockButton)))
    } else {
        btn.style(iced::theme::Button::Custom(Box::new(InactiveDockButton)))
    }
    .into()
}

// Кастомные стили для Dock
struct DockStyle;

impl iced::widget::container::StyleSheet for DockStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_SECONDARY)),
            border: iced::Border {
                color: TerminalColors::BORDER_SUBTLE,
                width: 1.0,
                radius: 0.0.into(),
            },
            ..Default::default()
        }
    }
}

struct ActiveDockButton;

impl iced::widget::button::StyleSheet for ActiveDockButton {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_ACTIVE)),
            border: iced::Border {
                color: TerminalColors::ACCENT_PRIMARY,
                width: 2.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let active = self.active(style);
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_HOVER)),
            ..active
        }
    }
}

struct InactiveDockButton;

impl iced::widget::button::StyleSheet for InactiveDockButton {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_SECONDARY)),
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
            background: Some(iced::Background::Color(TerminalColors::BG_HOVER)),
            border: iced::Border {
                color: TerminalColors::TEXT_MUTED,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }
}

fn welcome_view<'a>(auth: &AuthState) -> Element<'a, Message> {
    column![
        text("Welcome to NeuroGraph OS").size(28),
        text(""),
        text(include_str!("../assets/logo.txt"))
            .font(iced::Font::MONOSPACE)
            .size(10),
        text(""),
        text(format!(
            "Mode: {}",
            if auth.is_admin() { "Admin" } else { "User" }
        ))
        .size(16),
        text(""),
        text("System: Running (Direct Rust Core)").size(14),
        text(""),
        text("→ Click Chat to begin").size(14),
        text(""),
        button("[Lock]").on_press(Message::Lock).padding(10),
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center)
    .width(Length::Fill)
    .into()
}

fn chat_view<'a>(auth: &AuthState, chat_input: &'a str, chat_history: &'a [ChatMessage], chat_scroll_id: &scrollable::Id, chat_mode: crate::app::ChatMode) -> Element<'a, Message> {
    use crate::app::ChatMode;
    let is_root = auth.is_admin();
    // История сообщений
    let mut messages = column![].spacing(10);

    if chat_history.is_empty() {
        messages = messages.push(
            text("Введи команду для начала работы.\nНапример: 'status', 'create token', 'help'")
                .size(14)
                .style(iced::theme::Text::Color(iced::Color::from_rgb(0.6, 0.6, 0.6)))
        );
    } else {
        for msg in chat_history {
            let (prefix, color) = match msg.role {
                MessageRole::User => ("User:", iced::Color::from_rgb(0.2, 0.6, 1.0)), // Синий
                MessageRole::System => ("System:", iced::Color::from_rgb(0.2, 0.8, 0.4)), // Зеленый
            };

            messages = messages.push(
                column![
                    text(prefix).size(12).style(iced::theme::Text::Color(color)),
                    text(&msg.content)
                        .size(14)
                        .font(iced::Font::MONOSPACE),
                ]
                .spacing(5)
            );
        }
    }

    let history_scroll = scrollable(
        container(messages)
            .padding(10)
            .width(Length::Fill)
    )
    .id(chat_scroll_id.clone())
    .height(Length::Fill);

    // Input area
    let input_field = text_input("Введи команду...", chat_input)
        .on_input(Message::ChatInput)
        .on_submit(Message::SendMessage)
        .padding(10)
        .size(16);

    let send_button = button(text("Send").size(16))
        .on_press(Message::SendMessage)
        .padding(10);

    let input_row = row![input_field, send_button]
        .spacing(10)
        .padding(10);

    // V3: Tab-based header (Chat / Terminal)
    let chat_tab = button(
        text("Chat")
            .size(text_size::BASE)
            .style(iced::theme::Text::Color(
                if chat_mode == ChatMode::Chat { TerminalColors::TEXT_PRIMARY } else { TerminalColors::TEXT_SECONDARY }
            ))
    )
    .on_press(Message::SwitchChatMode(ChatMode::Chat))
    .padding(spacing::BASE)
    .style(iced::theme::Button::Custom(Box::new(TabButtonStyle { is_active: chat_mode == ChatMode::Chat })));

    let terminal_tab = button(
        text("Terminal")
            .size(text_size::BASE)
            .style(iced::theme::Text::Color(
                if chat_mode == ChatMode::Terminal { TerminalColors::TEXT_PRIMARY } else { TerminalColors::TEXT_SECONDARY }
            ))
    )
    .on_press(Message::SwitchChatMode(ChatMode::Terminal))
    .padding(spacing::BASE)
    .style(iced::theme::Button::Custom(Box::new(TabButtonStyle { is_active: chat_mode == ChatMode::Terminal })));

    let mode_badge = container(
        text(if is_root { "ROOT" } else { "USER" })
            .size(text_size::XS)
            .style(iced::theme::Text::Color(
                if is_root { TerminalColors::STATUS_WARNING } else { TerminalColors::ACCENT_PRIMARY }
            ))
    )
    .padding(spacing::SM)
    .style(iced::theme::Container::Custom(Box::new(ModeBadge { is_root })));

    let header = row![
        chat_tab,
        terminal_tab,
        Space::with_width(Length::Fill),
        mode_badge,
    ]
    .spacing(spacing::XS as f32)
    .align_items(iced::Alignment::Center);

    // V3: Conditional content based on tab
    let content_area = if chat_mode == ChatMode::Chat {
        // Chat mode - show message history
        column![
            header,
            container(history_scroll)
                .height(Length::Fill)
                .width(Length::Fill)
                .style(iced::theme::Container::Custom(Box::new(ChatHistoryStyle))),
            input_row,
        ]
        .spacing(spacing::BASE as f32)
        .height(Length::Fill)
    } else {
        // Terminal mode - show command prompt
        let terminal_content = render_terminal_view(chat_input, chat_history, is_root);
        column![
            header,
            terminal_content,
            input_row,
        ]
        .spacing(spacing::BASE as f32)
        .height(Length::Fill)
    };

    // Собираем все вместе
    let chat_container = container(content_area)
        .padding(spacing::LG)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(iced::theme::Container::Custom(Box::new(ChatContainer { is_root })));

    chat_container.into()
}

// V3: Terminal view with prompt
fn render_terminal_view<'a>(_chat_input: &'a str, chat_history: &'a [ChatMessage], is_root: bool) -> Element<'a, Message> {
    let prompt_symbol = if is_root { "root@neurograph #" } else { "user@neurograph $" };
    let prompt_color = if is_root { TerminalColors::STATUS_WARNING } else { TerminalColors::ACCENT_PRIMARY };

    // Terminal history (only last 10 commands)
    let mut terminal_lines = column![].spacing(spacing::SM as f32);

    for msg in chat_history.iter().rev().take(10).rev() {
        if msg.role == MessageRole::User {
            terminal_lines = terminal_lines.push(
                row![
                    text(prompt_symbol)
                        .size(text_size::SM)
                        .font(iced::Font::MONOSPACE)
                        .style(iced::theme::Text::Color(prompt_color)),
                    text(&msg.content)
                        .size(text_size::SM)
                        .font(iced::Font::MONOSPACE)
                        .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
                ]
                .spacing(spacing::SM as f32)
            );
        } else {
            terminal_lines = terminal_lines.push(
                text(&msg.content)
                    .size(text_size::SM)
                    .font(iced::Font::MONOSPACE)
                    .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY))
            );
        }
    }

    container(terminal_lines)
        .height(Length::Fill)
        .width(Length::Fill)
        .padding(spacing::BASE)
        .style(iced::theme::Container::Custom(Box::new(TerminalScreenStyle)))
        .into()
}

// Стили для Chat режимов
struct ChatContainer {
    is_root: bool,
}

impl iced::widget::container::StyleSheet for ChatContainer {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        if self.is_root {
            iced::widget::container::Appearance {
                background: Some(iced::Background::Color(TerminalColors::BG_PRIMARY)),
                border: iced::Border {
                    color: TerminalColors::STATUS_WARNING,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            }
        } else {
            iced::widget::container::Appearance {
                background: Some(iced::Background::Color(TerminalColors::BG_PRIMARY)),
                border: iced::Border {
                    color: TerminalColors::ACCENT_PRIMARY,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..Default::default()
            }
        }
    }
}

struct ChatHistoryStyle;

impl iced::widget::container::StyleSheet for ChatHistoryStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_SECONDARY)),
            border: iced::Border {
                color: TerminalColors::BORDER_SUBTLE,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
    }
}

// V3: Tab button style
struct TabButtonStyle {
    is_active: bool,
}

impl iced::widget::button::StyleSheet for TabButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: if self.is_active {
                Some(iced::Background::Color(TerminalColors::BG_ACTIVE))
            } else {
                Some(iced::Background::Color(TerminalColors::BG_SECONDARY))
            },
            border: iced::Border {
                color: if self.is_active { TerminalColors::ACCENT_PRIMARY } else { TerminalColors::BORDER_SUBTLE },
                width: if self.is_active { 2.0 } else { 1.0 },
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_HOVER)),
            border: iced::Border {
                color: TerminalColors::TEXT_SECONDARY,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }
}

// V3: Terminal screen style
struct TerminalScreenStyle;

impl iced::widget::container::StyleSheet for TerminalScreenStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_SECONDARY)),
            border: iced::Border {
                color: TerminalColors::BORDER_DEFAULT,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
    }
}

struct ModeBadge {
    is_root: bool,
}

impl iced::widget::container::StyleSheet for ModeBadge {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        if self.is_root {
            iced::widget::container::Appearance {
                background: Some(iced::Background::Color(TerminalColors::BG_SECONDARY)),
                border: iced::Border {
                    color: TerminalColors::STATUS_WARNING,
                    width: 1.0,
                    radius: 12.0.into(),
                },
                ..Default::default()
            }
        } else {
            iced::widget::container::Appearance {
                background: Some(iced::Background::Color(TerminalColors::BG_ACTIVE)),
                border: iced::Border {
                    color: TerminalColors::ACCENT_PRIMARY,
                    width: 1.0,
                    radius: 12.0.into(),
                },
                ..Default::default()
            }
        }
    }
}

fn settings_view<'a>() -> Element<'a, Message> {
    column![
        text("Settings").size(24),
        text(""),
        text("Module Configurations:").size(16),
        text("• Token Config").size(14),
        text("• Connection Config").size(14),
        text("• Grid Config").size(14),
        text("• Graph Config").size(14),
        text("• Guardian Config").size(14),
        text(""),
        text("Phase 3: Config management").size(12),
    ]
    .spacing(10)
    .into()
}

fn status_view<'a>(core: &CoreBridge) -> Element<'a, Message> {
    let status = core.process_message("status");

    column![
        text("System Status").size(24),
        text(""),
        text(status)
            .font(iced::Font::MONOSPACE)
            .size(14),
        text(""),
        text("Phase 5: Real-time monitoring").size(12),
    ]
    .spacing(10)
    .into()
}

fn modules_view<'a>() -> Element<'a, Message> {
    column![
        text("Module Manager")
            .size(text_size::XXL)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
        text(""),
        text("Системные модули:")
            .size(text_size::LG)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text(""),
        module_status_item("Token Manager", "RUNNING", true),
        module_status_item("Connection Pool", "RUNNING", true),
        module_status_item("Grid Index", "RUNNING", true),
        module_status_item("Graph Engine", "RUNNING", true),
        module_status_item("Guardian", "RUNNING", true),
        text(""),
        text("Phase 6: Модульная система управления")
            .size(text_size::SM)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_MUTED)),
    ]
    .spacing(spacing::SM as f32)
    .padding(spacing::LG)
    .into()
}

fn module_status_item<'a>(name: &str, status: &str, running: bool) -> Element<'a, Message> {
    let status_color = if running { TerminalColors::STATUS_OK } else { TerminalColors::TEXT_MUTED };

    container(
        row![
            text(format!("▶ {}", name))
                .size(text_size::BASE)
                .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
            text(format!("[{}]", status))
                .size(text_size::SM)
                .style(iced::theme::Text::Color(status_color)),
        ]
        .spacing(spacing::LG as f32)
        .align_items(iced::Alignment::Center)
    )
    .padding(spacing::SM)
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(ModuleItemStyle)))
    .into()
}

struct ModuleItemStyle;

impl iced::widget::container::StyleSheet for ModuleItemStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_SECONDARY)),
            border: iced::Border {
                color: TerminalColors::BORDER_DEFAULT,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
    }
}

fn admin_view<'a>() -> Element<'a, Message> {
    column![
        text("Admin Panel")
            .size(text_size::XXL)
            .style(iced::theme::Text::Color(TerminalColors::STATUS_WARNING)),
        text("!!! CRITICAL CHANGES !!!")
            .size(text_size::LG)
            .style(iced::theme::Text::Color(TerminalColors::STATUS_CRITICAL)),
        text(""),
        text("Phase 4: CDNA configuration").size(text_size::BASE),
        text("Direct access to Guardian & CDNA").size(text_size::SM),
    ]
    .spacing(spacing::BASE as f32)
    .padding(spacing::LG)
    .into()
}

// ============================================================================
// New V3 Views (заглушки - будут реализованы позже)
// ============================================================================

fn dashboard_view<'a>(core: &CoreBridge) -> Element<'a, Message> {
    // Get current metrics from core
    let stats = core.get_stats();
    let token_count = stats.tokens.len();
    let active_connections = 12; // Mock for now
    let memory_mb = 256; // Mock
    let throughput = 1250; // req/s - mock
    let latency_ms = 15; // Mock
    let uptime = "2h 34m"; // Mock

    // Metric Cards Grid (3 columns)
    let metrics_row_1 = row![
        metric_card("TOKENS", &token_count.to_string(), "●", TerminalColors::ACCENT_PRIMARY),
        metric_card("CONNECTIONS", &active_connections.to_string(), "●", TerminalColors::STATUS_OK),
        metric_card("MEMORY", &format!("{} MB", memory_mb), "◆", TerminalColors::STATUS_INFO),
    ]
    .spacing(spacing::BASE as f32);

    let metrics_row_2 = row![
        metric_card("THROUGHPUT", &format!("{} req/s", throughput), "▲", TerminalColors::ACCENT_PRIMARY),
        metric_card("LATENCY", &format!("{} ms", latency_ms), "◐", TerminalColors::STATUS_OK),
        metric_card("UPTIME", uptime, "⏱", TerminalColors::TEXT_SECONDARY),
    ]
    .spacing(spacing::BASE as f32);

    // Modules Section (status indicators)
    let modules_title = text("MODULES")
        .size(text_size::SM)
        .style(iced::theme::Text::Color(TerminalColors::TEXT_MUTED));

    let modules_grid = row![
        module_status_card("Core Engine", true, false),
        module_status_card("API Gateway", true, false),
        module_status_card("WebSocket", true, false),
        module_status_card("Database", true, false),
    ]
    .spacing(spacing::BASE as f32);

    // Quick Actions
    let actions_title = text("QUICK ACTIONS")
        .size(text_size::SM)
        .style(iced::theme::Text::Color(TerminalColors::TEXT_MUTED));

    let actions_row = row![
        quick_action_button("New Chat", "💬"),
        quick_action_button("View Logs", "📋"),
        quick_action_button("Restart All", "🔄"),
        quick_action_button("Export Config", "⚙"),
    ]
    .spacing(spacing::BASE as f32);

    // Full Dashboard Layout
    column![
        text("Dashboard")
            .size(text_size::XXL)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
        text("System Overview")
            .size(text_size::SM)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text(""),
        metrics_row_1,
        metrics_row_2,
        text(""),
        modules_title,
        modules_grid,
        text(""),
        actions_title,
        actions_row,
    ]
    .spacing(spacing::LG as f32)
    .padding(spacing::XL)
    .into()
}

// ============================================================================
// Dashboard Helper Components
// ============================================================================

fn metric_card<'a>(label: &str, value: &str, icon: &str, accent_color: iced::Color) -> Element<'a, Message> {
    container(
        column![
            row![
                text(icon)
                    .size(text_size::LG)
                    .style(iced::theme::Text::Color(accent_color)),
                text(label)
                    .size(text_size::XS)
                    .style(iced::theme::Text::Color(TerminalColors::TEXT_MUTED)),
            ]
            .spacing(spacing::XS as f32)
            .align_items(iced::Alignment::Center),
            text(value)
                .size(text_size::XXL)
                .font(iced::Font::MONOSPACE)
                .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
        ]
        .spacing(spacing::SM as f32)
        .padding(spacing::BASE)
        .align_items(iced::Alignment::Start)
    )
    .style(iced::theme::Container::Custom(Box::new(MetricCardStyle)))
    .width(Length::Fill)
    .into()
}

fn module_status_card<'a>(name: &str, is_running: bool, _is_starting: bool) -> Element<'a, Message> {
    let (status_color, status_text) = if is_running {
        (TerminalColors::STATUS_OK, "Running")
    } else {
        (TerminalColors::STATUS_CRITICAL, "Stopped")
    };

    container(
        column![
            text(name)
                .size(text_size::SM)
                .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
            row![
                text("●")
                    .size(text_size::SM)
                    .style(iced::theme::Text::Color(status_color)),
                text(status_text)
                    .size(text_size::XS)
                    .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
            ]
            .spacing(spacing::XS as f32)
            .align_items(iced::Alignment::Center),
        ]
        .spacing(spacing::XS as f32)
        .padding(spacing::BASE)
    )
    .style(iced::theme::Container::Custom(Box::new(ModuleCardStyle)))
    .width(Length::Fill)
    .into()
}

fn quick_action_button<'a>(label: &str, icon: &str) -> Element<'a, Message> {
    button(
        row![
            text(icon).size(text_size::BASE),
            text(label)
                .size(text_size::SM)
                .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
        ]
        .spacing(spacing::SM as f32)
        .align_items(iced::Alignment::Center)
    )
    .padding(spacing::BASE)
    .style(iced::theme::Button::Custom(Box::new(QuickActionButtonStyle)))
    .into()
}

// ============================================================================
// Dashboard Card Styles
// ============================================================================

struct MetricCardStyle;

impl iced::widget::container::StyleSheet for MetricCardStyle {
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

struct ModuleCardStyle;

impl iced::widget::container::StyleSheet for ModuleCardStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_SECONDARY)),
            border: iced::Border {
                color: TerminalColors::BORDER_SUBTLE,
                width: 1.0,
                radius: 6.0.into(),
            },
            ..Default::default()
        }
    }
}

struct QuickActionButtonStyle;

impl iced::widget::button::StyleSheet for QuickActionButtonStyle {
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

// ============================================================================
// Logs Screen Styles (V3 - Phase 6)
// ============================================================================

struct LogsContainerStyle;

impl iced::widget::container::StyleSheet for LogsContainerStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_SECONDARY)),
            border: iced::Border {
                color: TerminalColors::BORDER_SUBTLE,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
    }
}

struct LogFilterButtonStyle {
    is_active: bool,
}

impl iced::widget::button::StyleSheet for LogFilterButtonStyle {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: if self.is_active {
                Some(iced::Background::Color(TerminalColors::ACCENT_PRIMARY))
            } else {
                Some(iced::Background::Color(TerminalColors::BG_SECONDARY))
            },
            border: iced::Border {
                color: if self.is_active { TerminalColors::ACCENT_PRIMARY } else { TerminalColors::BORDER_DEFAULT },
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_HOVER)),
            border: iced::Border {
                color: TerminalColors::TEXT_SECONDARY,
                width: 1.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
    }
}

struct LogEntryStyle;

impl iced::widget::container::StyleSheet for LogEntryStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(TerminalColors::BG_PRIMARY)),
            border: iced::Border {
                color: TerminalColors::BORDER_SUBTLE,
                width: 0.0,
                radius: 4.0.into(),
            },
            ..Default::default()
        }
    }
}

struct LogLevelBadgeStyle {
    level: LogLevel,
}

impl iced::widget::container::StyleSheet for LogLevelBadgeStyle {
    type Style = iced::Theme;

    fn appearance(&self, _style: &Self::Style) -> iced::widget::container::Appearance {
        let bg_color = match self.level {
            LogLevel::Error => iced::Color::from_rgba(0.973, 0.318, 0.286, 0.15),  // Semi-transparent red
            LogLevel::Warn => iced::Color::from_rgba(0.824, 0.600, 0.133, 0.15),   // Semi-transparent yellow
            LogLevel::Info => iced::Color::from_rgba(0.345, 0.651, 1.0, 0.15),     // Semi-transparent blue
            LogLevel::Debug => iced::Color::from_rgba(0.5, 0.5, 0.5, 0.1),         // Semi-transparent gray
        };

        iced::widget::container::Appearance {
            background: Some(iced::Background::Color(bg_color)),
            border: iced::Border {
                color: iced::Color::TRANSPARENT,
                width: 0.0,
                radius: 3.0.into(),
            },
            ..Default::default()
        }
    }
}

fn logs_view<'a>() -> Element<'a, Message> {
    // Mock log entries
    let mock_logs = vec![
        LogEntry { level: LogLevel::Info, timestamp: "12:34:56", message: "System initialized successfully" },
        LogEntry { level: LogLevel::Info, timestamp: "12:35:02", message: "WebSocket server listening on 0.0.0.0:8080" },
        LogEntry { level: LogLevel::Warn, timestamp: "12:35:15", message: "High memory usage detected: 85%" },
        LogEntry { level: LogLevel::Error, timestamp: "12:35:22", message: "Failed to connect to database: timeout" },
        LogEntry { level: LogLevel::Info, timestamp: "12:35:30", message: "Token created: tk_abc123" },
        LogEntry { level: LogLevel::Debug, timestamp: "12:35:45", message: "Processing request from 192.168.1.100" },
        LogEntry { level: LogLevel::Info, timestamp: "12:36:01", message: "Checkpoint saved successfully" },
        LogEntry { level: LogLevel::Warn, timestamp: "12:36:12", message: "Slow query detected: 2.5s" },
    ];

    // Header with title and filters
    let title = text("System Logs")
        .size(text_size::XXL)
        .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY));

    let filter_buttons = row![
        log_filter_button("All", true),
        log_filter_button("Error", false),
        log_filter_button("Warn", false),
        log_filter_button("Info", false),
        log_filter_button("Debug", false),
    ]
    .spacing(spacing::SM as f32);

    let header = row![
        title,
        Space::with_width(Length::Fill),
        filter_buttons,
    ]
    .spacing(spacing::BASE as f32)
    .align_items(iced::Alignment::Center);

    // Log entries list
    let mut log_list = column![].spacing(spacing::XS as f32);

    for log in mock_logs {
        log_list = log_list.push(log_entry_view(&log));
    }

    let logs_container = container(
        scrollable(log_list)
            .height(Length::Fill)
    )
    .padding(spacing::BASE)
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(LogsContainerStyle)));

    // Footer with stats
    let footer = row![
        text("Total: 8")
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text("|")
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_MUTED)),
        text("Errors: 1")
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::STATUS_CRITICAL)),
        text("|")
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_MUTED)),
        text("Warnings: 2")
            .size(text_size::XS)
            .style(iced::theme::Text::Color(TerminalColors::STATUS_WARNING)),
    ]
    .spacing(spacing::SM as f32)
    .align_items(iced::Alignment::Center);

    column![
        header,
        logs_container,
        footer,
    ]
    .spacing(spacing::BASE as f32)
    .padding(spacing::LG)
    .into()
}

// Log entry data structure
#[derive(Debug, Clone)]
struct LogEntry {
    level: LogLevel,
    timestamp: &'static str,
    message: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

// Helper functions for logs view
fn log_filter_button<'a>(label: &str, is_active: bool) -> Element<'a, Message> {
    button(
        text(label)
            .size(text_size::SM)
            .style(iced::theme::Text::Color(
                if is_active { TerminalColors::TEXT_PRIMARY } else { TerminalColors::TEXT_SECONDARY }
            ))
    )
    .padding(spacing::SM)
    .style(iced::theme::Button::Custom(Box::new(LogFilterButtonStyle { is_active })))
    .into()
}

fn log_entry_view<'a>(log: &LogEntry) -> Element<'a, Message> {
    let (level_text, level_color) = match log.level {
        LogLevel::Error => ("ERROR", TerminalColors::STATUS_CRITICAL),
        LogLevel::Warn => ("WARN ", TerminalColors::STATUS_WARNING),
        LogLevel::Info => ("INFO ", TerminalColors::STATUS_INFO),
        LogLevel::Debug => ("DEBUG", TerminalColors::TEXT_MUTED),
    };

    container(
        row![
            text(log.timestamp)
                .size(text_size::SM)
                .font(iced::Font::MONOSPACE)
                .style(iced::theme::Text::Color(TerminalColors::TEXT_MUTED)),
            container(
                text(level_text)
                    .size(text_size::XS)
                    .font(iced::Font::MONOSPACE)
                    .style(iced::theme::Text::Color(level_color))
            )
            .padding(spacing::XS)
            .style(iced::theme::Container::Custom(Box::new(LogLevelBadgeStyle { level: log.level }))),
            text(log.message)
                .size(text_size::SM)
                .font(iced::Font::MONOSPACE)
                .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
        ]
        .spacing(spacing::BASE as f32)
        .align_items(iced::Alignment::Center)
    )
    .padding(spacing::SM)
    .width(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(LogEntryStyle)))
    .into()
}

fn integrations_view<'a>() -> Element<'a, Message> {
    column![
        text("Integrations")
            .size(text_size::XXL)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_PRIMARY)),
        text(""),
        text("Внешние интеграции:")
            .size(text_size::LG)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text(""),
        text("• Neural Networks / LLMs")
            .size(text_size::SM)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text("• Embeddings (GloVe, Word2Vec)")
            .size(text_size::SM)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text("• Knowledge Bases")
            .size(text_size::SM)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
        text("• Tools (Web search, Code execution)")
            .size(text_size::SM)
            .style(iced::theme::Text::Color(TerminalColors::TEXT_SECONDARY)),
    ]
    .spacing(spacing::BASE as f32)
    .padding(spacing::LG)
    .into()
}
