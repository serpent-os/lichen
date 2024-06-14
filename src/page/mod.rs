// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use lichen::tui::Element;

pub use self::timezone::Timezone;
pub use self::user::User;
pub use self::welcome::Welcome;

pub mod timezone;
pub mod user;
pub mod welcome;

/// Message mapping from lower page implementations
pub enum Message {
    Welcome(welcome::Message),
    User(user::Message),
    Timezone(timezone::Message),
}

/// Encapsulation of all potential pages
pub enum Page {
    Welcome(Welcome),
    User(User),
    Timezone(Timezone),
}

/// Encapsulation of all potential evnts
pub enum Event {
    Welcome(welcome::Event),
    User(user::Event),
    Timezone(timezone::Event),
}

impl Page {
    /// Welcome page
    pub fn welcome() -> Self {
        Self::Welcome(Welcome::default())
    }

    /// User page
    pub fn user() -> Self {
        Self::User(User::default())
    }

    /// Timzone page
    pub fn timezone() -> Self {
        Self::Timezone(Timezone::default())
    }

    /// Dispatch update
    pub fn update(&mut self, message: Message) -> Option<Event> {
        match (self, message) {
            (Self::Welcome(state), Message::Welcome(message)) => {
                Some(Event::Welcome(state.update(message)))
            }
            (Self::User(state), Message::User(message)) => Some(Event::User(state.update(message))),
            (Self::Timezone(state), Message::Timezone(message)) => {
                Some(Event::Timezone(state.update(message)))
            }
            _ => None,
        }
    }

    /// View the correct page
    pub fn view(&self) -> Element<Message> {
        match self {
            Self::Welcome(state) => state.view().map(Message::Welcome),
            Self::User(state) => state.view().map(Message::User),
            Self::Timezone(state) => state.view().map(Message::Timezone),
        }
    }
}
