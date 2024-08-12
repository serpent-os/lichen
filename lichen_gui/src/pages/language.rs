// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use cosmic::widget;
use cosmic::{widget::combo_box, Element};

use crate::pages::Plugin;
use crate::Message;

use super::{IconVariant, InstallerPage};

#[derive(Debug, Clone)]
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

impl InstallerPage for Page {
    fn name(&self) -> &str {
        "Language"
    }

    fn view(&self) -> Element<Message> {
        widget::combo_box(&self.languages, "Type to select your language", None, |_| {
            crate::Message::LanguagePicked
        })
        .padding(8)
        .into()
    }

    fn title(&self) -> &str {
        "Region & Language"
    }

    fn subtitle(&self) -> &str {
        "Configure your system locale"
    }

    fn icon(&self, variant: IconVariant) -> widget::icon::Named {
        match variant {
            IconVariant::Normal => widget::icon::from_name("preferences-desktop-locale"),
            IconVariant::Symbolic => widget::icon::from_name("preferences-desktop-locale-symbolic"),
        }
    }
}

inventory::submit! { Plugin { name: "language", page: || Box::new(Page::default()) }}
