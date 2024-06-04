// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! User management page
//! NOTE: TOTAL hack right now!

use std::{cell::RefCell, rc::Rc};

use ratatui::layout::{Direction, Flex, Rect};

use crate::{
    boxlayout::BoxLayout, button::Button, component::State, textbox::TextBox, theme, Component,
};

pub struct Users {
    vbox: RefCell<BoxLayout>,
}

impl Component for Users {
    fn render(&self, frame: &mut ratatui::prelude::Frame, area: Rect) {
        self.vbox.borrow().render(frame, area);
    }

    fn update(&self, action: crate::Action) -> Option<crate::Action> {
        self.vbox.borrow().update(action)
    }

    fn state(&self) -> State {
        State::NONE
    }

    fn push_state(&self, st: crate::component::State) {}

    fn pop_state(&self, st: crate::component::State) {}
}

impl Default for Users {
    fn default() -> Self {
        Self::new()
    }
}

impl Users {
    pub fn new() -> Self {
        let name = TextBox::new(format!("{}Username ", theme::current().icons.user));
        let password = TextBox::new(format!("{}Password ", theme::current().icons.password));
        password.set_hide_chars();

        let confirm_password = TextBox::new(format!(
            "{}Confirm password ",
            theme::current().icons.password
        ));
        confirm_password.set_hide_chars();
        let hbox = BoxLayout::new(vec![
            Rc::new(Button::new("Cancel")),
            Rc::new(Button::new("Ok")),
        ])
        .flex(Flex::End);
        let vbox = BoxLayout::new(vec![
            Rc::new(name),
            Rc::new(password),
            Rc::new(confirm_password),
            Rc::new(hbox),
        ])
        .direction(Direction::Vertical)
        .flex(Flex::Start);
        Self {
            vbox: RefCell::new(vbox),
        }
    }
}
