// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Widget APIs

use bitflags::bitflags;
use ratatui::{
    layout::{Constraint, Direction, Rect},
    Frame,
};

use crate::{event, Event, Shell};

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct State : u8 {
        const NONE = 1;
        const HIGHLIGHT = 1 << 1;
        const ACTIVE = 1 << 2;
    }
}

pub trait Widget<Message> {
    /// Pass an update down the chain of the widget
    fn update(&mut self, event: Event, shell: &mut Shell<Message>) -> event::Status;

    /// Draw using the final ratatui Frame within the bounds of area
    ///
    /// # Arguments:
    ///
    /// - `frame` - Ratatui frame target
    /// - `area` - Bounds of our drawing
    fn render(&self, frame: &mut Frame, area: Rect);

    // State management funcs
    fn state(&self) -> State;
    fn push_state(&self, st: State);
    fn pop_state(&self, st: State);

    /// Return the optimal constraints of the widget
    fn constraints(&self, direction: Direction) -> Constraint {
        match direction {
            Direction::Horizontal => Constraint::Max(10),
            Direction::Vertical => Constraint::Max(3),
        }
    }
}
