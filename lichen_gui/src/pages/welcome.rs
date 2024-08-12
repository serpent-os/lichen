// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use cosmic::{
    iced::alignment::{Horizontal, Vertical},
    theme,
    widget::{self},
    Element,
};

use crate::Message;

use super::{IconVariant, InstallerPage, Plugin};

#[derive(Default)]
pub struct Page {}

impl InstallerPage for Page {
    fn name(&self) -> &str {
        "Welcome"
    }

    fn view(&self) -> Element<Message> {
        widget::container(
            widget::text::body("TODO: Confirm setup steps\n✅ Thing 1\n✅ Thing 2\n✅ Thing 3")
                .horizontal_alignment(Horizontal::Center)
                .vertical_alignment(Vertical::Center),
        )
        .style(theme::Container::Dialog)
        .padding(8)
        .into()
    }

    fn title(&self) -> &str {
        "Welcome to the future"
    }

    fn subtitle(&self) -> &str {
        "We're about to install Serpent OS on your device"
    }

    fn icon(&self, variant: IconVariant) -> widget::icon::Named {
        match variant {
            IconVariant::Normal => widget::icon::from_name("system-software-install"),
            IconVariant::Symbolic => widget::icon::from_name("go-home-symbolic"),
        }
    }
}

inventory::submit! { Plugin {
    name: "welcome",
    page: || Box::new(Page::default())
}}
