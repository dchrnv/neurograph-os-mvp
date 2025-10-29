// Auth module - Phase 1: Simple mock implementation

use iced::{widget::{column, text, text_input, button, container}, Element, Length};
use crate::app::Message;

pub struct AuthState {
    password_input: String,
    is_authenticated: bool,
    is_admin: bool,
    error: Option<String>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            password_input: String::new(),
            is_authenticated: false,
            is_admin: false,
            error: None,
        }
    }

    pub fn update_password(&mut self, password: String) {
        self.password_input = password;
        self.error = None;
    }

    pub fn try_login(&mut self, is_root: bool) -> bool {
        // Phase 1: Mock authentication (demo only!)
        // TODO Phase 2: Real Argon2id validation
        if self.password_input == "demo" || (is_root && self.password_input == "root") {
            self.is_authenticated = true;
            self.is_admin = is_root;
            self.password_input.clear();
            true
        } else {
            self.error = Some("Invalid password".to_string());
            false
        }
    }

    pub fn lock(&mut self) {
        self.is_authenticated = false;
        self.password_input.clear();
    }

    pub fn is_authenticated(&self) -> bool {
        self.is_authenticated
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    pub fn view(&self) -> Element<Message> {
        let logo = text(include_str!("../assets/logo.txt"))
            .font(iced::Font::MONOSPACE)
            .size(12);

        let title = text("NEUROGRAPH OS")
            .size(24);

        let warning = text("⚠️  EXPERIMENTAL COGNITIVE SYSTEM")
            .size(14);

        let password = text_input("Password", &self.password_input)
            .on_input(Message::PasswordInput)
            .on_submit(Message::LoginAttempt(false))
            .secure(true)
            .padding(10);

        let error_msg = if let Some(err) = &self.error {
            text(err).size(14)
        } else {
            text("")
        };

        let buttons = iced::widget::row![
            button("User Login")
                .on_press(Message::LoginAttempt(false))
                .padding(10),
            button("Root Login")
                .on_press(Message::LoginAttempt(true))
                .padding(10),
        ]
        .spacing(10);

        let content = column![logo, title, warning, password, error_msg, buttons]
            .spacing(20)
            .padding(40)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
