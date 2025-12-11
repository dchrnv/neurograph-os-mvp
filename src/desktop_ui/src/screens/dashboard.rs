// Dashboard screen - System overview
// Based on DESKTOP_UI_SPEC_V3.md section 5

use iced::widget::{column, container, row, text, Space};
use iced::{Element, Length};
use crate::theme::Theme;

pub fn view<Message: 'static + Clone>(_theme: &Theme) -> Element<'static, Message> {
    let header = text("Dashboard")
        .size(24);

    // Metrics row - 4 cards
    let metrics = row![
        metric_card("CPU".to_string(), "12%".to_string()),
        metric_card("Memory".to_string(), "2.3 GB".to_string()),
        metric_card("Tokens".to_string(), "1.2M".to_string()),
        metric_card("Connections".to_string(), "847".to_string()),
    ]
    .spacing(16);

    // Active modules section
    let modules_header = text("Active Modules")
        .size(18);

    let modules_list = column![
        module_item("Gateway".to_string(), "Running".to_string()),
        module_item("Intuition Engine".to_string(), "Running".to_string()),
        module_item("Guardian".to_string(), "Running".to_string()),
        module_item("Experience Stream".to_string(), "Running".to_string()),
    ]
    .spacing(8);

    // Recent activity section
    let activity_header = text("Recent Activity")
        .size(18);

    let activity_list = column![
        activity_item("Signal processed".to_string(), "2 seconds ago".to_string()),
        activity_item("Token created".to_string(), "5 seconds ago".to_string()),
        activity_item("Connection established".to_string(), "12 seconds ago".to_string()),
        activity_item("Pattern detected".to_string(), "23 seconds ago".to_string()),
    ]
    .spacing(8);

    let content = column![
        header,
        Space::with_height(24),
        metrics,
        Space::with_height(32),
        modules_header,
        Space::with_height(12),
        modules_list,
        Space::with_height(32),
        activity_header,
        Space::with_height(12),
        activity_list,
    ]
    .padding(24)
    .spacing(0);

    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

fn metric_card<Message: 'static + Clone>(label: String, value: String) -> Element<'static, Message> {
    let label_text = text(label)
        .size(12);

    let value_text = text(value)
        .size(24);

    let card_content = column![
        label_text,
        Space::with_height(8),
        value_text,
    ]
    .padding(16);

    container(card_content)
        .width(Length::FillPortion(1))
        .into()
}

fn module_item<Message: 'static + Clone>(name: String, status: String) -> Element<'static, Message> {
    let name_text = text(name)
        .size(14);

    let status_text = text(status)
        .size(12);

    row![
        name_text,
        Space::with_width(Length::Fill),
        status_text,
    ]
    .padding(8)
    .into()
}

fn activity_item<Message: 'static + Clone>(event: String, time: String) -> Element<'static, Message> {
    let event_text = text(event)
        .size(14);

    let time_text = text(time)
        .size(11);

    row![
        event_text,
        Space::with_width(Length::Fill),
        time_text,
    ]
    .padding(8)
    .into()
}
