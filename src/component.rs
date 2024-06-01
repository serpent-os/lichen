// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Component APIs

use crossterm::event::KeyEvent;
use ratatui::widgets::Widget;


pub enum Action {
    Key(KeyEvent),
    Quit,
    Redraw,
    Noop,
}

pub trait Component {
    fn widget(&self) -> impl Widget;
    fn update(&self, action: Action) -> Option<Action>;
}
