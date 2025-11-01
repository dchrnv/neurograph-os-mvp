// Workspaces - Different screens

use iced::{widget::{column, text, button, container, row, text_input, scrollable}, Element, Length};
use crate::{app::{Message, ChatMessage, MessageRole}, auth::AuthState, core::CoreBridge};
use crate::theme::{CyberColors, text_size, spacing};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Workspace {
    Welcome,
    Chat,
    Settings,
    Status,
    Modules,  // Новый workspace для управления модулями
    Admin,
}

// ASCII иконки для Dock (из спецификации UI_Control_Panel_V2.md)
impl Workspace {
    fn icon(&self) -> &'static str {
        match self {
            Workspace::Welcome => "[≈]",   // Главная
            Workspace::Chat => "[◐]",      // Чат/диалог
            Workspace::Settings => "[⚙]",  // Настройки
            Workspace::Status => "[◉]",    // Статус/метрики
            Workspace::Modules => "[⬡]",   // Модули/граф
            Workspace::Admin => "[!]",     // Админ
        }
    }

    fn label(&self) -> &'static str {
        match self {
            Workspace::Welcome => "Home",
            Workspace::Chat => "Chat",
            Workspace::Settings => "Settings",
            Workspace::Status => "Metrics",
            Workspace::Modules => "Modules",
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
    ) -> Element<'a, Message> {
        // Dock (left sidebar) - Киберпанк стиль
        let mut dock_buttons = vec![
            dock_button(Workspace::Welcome, *self),
            dock_button(Workspace::Chat, *self),
            dock_button(Workspace::Status, *self),
            dock_button(Workspace::Modules, *self),
            dock_button(Workspace::Settings, *self),
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
            Workspace::Chat => chat_view(auth, chat_input, chat_history, chat_scroll),
            Workspace::Settings => settings_view(),
            Workspace::Status => status_view(core),
            Workspace::Modules => modules_view(),
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
                    if is_active { CyberColors::ACCENT_PRIMARY } else { CyberColors::TEXT_SECONDARY }
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
            background: Some(iced::Background::Color(CyberColors::BG_SECONDARY)),
            border: iced::Border {
                color: CyberColors::BG_HOVER,
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
            background: Some(iced::Background::Color(CyberColors::BG_TERTIARY)),
            border: iced::Border {
                color: CyberColors::ACCENT_PRIMARY,
                width: 2.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        }
    }

    fn hovered(&self, style: &Self::Style) -> iced::widget::button::Appearance {
        let active = self.active(style);
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(CyberColors::BG_HOVER)),
            ..active
        }
    }
}

struct InactiveDockButton;

impl iced::widget::button::StyleSheet for InactiveDockButton {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(CyberColors::BG_SECONDARY)),
            border: iced::Border {
                color: CyberColors::BG_HOVER,
                width: 1.0,
                radius: 8.0.into(),
            },
            ..Default::default()
        }
    }

    fn hovered(&self, _style: &Self::Style) -> iced::widget::button::Appearance {
        iced::widget::button::Appearance {
            background: Some(iced::Background::Color(CyberColors::BG_HOVER)),
            border: iced::Border {
                color: CyberColors::TEXT_MUTED,
                width: 1.0,
                radius: 8.0.into(),
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

fn chat_view<'a>(auth: &AuthState, chat_input: &'a str, chat_history: &'a [ChatMessage], chat_scroll_id: &scrollable::Id) -> Element<'a, Message> {
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

    // Заголовок с индикатором режима
    let mode_indicator = if is_root { "ROOT" } else { "USER" };
    let mode_color = if is_root { CyberColors::MODE_ROOT } else { CyberColors::MODE_USER };

    let header = row![
        text("NeuroGraph Chat")
            .size(text_size::XXL)
            .style(iced::theme::Text::Color(CyberColors::TEXT_PRIMARY)),
        container(
            text(format!("Mode: {}", mode_indicator))
                .size(text_size::SM)
                .style(iced::theme::Text::Color(
                    if is_root { CyberColors::MODE_ROOT } else { CyberColors::ACCENT_PRIMARY }
                ))
        )
        .padding(spacing::SM)
        .style(iced::theme::Container::Custom(Box::new(ModeBadge { is_root }))),
    ]
    .spacing(spacing::LG as f32)
    .align_items(iced::Alignment::Center);

    // Собираем все вместе
    let chat_container = container(
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
    )
    .padding(spacing::LG)
    .width(Length::Fill)
    .height(Length::Fill)
    .style(iced::theme::Container::Custom(Box::new(ChatContainer { is_root })));

    chat_container.into()
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
                background: Some(iced::Background::Color(iced::Color::BLACK)),
                border: iced::Border {
                    color: CyberColors::MODE_ROOT,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                ..Default::default()
            }
        } else {
            iced::widget::container::Appearance {
                background: Some(iced::Background::Color(CyberColors::BG_PRIMARY)),
                border: iced::Border {
                    color: CyberColors::MODE_USER,
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
            background: Some(iced::Background::Color(CyberColors::BG_SECONDARY)),
            border: iced::Border {
                color: CyberColors::BG_HOVER,
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
                background: Some(iced::Background::Color(iced::Color::from_rgb(0.1, 0.04, 0.0))),
                border: iced::Border {
                    color: CyberColors::MODE_ROOT,
                    width: 1.0,
                    radius: 12.0.into(),
                },
                ..Default::default()
            }
        } else {
            iced::widget::container::Appearance {
                background: Some(iced::Background::Color(CyberColors::BG_TERTIARY)),
                border: iced::Border {
                    color: CyberColors::ACCENT_PRIMARY,
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
            .style(iced::theme::Text::Color(CyberColors::TEXT_PRIMARY)),
        text(""),
        text("Системные модули:")
            .size(text_size::LG)
            .style(iced::theme::Text::Color(CyberColors::TEXT_SECONDARY)),
        text(""),
        module_status_item("Token Manager", "RUNNING", true),
        module_status_item("Connection Pool", "RUNNING", true),
        module_status_item("Grid Index", "RUNNING", true),
        module_status_item("Graph Engine", "RUNNING", true),
        module_status_item("Guardian", "RUNNING", true),
        text(""),
        text("Phase 6: Модульная система управления")
            .size(text_size::SM)
            .style(iced::theme::Text::Color(CyberColors::TEXT_MUTED)),
    ]
    .spacing(spacing::SM as f32)
    .padding(spacing::LG)
    .into()
}

fn module_status_item<'a>(name: &str, status: &str, running: bool) -> Element<'a, Message> {
    let status_color = if running { CyberColors::STATUS_OK } else { CyberColors::TEXT_MUTED };

    container(
        row![
            text(format!("▶ {}", name))
                .size(text_size::BASE)
                .style(iced::theme::Text::Color(CyberColors::TEXT_PRIMARY)),
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
            background: Some(iced::Background::Color(CyberColors::BG_SECONDARY)),
            border: iced::Border {
                color: CyberColors::BG_HOVER,
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
            .style(iced::theme::Text::Color(CyberColors::MODE_ROOT)),
        text("!!! CRITICAL CHANGES !!!")
            .size(text_size::LG)
            .style(iced::theme::Text::Color(CyberColors::STATUS_CRITICAL)),
        text(""),
        text("Phase 4: CDNA configuration").size(text_size::BASE),
        text("Direct access to Guardian & CDNA").size(text_size::SM),
    ]
    .spacing(spacing::BASE as f32)
    .padding(spacing::LG)
    .into()
}
