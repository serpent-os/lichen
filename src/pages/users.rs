// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! User management page
//! NOTE: TOTAL hack right now!

use ratatui::layout::{Flex, Rect};

use crate::{
    boxlayout::BoxLayout,
    button::Button,
    component::{Orientation, State},
    textbox::TextBox,
    theme, Component,
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
        let name = TextBox::new(format!("{}Username ", theme::current().icons.user));
        let mut password = TextBox::new(format!("{}Password ", theme::current().icons.password));
        password.set_hide_chars();

        let mut confirm_password = TextBox::new(format!(
            "{}Confirm password ",
            theme::current().icons.password
        ));
        confirm_password.set_hide_chars();
        let hbox = BoxLayout::new(vec![
            Box::new(Button::new("Cancel")),
            Box::new(Button::new("Ok")),
        ])
        .flex(Flex::End);
        let vbox = BoxLayout::new(vec![
            Box::new(name),
            Box::new(password),
            Box::new(confirm_password),
            Box::new(hbox),
        ])
        .orientation(Orientation::Vertical)
        .flex(Flex::Start);
        Self { vbox }
    }
}
