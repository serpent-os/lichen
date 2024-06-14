// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! User management page
//! NOTE: TOTAL hack right now!

use lichen::tui::{
    widget::{button, hbox, vbox},
    Element,
};
use ratatui::layout::Flex;

use crate::theme;

/// Events we can produce
#[derive(Clone)]
pub enum Event {
    // Submitted timezone selection
    Timezone(chrono_tz::Tz),
    Cancel,
}

/// Messages we can produce
#[derive(Clone)]
pub enum Message {
    Submit,
    Cancel,
}

#[derive(Default)]
pub struct Timezone {
    ok: button::State,
    cancel: button::State,
}

impl Timezone {
    pub fn update(&mut self, message: Message) -> Event {
        match message {
            Message::Submit => Event::Timezone(chrono_tz::Europe::London),
            Message::Cancel => Event::Cancel,
        }
    }

    pub fn view(&self) -> Element<Message> {
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

        vbox(vec![buttons.into()]).spacing(1).flex(Flex::End).into()
    }
}
