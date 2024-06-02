// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! User management page
//! NOTE: TOTAL hack right now!

use ratatui::layout::Rect;

use crate::{boxlayout::BoxLayout, textbox::TextBox, Component};

pub struct Users {
    vbox: BoxLayout,
}

impl Component for Users {
    fn render(&self, frame: &mut ratatui::prelude::Frame, area: Rect) {
        self.vbox.render(frame, area);
    }

    fn update(&mut self, action: crate::Action) -> Option<crate::Action> {
        self.vbox.update(action)
    }
}

impl Default for Users {
    fn default() -> Self {
        Self::new()
    }
}

impl Users {
    pub fn new() -> Self {
        let name = TextBox::new("Username");
        let mut password = TextBox::new("Password");
        password.set_hide_chars();
        let vbox = BoxLayout::new(vec![Box::new(name), Box::new(password)])
            .orientation(crate::boxlayout::Orientation::Vertical);
        Self { vbox }
    }
}
