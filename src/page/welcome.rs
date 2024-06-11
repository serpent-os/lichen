// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Welcome page

use lichen::tui::{
    widget::{button, hbox, paragraph, vbox},
    Element,
};
use ratatui::{
    layout::{Alignment, Flex},
    style::{Style, Stylize},
    text::Line,
};

use crate::theme;

pub enum Event {
    Ok,
}

#[derive(Clone)]
pub enum Message {
    Ok,
}

#[derive(Default)]
pub struct Welcome {
    ok: button::State,
}

impl Welcome {
    pub fn update(&mut self, message: Message) -> Event {
        match message {
            Message::Ok => Event::Ok,
        }
    }

    pub fn view(&self) -> Element<Message> {
        let intro = paragraph(vec![
            Line::from("Are you ready to install Serpent OS?").style(Style::default().bold()).alignment(Alignment::Center),
            Line::from(""), 
            Line::from(""),
            Line::from("📋 Disclaimer").style(Style::default().bold()),
            Line::from(""),
            Line::from("Serpent OS is not ready for production use nor should it be used for market predictions, nuclear facility management or the care of infants. At any point your installation may encounter errors or in fact no longer have ever been installed due to temporal fluctations resulting from the misuse of this install in a particle accelerator.").style(Style::default().dim())
        ])
        .wrap()
        .into();

        let ok = button(&self.ok, "Begin")
            .on_press(Message::Ok)
            .style(theme::button)
            .into();

        let buttons = hbox(vec![ok]).flex(Flex::End).into();

        vbox(vec![intro, buttons]).spacing(2).into()
    }
}
