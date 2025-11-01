use iced::{Application, Command, Element, Theme};
use iced::widget::scrollable;

use crate::auth::AuthState;
use crate::core::CoreBridge;
use crate::workspaces::{Workspace, WorkspaceView};

pub struct NeuroGraphApp {
    auth_state: AuthState,
    current_workspace: Workspace,
    core: CoreBridge,  // Прямой доступ к Rust core!

    // Chat state
    chat_input: String,
    chat_history: Vec<ChatMessage>,
    chat_scroll: scrollable::Id,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub role: MessageRole,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageRole {
    User,
    System,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Auth
    PasswordInput(String),
    LoginAttempt(bool), // true = root, false = user
    Lock,

    // Navigation
    SwitchWorkspace(Workspace),

    // Chat
    ChatInput(String),
    SendMessage,
}

impl Application for NeuroGraphApp {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                auth_state: AuthState::new(),
                current_workspace: Workspace::Welcome,
                core: CoreBridge::new(),  // Инициализация Rust core
                chat_input: String::new(),
                chat_history: Vec::new(),
                chat_scroll: scrollable::Id::unique(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("NeuroGraph OS")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::PasswordInput(password) => {
                self.auth_state.update_password(password);
            }
            Message::LoginAttempt(is_root) => {
                if self.auth_state.try_login(is_root) {
                    self.current_workspace = Workspace::Welcome;
                }
            }
            Message::SwitchWorkspace(workspace) => {
                self.current_workspace = workspace;
            }
            Message::Lock => {
                self.auth_state.lock();
            }
            Message::ChatInput(input) => {
                self.chat_input = input;
            }
            Message::SendMessage => {
                if !self.chat_input.trim().is_empty() {
                    // Добавляем сообщение пользователя
                    self.chat_history.push(ChatMessage {
                        role: MessageRole::User,
                        content: self.chat_input.clone(),
                    });

                    // Обрабатываем через CoreBridge
                    let response = self.core.process_message(&self.chat_input);

                    // Добавляем ответ системы
                    self.chat_history.push(ChatMessage {
                        role: MessageRole::System,
                        content: response,
                    });

                    // Очищаем input
                    self.chat_input.clear();

                    // Прокручиваем вниз
                    return scrollable::snap_to(
                        self.chat_scroll.clone(),
                        scrollable::RelativeOffset::END,
                    );
                }
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Message> {
        if !self.auth_state.is_authenticated() {
            self.auth_state.view()
        } else {
            self.current_workspace.view(
                &self.auth_state,
                &self.core,
                &self.chat_input,
                &self.chat_history,
                &self.chat_scroll,
            )
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
