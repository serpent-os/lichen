// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Component APIs

use bitflags::bitflags;
use ratatui::{
    layout::{Constraint, Direction, Rect},
    Frame,
};

use crate::{Event, Shell};

bitflags! {
    #[derive(Debug, Clone, Copy)]
    pub struct State : u8 {
        const NONE = 1;
        const HIGHLIGHT = 1 << 1;
        const ACTIVE = 1 << 2;
    }
}

pub trait Component {
    type Message;
    /// Draw using the final ratatui Frame within the bounds of area
    ///
    /// # Arguments:
    ///
    /// - `frame` - Ratatui frame target
    /// - `area` - Bounds of our drawing
    fn render(&self, frame: &mut Frame, area: Rect);

    /// Pass an update down the chain of the component
    fn update(&self, event: Event, shell: &mut Shell<Self::Message>);

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
