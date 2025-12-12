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
    chat_mode: ChatMode,  // V3: Chat/Terminal tabs
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ChatMode {
    Chat,
    Terminal,
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
    SwitchChatMode(ChatMode),  // V3: Toggle between Chat/Terminal tabs
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
                chat_mode: ChatMode::Chat,  // V3: Default to Chat mode
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
            Message::SwitchChatMode(mode) => {
                self.chat_mode = mode;
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
                self.chat_mode,  // V3: Pass chat mode
            )
        }
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}
