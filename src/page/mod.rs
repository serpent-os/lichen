// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use lichen::tui::Element;

pub use self::user::User;
pub use self::welcome::Welcome;

pub mod user;
pub mod welcome;

pub enum Message {
    Welcome(welcome::Message),
    User(user::Message),
}

pub enum Page {
    Welcome(Welcome),
    User(User),
}

pub enum Event {
    Welcome(welcome::Event),
    User(user::Event),
}

impl Page {
    pub fn welcome() -> Self {
        Self::Welcome(Welcome::default())
    }

    pub fn user() -> Self {
        Self::User(User::default())
    }

    pub fn update(&mut self, message: Message) -> Option<Event> {
        match (self, message) {
            (Self::Welcome(state), Message::Welcome(message)) => {
                Some(Event::Welcome(state.update(message)))
            }
            (Self::User(state), Message::User(message)) => Some(Event::User(state.update(message))),
            _ => None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        match self {
            Self::Welcome(state) => state.view().map(Message::Welcome),
            Self::User(state) => state.view().map(Message::User),
        }
    }
}
