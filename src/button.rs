// SPDX-FileCopyrightText: Copyright © 2024 Serpent OS Developers
//
// SPDX-License-Identifier: MPL-2.0

//! Trivial button widget

use ratatui::{
    layout::{Constraint, Direction},
    text::Line,
    widgets::{Block, BorderType, Borders, Padding, Paragraph},
};

use crate::{Component, State};

/// The Button type
pub struct Button {
    /// Label for the button
    contents: String,
}

impl Component for Button {
    /// Draw the button as a block'd label
    fn render(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect) {
        let line = Line::from(self.contents.as_str());
        let draw = Paragraph::new(line)
            .block(
                Block::default()
                    .padding(Padding::symmetric(1, 0))
                    .border_type(BorderType::Rounded)
                    .borders(Borders::ALL),
            )
            .centered();

        frame.render_widget(draw, area)
    }

    /// TODO: Handle this
    fn update(&self, _: crate::Action) -> Option<crate::Action> {
        None
    }

    /// TODO: Handle this
    fn state(&self) -> State {
        State::NONE
    }

    /// TODO: Handle this
    fn push_state(&self, _: State) {}

    /// TODO: Handle this
    fn pop_state(&self, _: State) {}

    /// Return padded constraints
    fn constraints(&self, direction: Direction) -> Constraint {
        match direction {
            Direction::Horizontal => Constraint::Max((self.contents.chars().count() + 6) as u16),
            Direction::Vertical => Constraint::Max(3),
        }
    }
}

impl Button {
    /// Create a new button with the given text
    pub fn new(contents: impl AsRef<str>) -> Self {
        Self {
            contents: contents.as_ref().to_string(),
        }
    }
}
