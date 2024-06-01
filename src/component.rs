// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Component APIs

use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{layout::Rect, Frame};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Quit,
    Redraw,
    Noop,
}

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect);
    fn update(&mut self, action: Action) -> Option<Action>;
}
