// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! User management page
//! NOTE: TOTAL hack right now!

use ratatui::layout::Rect;

use crate::{
    boxlayout::BoxLayout,
    component::{Orientation, State},
    textbox::TextBox,
    Component,
};

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

    fn state(&self) -> State {
        State::NONE
    }

    fn push_state(&mut self, st: crate::component::State) {}

    fn pop_state(&mut self, st: crate::component::State) {}
}

impl Default for Users {
    fn default() -> Self {
        Self::new()
    }
}

impl Users {
    pub fn new() -> Self {
        let name = TextBox::new(" 👤 Username ");
        let mut password = TextBox::new(" 🔑 Password ");
        password.set_hide_chars();

        let mut confirm_password = TextBox::new(" 🔑 Confirm password ");
        confirm_password.set_hide_chars();
        let vbox = BoxLayout::new(vec![
            Box::new(name),
            Box::new(password),
            Box::new(confirm_password),
        ])
        .orientation(Orientation::Vertical);
        Self { vbox }
    }
}
