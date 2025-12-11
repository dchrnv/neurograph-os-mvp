// Authentication screen with PIN input
// Based on DESKTOP_UI_SPEC_V3.md section 4

use iced::widget::{column, container, text, text_input, button, Column};
use iced::{Element, Alignment, Length};
use crate::theme::Theme;

pub fn view<Message: 'static + Clone>(_theme: &Theme) -> Element<'static, Message> {
    let logo = text("NEUROGRAPH")
        .size(32);

    let subtitle = text("Enter PIN")
        .size(14);

    let pin_input = text_input("Enter PIN", "")
        .size(16)
        .padding(12)
        .width(Length::Fixed(200.0));

    let unlock_button = button(
        text("Unlock")
            .size(14)
    )
    .padding([10, 20])
    .width(Length::Fixed(200.0));

    let version_text = text("v0.46.0")
        .size(11);

    let content: Column<Message> = column![
        logo,
        subtitle,
        pin_input,
        unlock_button,
    ]
    .spacing(16)
    .align_x(Alignment::Center)
    .width(Length::Shrink);

    let card = container(content)
        .padding(48);

    let main_container = container(
        column![card, version_text]
            .spacing(24)
            .align_x(Alignment::Center)
    )
    .width(Length::Fill)
    .height(Length::Fill)
    .center_x(Length::Fill)
    .center_y(Length::Fill);

    main_container.into()
}
