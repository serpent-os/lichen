// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use cosmic::{
    iced::Length,
    widget::{self, combo_box},
    Element,
};

use crate::Message;

pub struct Page {
    languages: combo_box::State<String>,
}

impl Default for Page {
    fn default() -> Self {
        Self {
            languages: combo_box::State::new(vec!["one".into(), "two".into()]),
        }
    }
}

impl Page {
    pub fn view(&self) -> Element<Message> {
        widget::column::with_children(vec![
            widget::row::with_children(vec![
                widget::icon::from_name("preferences-desktop-locale").size(96).into(),
                widget::column::with_children(vec![
                    widget::text::title1("Region & language").into(),
                    widget::text::title4("Make your laptop talk the right lingo").into(),
                ])
                .spacing(8)
                .padding(8)
                .into(),
            ])
            .into(),
            widget::combo_box(&self.languages, "Type to select your language", None, |_| {
                crate::Message::LanguagePicked
            })
            .padding(8)
            .into(),
        ])
        .height(Length::Fill)
        .spacing(16)
        .padding(12)
        .into()
    }
}
