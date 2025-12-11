// Chat/Terminal screen - Dual-mode interface
// Based on DESKTOP_UI_SPEC_V3.md section 6

use iced::widget::{column, container, row, text, text_input, button, scrollable, Space};
use iced::{Element, Length, Alignment};
use crate::theme::Theme;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChatMode {
    Chat,
    Terminal,
}

pub fn view<Message: 'static + Clone>(_theme: &Theme, mode: ChatMode, input: &str) -> Element<'static, Message> {
    let header = row![
        text(match mode {
            ChatMode::Chat => "Chat Mode",
            ChatMode::Terminal => "Terminal Mode",
        })
        .size(20),
        Space::with_width(Length::Fill),
        button(text("Switch Mode (Ctrl+T)").size(12))
            .padding([8, 16]),
    ]
    .padding(16)
    .align_y(Alignment::Center);

    // Message history (mock data)
    let messages = column![
        message_bubble("User".to_string(), "Hello, NeuroGraph!".to_string(), true),
        message_bubble("System".to_string(), "Hello! I'm NeuroGraph OS. How can I help you?".to_string(), false),
        message_bubble("User".to_string(), "Show me system status".to_string(), true),
        message_bubble("System".to_string(), "System Status:\n- CPU: 12%\n- Memory: 2.3 GB\n- Active Modules: 4".to_string(), false),
    ]
    .spacing(12)
    .padding(16);

    let messages_scroll = scrollable(messages)
        .height(Length::Fill);

    // Input area
    let input_field = text_input(
        match mode {
            ChatMode::Chat => "Type your message...",
            ChatMode::Terminal => "$ Enter command...",
        },
        input
    )
    .size(14)
    .padding(12);

    let send_button = button(
        text(match mode {
            ChatMode::Chat => "Send",
            ChatMode::Terminal => "Execute",
        })
        .size(14)
    )
    .padding([12, 24]);

    let input_row = row![
        input_field,
        send_button,
    ]
    .spacing(12)
    .padding(16)
    .align_y(Alignment::Center);

    let content = column![
        header,
        container(messages_scroll)
            .height(Length::Fill),
        input_row,
    ]
    .spacing(0);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn message_bubble<Message: 'static + Clone>(sender: String, content: String, is_user: bool) -> Element<'static, Message> {
    let sender_text = text(sender)
        .size(12);

    let content_text = text(content)
        .size(14);

    let bubble = column![
        sender_text,
        Space::with_height(4),
        content_text,
    ]
    .padding(12)
    .spacing(0);

    let bubble_container = container(bubble);

    if is_user {
        row![
            Space::with_width(Length::FillPortion(1)),
            container(bubble_container)
                .width(Length::FillPortion(3)),
        ]
        .into()
    } else {
        row![
            container(bubble_container)
                .width(Length::FillPortion(3)),
            Space::with_width(Length::FillPortion(1)),
        ]
        .into()
    }
}
