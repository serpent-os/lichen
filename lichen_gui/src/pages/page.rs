// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

use cosmic::{widget, Element};

use super::{language, welcome};

pub enum Page {
    Welcome(welcome::Page),
    Language(language::Page),
    None,
}

impl Page {
    // Create new welcome page
    pub fn welcome() -> Self {
        Self::Welcome(welcome::Page::default())
    }

    pub fn language() -> Self {
        Self::Language(language::Page::default())
    }

    pub fn none() -> Self {
        Self::None
    }

    pub fn view(&self) -> Element<crate::Message> {
        match self {
            Page::Welcome(p) => p.view(),
            Page::Language(p) => p.view(),
            Page::None => widget::text("not yet implemented").into(),
        }
    }
}
