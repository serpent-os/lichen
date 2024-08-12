// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use cosmic::{
    iced::{
        alignment::{Horizontal, Vertical},
        Length,
    },
    theme,
    widget::{self},
    Element,
};

use crate::Message;

#[derive(Default)]
pub struct Page {}

impl Page {
    pub fn view(&self) -> Element<Message> {
        widget::column::with_children(vec![
            widget::row::with_children(vec![
                widget::icon::from_name("system-software-install").size(96).into(),
                widget::column::with_children(vec![
                    widget::text::title1("Welcome to the future").into(),
                    widget::text::title4("We're about to install Serpent OS onto your device").into(),
                ])
                .spacing(8)
                .padding(8)
                .into(),
            ])
            .into(),
            widget::container(
                widget::text::body("TODO: Confirm setup steps\n✅ Thing 1\n✅ Thing 2\n✅ Thing 3")
                    .horizontal_alignment(Horizontal::Center)
                    .vertical_alignment(Vertical::Center),
            )
            .style(theme::Container::Dialog)
            .padding(8)
            .into(),
        ])
        .height(Length::Fill)
        .spacing(16)
        .padding(12)
        .into()
    }
}
