// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! User management page
//! NOTE: TOTAL hack right now!

use lichen::tui::{
    widget::{
        button, hbox,
        text_box::{self, text_box},
        vbox,
    },
    Element,
};
use ratatui::layout::Flex;

use crate::theme;

pub enum Event {
    User { username: String, password: String },
    Cancel,
}

#[derive(Clone)]
pub enum Message {
    Submit,
    Cancel,
}

/// the Users type
#[derive(Default)]
pub struct User {
    username: text_box::State,
    password: text_box::State,
    password_confirmation: text_box::State,
    cancel: button::State,
    ok: button::State,
}

impl User {
    pub fn update(&mut self, message: Message) -> Event {
        match message {
            Message::Submit => Event::User {
                username: self.username.lines().into_iter().next().unwrap_or_default(),
                password: self.password.lines().into_iter().next().unwrap_or_default(),
            },
            Message::Cancel => {
                self.username.reset();
                self.password.reset();
                self.password_confirmation.reset();

                Event::Cancel
            }
        }
    }

    /// Return a new Users page
    pub fn view(&self) -> Element<Message> {
        let username = text_box(&self.username)
            .title(format!("{}Username ", theme::current().icons.user))
            .style(theme::text_box);
        let password = text_box(&self.password)
            .hide_chars()
            .title(format!("{}Password ", theme::current().icons.password))
            .style(theme::text_box);

        let confirm_password = text_box(&self.password_confirmation)
            .hide_chars()
            .title(format!("{}Confirm password ", theme::current().icons.password))
            .style(theme::text_box);

        let buttons = hbox(vec![
            button(&self.cancel, "Cancel")
                .on_press(Message::Cancel)
                .style(theme::button)
                .into(),
            button(&self.ok, "Ok")
                .on_press(Message::Submit)
                .style(theme::button)
                .into(),
        ])
        .flex(Flex::End);

        vbox(vec![
            username.into(),
            password.into(),
            confirm_password.into(),
            buttons.into(),
        ])
        .spacing(1)
        .flex(Flex::Start)
        .into()
    }
}
