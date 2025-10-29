// Workspaces - Different screens

use iced::{widget::{column, text, button, container, row, text_input, scrollable}, Element, Length};
use crate::{app::Message, auth::AuthState, core::CoreBridge};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Workspace {
    Welcome,
    Chat,
    Settings,
    Status,
    Admin,
}

pub trait WorkspaceView {
    fn view(&self, auth: &AuthState, core: &CoreBridge) -> Element<Message>;
}

impl WorkspaceView for Workspace {
    fn view(&self, auth: &AuthState, core: &CoreBridge) -> Element<Message> {
        // Dock (left sidebar)
        let dock = column![
            dock_button("🏠", Workspace::Welcome, *self),
            dock_button("💬", Workspace::Chat, *self),
            dock_button("⚙️", Workspace::Settings, *self),
            dock_button("📊", Workspace::Status, *self),
            if auth.is_admin() {
                dock_button("🔒", Workspace::Admin, *self)
            } else {
                button("").width(0).into()
            },
        ]
        .spacing(10)
        .padding(10)
        .width(60);

        // Main content area
        let content = match self {
            Workspace::Welcome => welcome_view(auth),
            Workspace::Chat => chat_view(core),
            Workspace::Settings => settings_view(),
            Workspace::Status => status_view(core),
            Workspace::Admin => admin_view(),
        };

        let main_area = container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20);

        row![dock, main_area].into()
    }
}

fn dock_button(icon: &str, workspace: Workspace, current: Workspace) -> Element<'static, Message> {
    let btn = button(text(icon).size(24))
        .on_press(Message::SwitchWorkspace(workspace))
        .padding(10);

    if workspace == current {
        btn.style(iced::theme::Button::Primary)
    } else {
        btn.style(iced::theme::Button::Secondary)
    }
    .into()
}

fn welcome_view(auth: &AuthState) -> Element<'static, Message> {
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
        button("🔒 Lock").on_press(Message::Lock).padding(10),
    ]
    .spacing(10)
    .align_items(iced::Alignment::Center)
    .width(Length::Fill)
    .into()
}

fn chat_view(core: &CoreBridge) -> Element<'static, Message> {
    // Phase 1: Простая демонстрация что можем вызывать core
    let demo_status = core.process_message("status");

    column![
        text("Chat").size(24),
        text(""),
        text("Phase 1 Demo: Direct Core Integration").size(14),
        text("").size(8),
        text("Выполняем: core.process_message(\"status\")").size(12),
        text("").size(8),
        text(demo_status)
            .font(iced::Font::MONOSPACE)
            .size(12),
        text("").size(8),
        text("Phase 2: Полный UI чата с вводом").size(12),
    ]
    .spacing(10)
    .into()
}

fn settings_view() -> Element<'static, Message> {
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

fn status_view(core: &CoreBridge) -> Element<'static, Message> {
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

fn admin_view() -> Element<'static, Message> {
    column![
        text("Admin Panel").size(24),
        text("⚠️  CRITICAL CHANGES").size(18),
        text(""),
        text("Phase 4: CDNA configuration").size(14),
        text("Direct access to Guardian & CDNA").size(12),
    ]
    .spacing(10)
    .into()
}
