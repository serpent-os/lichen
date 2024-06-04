// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Component APIs

use bitflags::bitflags;
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::{
    layout::{Constraint, Direction, Rect},
    Frame,
};

#[derive(Debug, Clone, Copy)]
pub enum Action {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Quit,
    Redraw,
    Noop,
}

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct State : u8 {
        const NONE = 1;
        const HIGHLIGHT = 1 << 1;
        const ACTIVE = 1 << 2;
    }
}

pub trait Component {
    fn render(&self, frame: &mut Frame, area: Rect);
    fn update(&self, action: Action) -> Option<Action>;

    // State management funcs
    fn state(&self) -> State;
    fn push_state(&self, st: State);
    fn pop_state(&self, st: State);

    fn constraints(&self, direction: Direction) -> Constraint {
        match direction {
            Direction::Horizontal => Constraint::Max(10),
            Direction::Vertical => Constraint::Max(3),
        }
    }
}
